use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use sangoku::{
    bundle,
    emu::Emulator,
    error::{Error, Result},
};

mod console;
mod diagnostics;
mod link;

const TOOLCHAIN: &str = "gbc-lr35902";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{:?}", miette::Report::new(error));
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let mut args = env::args_os().skip(1);
    let Some(command) = args.next() else {
        return Err(usage());
    };

    let root = bundle::root()?;
    let (paths, opt_level, alloc) = parse_build_args(args)?;
    match command.to_str() {
        Some("example") => {
            let output = paths
                .first()
                .cloned()
                .unwrap_or_else(|| PathBuf::from("build/hello.gb"));
            if paths.len() > 1 {
                return Err(usage());
            }
            build_rom(
                &root,
                &root.join("examples/hello.rs"),
                &output,
                &opt_level,
                alloc,
            )
        }
        Some("build") => {
            let Some(source) = paths.first() else {
                return Err(usage());
            };
            let output = paths.get(1).cloned().unwrap_or_else(|| {
                let name = source.file_stem().unwrap_or_else(|| OsStr::new("game"));
                PathBuf::from("build").join(name).with_extension("gb")
            });
            if paths.len() > 2 {
                return Err(usage());
            }
            build_rom(&root, source, &output, &opt_level, alloc)
        }
        Some("run") => {
            let Some(source) = paths.first() else {
                return Err(usage());
            };
            if paths.len() > 1 {
                return Err(usage());
            }
            let name = source.file_stem().unwrap_or_else(|| OsStr::new("game"));
            let output = PathBuf::from("build").join(name).with_extension("gb");
            build_rom(&root, source, &output, &opt_level, alloc)?;
            run_rom(&output)
        }
        Some("frontend") => {
            let Some(source) = paths.first() else {
                return Err(usage());
            };
            let output = paths.get(1).cloned().unwrap_or_else(|| {
                let name = source.file_stem().unwrap_or_else(|| OsStr::new("game"));
                PathBuf::from("build").join(name).with_extension("mir")
            });
            if paths.len() > 2 {
                return Err(usage());
            }
            emit_mir(&root, source, &output, &opt_level, alloc)
        }
        Some("clif" | "asm") => {
            let clif_only = command == "clif";
            let Some(source) = paths.first() else {
                return Err(usage());
            };
            let output = paths.get(1).cloned().unwrap_or_else(|| {
                let name = source.file_stem().unwrap_or_else(|| OsStr::new("game"));
                PathBuf::from("build")
                    .join(name)
                    .with_extension(if clif_only { "clif" } else { "asm" })
            });
            if paths.len() > 2 {
                return Err(usage());
            }
            emit_code(&root, source, &output, &opt_level, alloc, clif_only)
        }
        Some("lower") => {
            let Some(source) = paths.first() else {
                return Err(usage());
            };
            let output = paths.get(1).cloned().unwrap_or_else(|| {
                let name = source.file_stem().unwrap_or_else(|| OsStr::new("game"));
                PathBuf::from("build").join(name).with_extension("s")
            });
            if paths.len() > 2 {
                return Err(usage());
            }
            lower_clif(&root, source, &output)
        }
        _ => Err(usage()),
    }
}

const MAX_INSTRUCTIONS: u64 = 2_000_000_000;

fn run_rom(rom: &Path) -> Result<()> {
    let image = fs::read(rom)
        .map_err(|error| Error::io(format!("could not read {}", rom.display()), error))?;
    let mut emulator = Emulator::new(&image, 0x100)?;
    emulator.set_sp(0xE000);
    let mut stdout = io::stdout().lock();
    let mut printed = 0;
    for _ in 0..MAX_INSTRUCTIONS {
        let before = emulator.pc;
        emulator.step()?;
        if emulator.serial.len() > printed {
            stdout
                .write_all(&emulator.serial[printed..])
                .and_then(|()| stdout.flush())
                .map_err(|error| Error::io("could not write stdout".to_string(), error))?;
            printed = emulator.serial.len();
        }
        if emulator.pc == before {
            eprintln!("halted with gbc_main returning {}", emulator.a);
            return Ok(());
        }
    }
    Err(format!("program did not halt within {MAX_INSTRUCTIONS} instructions").into())
}

fn usage() -> Error {
    "usage: sangoku example [output.gb] [--opt-level 0|1|2|3|s|z] [--alloc] | sangoku build <source.rs> [output.gb] [--opt-level 0|1|2|3|s|z] [--alloc] | sangoku run <source.rs> [--opt-level 0|1|2|3|s|z] [--alloc] | sangoku frontend <source.rs> [output.mir|-] [--opt-level 0|1|2|3|s|z] [--alloc] | sangoku clif <source.rs> [output.clif|-] [--opt-level 0|1|2|3|s|z] [--alloc] | sangoku asm <source.rs> [output.asm|-] [--opt-level 0|1|2|3|s|z] [--alloc] | sangoku lower <input.clif> [output.s|-]".into()
}

fn parse_build_args(
    args: impl IntoIterator<Item = OsString>,
) -> Result<(Vec<PathBuf>, String, bool)> {
    let mut args = args.into_iter();
    let mut paths = Vec::new();
    let mut opt_level = "0".to_owned();
    let mut alloc = false;
    while let Some(arg) = args.next() {
        if arg == "--alloc" {
            alloc = true;
            continue;
        }
        let level = if arg == "--opt-level" {
            Some(
                args.next()
                    .ok_or("--opt-level requires 0, 1, 2, 3, s or z")?,
            )
        } else {
            arg.to_str()
                .and_then(|value| value.strip_prefix("--opt-level="))
                .map(OsString::from)
        };
        if let Some(level) = level {
            match level.to_str() {
                Some(value @ ("0" | "1" | "2" | "3" | "s" | "z")) => opt_level = value.into(),
                _ => return Err("--opt-level requires 0, 1, 2, 3, s or z".into()),
            }
        } else if arg
            .to_str()
            .is_some_and(|value| value.starts_with('-') && value != "-")
        {
            return Err(format!("unknown build option: {}", arg.to_string_lossy()).into());
        } else {
            paths.push(PathBuf::from(arg));
        }
    }
    Ok((paths, opt_level, alloc))
}

fn rustc_command(root: &Path, opt_level: &str) -> Command {
    let mut command = if bundle::is_bundled(root) {
        let mut command = bundle::command(root, &root.join("toolchain/bin/rustc"));
        command.arg("--sysroot").arg(root.join("toolchain"));
        command
    } else {
        let mut command = Command::new("rustc");
        command.arg(format!("+{TOOLCHAIN}"));
        command
    };
    command
        .current_dir(root)
        .arg("-Zunstable-options")
        .arg("--target")
        .arg(root.join("targets/lr35902.json"))
        .args([
            "--crate-type=lib",
            "-Cpanic=immediate-abort",
            "-Cdebug-assertions=off",
            "-Zinline-mir=yes",
            "-Zmir-opt-level=2",
            // mir debug locals otherwise keep dead panic formatting arguments live.
            "-Zmir-strip-debuginfo=all-locals",
        ])
        .arg(format!("--remap-path-prefix={}=.", root.display()))
        .arg(format!("-Copt-level={opt_level}"));
    if opt_level == "0" {
        command.arg("-Zinline-mir-hint-threshold=50");
    }
    command
}

fn build_backend(root: &Path) -> Result<PathBuf> {
    if bundle::is_bundled(root) {
        return Ok(root.join("backend/librustc_codegen_cranelift.so"));
    }
    run_command(
        Command::new("cargo")
            .current_dir(root)
            .arg(format!("+{TOOLCHAIN}"))
            .args(["build", "--release", "--locked", "--manifest-path"])
            .arg(root.join("third_party/rustc_codegen_cranelift/Cargo.toml")),
    )?;
    Ok(root
        .join("third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so"))
}

fn lower_clif(root: &Path, input: &Path, output: &Path) -> Result<()> {
    let input = absolute_path(input)?;
    if !input.is_file() {
        return Err(format!("CLIF input does not exist: {}", input.display()).into());
    }
    let mut command = if bundle::is_bundled(root) {
        bundle::command(root, &root.join("toolchain/bin/sangoku-lower"))
    } else {
        let mut command = Command::new("cargo");
        command.args(["run", "-q", "-p", "sangoku-lower", "--"]);
        command
    };
    command.current_dir(root).arg(&input).arg("-");
    let result = command
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| Error::io(format!("could not run {command:?}"), error))?;
    if !result.status.success() {
        return Err(Error::CommandFailed {
            command: format!("{command:?}"),
            status: result.status,
        });
    }
    write_output(output, result.stdout)
}

fn emit_code(
    root: &Path,
    source: &Path,
    output: &Path,
    opt_level: &str,
    alloc: bool,
    clif_only: bool,
) -> Result<()> {
    let source = absolute_path(source)?;
    if !source.is_file() {
        return Err(format!("Rust source does not exist: {}", source.display()).into());
    }

    let backend = build_backend(root)?;
    let artifacts = ArtifactDir::new(&source)?;
    let libraries = core_libraries(root, &backend, opt_level)?;
    let console = console::libraries(root, &backend, opt_level, &libraries)?;
    let mut command = rustc_command(root, opt_level);
    if clif_only {
        command.env("CG_CLIF_CLIF_ONLY", "1");
    }
    run_command(
        command
            .arg("--edition=2024")
            .arg(format!("-Zcodegen-backend={}", backend.display()))
            .args(libraries.externs(if alloc { 4 } else { 2 }))
            .args(console.externs())
            .arg(if clif_only {
                "--emit=asm,llvm-ir"
            } else {
                "--emit=asm"
            })
            .arg(&source)
            .arg("-o")
            .arg(artifacts.path().join("game.s")),
    )?;
    let emitted = if clif_only { "game.ll" } else { "game.s" };
    let contents = fs::read(artifacts.path().join(emitted))
        .map_err(|error| Error::io(format!("backend did not write {emitted}"), error))?;
    write_output(output, contents)
}

fn write_output(output: &Path, contents: Vec<u8>) -> Result<()> {
    if output == Path::new("-") {
        return io::stdout()
            .write_all(&contents)
            .map_err(|error| Error::io("could not write stdout", error));
    }
    let output = absolute_path(output)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| Error::io(format!("could not create {}", parent.display()), error))?;
    }
    fs::write(&output, contents)
        .map_err(|error| Error::io(format!("could not write {}", output.display()), error))?;
    println!("Wrote {}", output.display());
    Ok(())
}

fn emit_mir(root: &Path, source: &Path, output: &Path, opt_level: &str, alloc: bool) -> Result<()> {
    let source = absolute_path(source)?;
    if !source.is_file() {
        return Err(format!("Rust source does not exist: {}", source.display()).into());
    }

    let backend = build_backend(root)?;
    let libraries = core_libraries(root, &backend, opt_level)?;
    let console = console::libraries(root, &backend, opt_level, &libraries)?;
    let mut command = rustc_command(root, opt_level);
    command.arg("--edition=2024");
    command
        .args(libraries.externs(if alloc { 4 } else { 2 }))
        .args(console.externs());
    command.arg("-Zunpretty=mir").arg(&source);
    if output == Path::new("-") {
        return run_command(&mut command);
    }

    let result = command
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| Error::io(format!("could not run {command:?}"), error))?;
    if !result.status.success() {
        return Err(Error::CommandFailed {
            command: format!("{command:?}"),
            status: result.status,
        });
    }
    write_output(output, result.stdout)
}

fn build_rom(
    root: &Path,
    source: &Path,
    output: &Path,
    opt_level: &str,
    alloc: bool,
) -> Result<()> {
    let source = absolute_path(source)?;
    if !source.is_file() {
        return Err(format!("Rust source does not exist: {}", source.display()).into());
    }

    let output = absolute_path(output)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| Error::io(format!("could not create {}", parent.display()), error))?;
    }

    let backend = build_backend(root)?;
    let artifacts = ArtifactDir::new(&source)?;
    let rust_asm = artifacts.path().join("game.asm");
    let libraries = core_libraries(root, &backend, opt_level)?;
    let console = console::libraries(root, &backend, opt_level, &libraries)?;

    let mut command = rustc_command(root, opt_level);
    command.arg("--edition=2024");
    command
        .arg(format!("-Zcodegen-backend={}", backend.display()))
        .arg("--emit=asm")
        .args(libraries.externs(if alloc { 4 } else { 2 }))
        .args(console.externs())
        .arg(&source)
        .arg("-o")
        .arg(&rust_asm);
    run_command(&mut command)?;

    let program = read_text(&rust_asm)?;
    let core = read_text(&libraries.core_asm())?;
    let builtins = read_text(&libraries.builtins_asm())?;
    let mut texts = Vec::new();
    for path in console.assembly() {
        texts.push(read_text(&path)?);
    }
    if alloc {
        texts.push(read_text(&libraries.alloc_asm())?);
        texts.push(read_text(&libraries.heap_asm())?);
    }
    let sections: Vec<_> = link::split_sections(&program)
        .into_iter()
        .chain(link::split_sections(&core))
        .chain(link::builtin_sections(&builtins))
        .chain(texts.iter().flat_map(|text| link::split_sections(text)))
        .collect();
    let kept: Vec<_> = link::reachable(&sections, &[link::ENTRY])
        .into_iter()
        .map(|index| &sections[index])
        .collect();
    if kept.is_empty() {
        return Err(format!("{} is not defined by the program or core", link::ENTRY).into());
    }
    let linked: String = kept.iter().map(|section| section.text).collect();

    let header_object = artifacts.path().join("header.o");
    let mut header = Command::new("rgbasm");
    if linked.contains("___rust_alloc") {
        header.arg("-DHEAP=1");
    }
    run_command(
        header
            .arg("-o")
            .arg(&header_object)
            .arg(root.join("rom/header.asm")),
    )?;
    let mut fixed_objects = vec![header_object];
    for name in link::runtime_objects(&linked, opt_level) {
        let object = artifacts.path().join(format!("{name}.o"));
        let mut source = root.join(sangoku::runtime::object(name).path);
        if name == "panic"
            && let Some(symbol) = link::panic_symbol(&linked)
        {
            source = artifacts.path().join("panic.asm");
            fs::write(
                &source,
                read_text(&root.join("runtime/panic.asm"))?.replace("__gbc_panic", symbol),
            )
            .map_err(|error| Error::io(format!("could not write {}", source.display()), error))?;
        }
        assemble(root, &source, &object)?;
        fixed_objects.push(object);
    }

    let probe_source = artifacts.path().join("probe.asm");
    let probe_object = artifacts.path().join("probe.o");
    let probe_map = artifacts.path().join("probe.map");
    fs::write(&probe_source, link::bank_one(&linked))
        .map_err(|error| Error::io(format!("could not write {}", probe_source.display()), error))?;
    assemble(root, &probe_source, &probe_object)?;
    run_command(
        Command::new("rgblink")
            .arg("-m")
            .arg(&probe_map)
            .arg("-o")
            .arg(artifacts.path().join("probe.gb"))
            .args(&fixed_objects)
            .arg(&probe_object),
    )?;
    let (sizes, rom0_free) = link::parse_map(&read_text(&probe_map)?);
    let used: usize = sizes.values().sum();
    let retained: Vec<_> = kept.iter().map(|section| **section).collect();
    if let Some(dependency) = link::formatting_dependency(&retained, &sizes) {
        let warning = diagnostics::RetainedFormatting::new(
            dependency,
            used,
            &artifacts.path().display().to_string(),
        );
        eprintln!("{:?}", miette::Report::new(warning));
    }

    let game_source = artifacts.path().join("game_placed.asm");
    let game_object = artifacts.path().join("game.o");
    fs::write(&game_source, link::place(&kept, &sizes, rom0_free))
        .map_err(|error| Error::io(format!("could not write {}", game_source.display()), error))?;
    assemble(root, &game_source, &game_object)?;
    let capacity = 32768;
    if used > capacity {
        return Err(Error::RomCapacity {
            used,
            capacity,
            excess: used - capacity,
            artifacts: artifacts.path().display().to_string(),
            largest: link::largest_sections(&sizes, 5),
        });
    }
    run_command(
        Command::new("rgblink")
            .arg("-m")
            .arg(artifacts.path().join("game.map"))
            .arg("-n")
            .arg(artifacts.path().join("game.sym"))
            .arg("-t")
            .arg("-o")
            .arg(&output)
            .args(&fixed_objects)
            .arg(&game_object),
    )?;
    run_command(
        Command::new("rgbfix")
            .args(["-v", "-p", "0", "-t", "SANGOKU"])
            .arg(&output),
    )?;

    println!("Wrote {}", output.display());
    Ok(())
}

struct CoreLibraries(PathBuf);

impl CoreLibraries {
    fn core_asm(&self) -> PathBuf {
        self.0.join("core.s")
    }

    fn builtins_asm(&self) -> PathBuf {
        self.0.join("compiler_builtins.s")
    }

    fn alloc_asm(&self) -> PathBuf {
        self.0.join("alloc.s")
    }

    fn heap_asm(&self) -> PathBuf {
        self.0.join("gbc_heap.s")
    }

    fn externs(&self, count: usize) -> Vec<OsString> {
        let mut args = vec![
            "-L".into(),
            format!("dependency={}", self.0.display()).into(),
        ];
        let crates = [
            ("core", "libcore.rmeta"),
            ("compiler_builtins", "libcompiler_builtins.rmeta"),
            ("alloc", "liballoc.rmeta"),
            ("force:gbc_heap", "libgbc_heap.rmeta"),
        ];
        for (name, file) in crates.into_iter().take(count) {
            args.push("--extern".into());
            args.push(format!("{name}={}", self.0.join(file).display()).into());
        }
        args
    }
}

fn modified(path: &Path) -> Option<SystemTime> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
}

fn core_libraries(root: &Path, backend: &Path, opt_level: &str) -> Result<CoreLibraries> {
    if bundle::is_bundled(root) {
        let directory = root.join("libraries").join(format!("opt-{opt_level}"));
        for name in [
            "core.s",
            "compiler_builtins.s",
            "alloc.s",
            "gbc_heap.s",
            "libcore.rmeta",
            "libcompiler_builtins.rmeta",
            "liballoc.rmeta",
            "libgbc_heap.rmeta",
        ] {
            if !directory.join(name).is_file() {
                return Err(Error::Bundle(format!(
                    "missing prebuilt library {}",
                    directory.join(name).display()
                )));
            }
        }
        return Ok(CoreLibraries(directory));
    }
    let libraries = CoreLibraries(
        root.join("build/core/immediate-abort")
            .join(format!("opt-{opt_level}")),
    );
    let backend_time = modified(backend);
    let options = format!(
        "{:?}",
        rustc_command(root, opt_level)
            .get_args()
            .collect::<Vec<_>>()
    );
    let options_file = libraries.0.join("rustc-options");
    let options_match = fs::read_to_string(&options_file).is_ok_and(|saved| saved == options);
    let fresh = options_match
        && [
            libraries.core_asm(),
            libraries.builtins_asm(),
            libraries.alloc_asm(),
            libraries.heap_asm(),
        ]
        .iter()
        .all(|path| modified(path).is_some_and(|time| Some(time) >= backend_time));
    if fresh {
        return Ok(libraries);
    }
    fs::create_dir_all(&libraries.0)
        .map_err(|error| Error::io(format!("could not create {}", libraries.0.display()), error))?;
    fs::write(&options_file, "")
        .map_err(|error| Error::io(format!("could not write {}", options_file.display()), error))?;
    let library = root.join("third_party/rust/library");
    let compile = |name: &str, source: PathBuf, extra: Vec<OsString>| {
        run_command(
            rustc_command(root, opt_level)
                .args(["--edition=2024", "-Awarnings", "--emit=asm,metadata"])
                .arg(format!("-Zcodegen-backend={}", backend.display()))
                .args(["--crate-name", name])
                .args(extra)
                .arg(source)
                .arg("-o")
                .arg(libraries.0.join(format!("{name}.s"))),
        )
    };
    compile("core", library.join("core/src/lib.rs"), Vec::new())?;
    let mut extra = libraries.externs(1);
    for cfg in ["compiler-builtins", "unmangled-names", "mem"] {
        extra.push("--cfg".into());
        extra.push(format!("feature=\"{cfg}\"").into());
    }
    compile(
        "compiler_builtins",
        library.join("compiler-builtins/compiler-builtins/src/lib.rs"),
        extra,
    )?;
    compile(
        "alloc",
        library.join("alloc/src/lib.rs"),
        libraries.externs(2),
    )?;
    compile(
        "gbc_heap",
        root.join("runtime/heap/lib.rs"),
        libraries.externs(2),
    )?;
    fs::write(&options_file, options)
        .map_err(|error| Error::io(format!("could not write {}", options_file.display()), error))?;
    Ok(libraries)
}

fn read_text(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .map_err(|error| Error::io(format!("could not read {}", path.display()), error))
}

fn assemble(root: &Path, source: &Path, object: &Path) -> Result<()> {
    run_command(
        Command::new("rgbasm")
            .arg("-I")
            .arg(root)
            .arg("-o")
            .arg(object)
            .arg(source),
    )
}

fn absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        env::current_dir()
            .map(|current_dir| current_dir.join(path))
            .map_err(|error| Error::io(format!("could not resolve path {}", path.display()), error))
    }
}

fn run_command(command: &mut Command) -> Result<()> {
    let description = format!("{command:?}");
    let status = command
        .status()
        .map_err(|error| Error::io(format!("could not run {description}"), error))?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::CommandFailed {
            command: description,
            status,
        })
    }
}

struct ArtifactDir(PathBuf);

impl ArtifactDir {
    fn new(source: &Path) -> Result<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(Error::Clock)?
            .as_nanos();
        let name = source.file_stem().unwrap_or_else(|| OsStr::new("game"));
        let path = absolute_path(Path::new("build/artifacts"))?.join(format!(
            "{}-{}-{timestamp}",
            name.to_string_lossy(),
            std::process::id()
        ));
        fs::create_dir_all(&path)
            .map_err(|error| Error::io(format!("could not create {}", path.display()), error))?;
        eprintln!("Build artifacts: {}", path.display());
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::parse_build_args;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn accepts_optimization_levels_around_paths() {
        for level in ["0", "1", "2", "3", "s", "z"] {
            for args in [
                vec!["--opt-level", level, "game.rs", "game.gb"],
                vec!["game.rs", "game.gb", "--opt-level", level],
            ] {
                let (paths, selected, _) =
                    parse_build_args(args.into_iter().map(OsString::from)).unwrap();
                assert_eq!(paths, [PathBuf::from("game.rs"), PathBuf::from("game.gb")]);
                assert_eq!(selected, level);
            }
        }
        let (_, selected, _) = parse_build_args([OsString::from("--opt-level=z")]).unwrap();
        assert_eq!(selected, "z");
        let (paths, selected, _) = parse_build_args([]).unwrap();
        assert!(paths.is_empty());
        assert_eq!(selected, "0");
    }

    #[test]
    fn alloc_is_opt_in() {
        let (_, _, alloc) = parse_build_args([]).unwrap();
        assert!(!alloc);
        for args in [["--alloc", "game.rs"], ["game.rs", "--alloc"]] {
            let (paths, _, alloc) = parse_build_args(args.into_iter().map(OsString::from)).unwrap();
            assert!(alloc);
            assert_eq!(paths, [PathBuf::from("game.rs")]);
        }
    }

    #[test]
    fn rejects_missing_or_invalid_optimization_levels() {
        for args in [
            vec!["--opt-level"],
            vec!["--opt-level", "4"],
            vec!["--opt-level="],
            vec!["--unknown"],
        ] {
            assert!(parse_build_args(args.into_iter().map(OsString::from)).is_err());
        }
    }
}
