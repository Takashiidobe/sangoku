use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use sangoku::emu::Emulator;

mod link;

const TOOLCHAIN: &str = "gbc-lr35902";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("sangoku: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let Some(command) = args.next() else {
        return Err(usage());
    };

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let (paths, opt_level) = parse_build_args(args)?;
    match command.to_str() {
        Some("example") => {
            let output = paths
                .first()
                .cloned()
                .unwrap_or_else(|| PathBuf::from("build/hello.gb"));
            if paths.len() > 1 {
                return Err(usage());
            }
            build_rom(&root, &root.join("examples/hello.rs"), &output, &opt_level)
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
            build_rom(&root, source, &output, &opt_level)
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
            build_rom(&root, source, &output, &opt_level)?;
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
            emit_mir(&root, source, &output, &opt_level)
        }
        Some("clif") => {
            let Some(source) = paths.first() else {
                return Err(usage());
            };
            let output = paths.get(1).cloned().unwrap_or_else(|| {
                let name = source.file_stem().unwrap_or_else(|| OsStr::new("game"));
                PathBuf::from("build").join(name).with_extension("clif")
            });
            if paths.len() > 2 {
                return Err(usage());
            }
            emit_clif(&root, source, &output, &opt_level)
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

fn run_rom(rom: &Path) -> Result<(), String> {
    let image =
        fs::read(rom).map_err(|error| format!("could not read {}: {error}", rom.display()))?;
    let mut emulator = Emulator::new(&image, 0x100).map_err(|error| error.to_string())?;
    emulator.set_sp(0xFFFE);
    let mut stdout = io::stdout().lock();
    let mut printed = 0;
    for _ in 0..MAX_INSTRUCTIONS {
        let before = emulator.pc;
        emulator.step().map_err(|error| error.to_string())?;
        if emulator.serial.len() > printed {
            stdout
                .write_all(&emulator.serial[printed..])
                .and_then(|()| stdout.flush())
                .map_err(|error| format!("could not write stdout: {error}"))?;
            printed = emulator.serial.len();
        }
        if emulator.pc == before {
            eprintln!("halted with gbc_main returning {}", emulator.a);
            return Ok(());
        }
    }
    Err(format!(
        "program did not halt within {MAX_INSTRUCTIONS} instructions"
    ))
}

fn usage() -> String {
    "usage: sangoku example [output.gb] [--opt-level 0|1|2|3|s|z] | sangoku build <source.rs> [output.gb] [--opt-level 0|1|2|3|s|z] | sangoku run <source.rs> [--opt-level 0|1|2|3|s|z] | sangoku frontend <source.rs> [output.mir|-] [--opt-level 0|1|2|3|s|z] | sangoku clif <source.rs> [output.clif|-] [--opt-level 0|1|2|3|s|z] | sangoku lower <input.clif> [output.s|-]".into()
}

fn parse_build_args(
    args: impl IntoIterator<Item = OsString>,
) -> Result<(Vec<PathBuf>, String), String> {
    let mut args = args.into_iter();
    let mut paths = Vec::new();
    let mut opt_level = "0".to_owned();
    while let Some(arg) = args.next() {
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
            return Err(format!("unknown build option: {}", arg.to_string_lossy()));
        } else {
            paths.push(PathBuf::from(arg));
        }
    }
    Ok((paths, opt_level))
}

fn rustc_command(root: &Path, opt_level: &str) -> Command {
    let mut command = Command::new("rustc");
    command
        .current_dir(root)
        .arg(format!("+{TOOLCHAIN}"))
        .arg("-Zunstable-options")
        .arg("--target")
        .arg(root.join("targets/lr35902.json"))
        .args([
            "--crate-type=lib",
            "-Cpanic=abort",
            "-Cdebug-assertions=off",
        ])
        .arg(format!("--remap-path-prefix={}=.", root.display()))
        .arg(format!("-Copt-level={opt_level}"));
    command
}

fn build_backend(root: &Path) -> Result<PathBuf, String> {
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

fn lower_clif(root: &Path, input: &Path, output: &Path) -> Result<(), String> {
    let input = absolute_path(input)?;
    if !input.is_file() {
        return Err(format!("CLIF input does not exist: {}", input.display()));
    }
    let mut command = Command::new("cargo");
    command
        .current_dir(root)
        .args(["run", "-q", "-p", "sangoku-lower", "--"])
        .arg(&input)
        .arg("-");
    let result = command
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| format!("could not run {command:?}: {error}"))?;
    if !result.status.success() {
        return Err(format!("{command:?} exited with {}", result.status));
    }
    write_output(output, result.stdout)
}

fn emit_clif(root: &Path, source: &Path, output: &Path, opt_level: &str) -> Result<(), String> {
    let source = absolute_path(source)?;
    if !source.is_file() {
        return Err(format!("Rust source does not exist: {}", source.display()));
    }

    let backend = build_backend(root)?;
    let temp = TempDir::new()?;
    run_command(
        rustc_command(root, opt_level)
            .env("CG_CLIF_CLIF_ONLY", "1")
            .arg(format!("-Zcodegen-backend={}", backend.display()))
            .arg("--emit=asm,llvm-ir")
            .arg(&source)
            .arg("-o")
            .arg(temp.path().join("game.s")),
    )?;
    let clif = fs::read(temp.path().join("game.ll"))
        .map_err(|error| format!("backend did not write CLIF: {error}"))?;
    write_output(output, clif)
}

fn write_output(output: &Path, contents: Vec<u8>) -> Result<(), String> {
    if output == Path::new("-") {
        return io::stdout()
            .write_all(&contents)
            .map_err(|error| format!("could not write stdout: {error}"));
    }
    let output = absolute_path(output)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    fs::write(&output, contents)
        .map_err(|error| format!("could not write {}: {error}", output.display()))?;
    println!("Wrote {}", output.display());
    Ok(())
}

fn emit_mir(root: &Path, source: &Path, output: &Path, opt_level: &str) -> Result<(), String> {
    let source = absolute_path(source)?;
    if !source.is_file() {
        return Err(format!("Rust source does not exist: {}", source.display()));
    }

    let mut command = rustc_command(root, opt_level);
    command.arg("-Zunpretty=mir").arg(&source);
    if output == Path::new("-") {
        return run_command(&mut command);
    }

    let result = command
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| format!("could not run {command:?}: {error}"))?;
    if !result.status.success() {
        return Err(format!("{command:?} exited with {}", result.status));
    }
    write_output(output, result.stdout)
}

fn build_rom(root: &Path, source: &Path, output: &Path, opt_level: &str) -> Result<(), String> {
    let source = absolute_path(source)?;
    if !source.is_file() {
        return Err(format!("Rust source does not exist: {}", source.display()));
    }

    let output = absolute_path(output)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }

    let backend = build_backend(root)?;
    let temp = TempDir::new()?;
    let rust_asm = temp.path().join("game.asm");
    let libraries = core_libraries(root, &backend, opt_level)?;

    let mut command = rustc_command(root, opt_level);
    command
        .arg(format!("-Zcodegen-backend={}", backend.display()))
        .arg("--emit=asm")
        .args(libraries.externs(true))
        .arg(source)
        .arg("-o")
        .arg(&rust_asm);
    run_command(&mut command)?;

    let texts = [
        read_text(&rust_asm)?,
        read_text(&libraries.core_asm())?,
        read_text(&libraries.builtins_asm())?,
    ];
    let sections: Vec<_> = texts
        .iter()
        .flat_map(|text| link::split_sections(text))
        .collect();
    let kept: Vec<_> = link::reachable(&sections, &[link::ENTRY])
        .into_iter()
        .map(|index| &sections[index])
        .collect();
    if kept.is_empty() {
        return Err(format!(
            "{} is not defined by the program or core",
            link::ENTRY
        ));
    }
    let linked: String = kept.iter().map(|section| section.text).collect();

    let header_object = temp.path().join("header.o");
    assemble(&root.join("rom/header.asm"), &header_object)?;
    let mut fixed_objects = vec![header_object];
    for name in link::runtime_objects(&linked) {
        let object = temp.path().join(format!("{name}.o"));
        let mut source = root.join(format!("runtime/{name}.asm"));
        if name == "panic"
            && let Some(symbol) = link::panic_symbol(&linked)
        {
            source = temp.path().join("panic.asm");
            fs::write(
                &source,
                read_text(&root.join("runtime/panic.asm"))?.replace("__gbc_panic", symbol),
            )
            .map_err(|error| format!("could not write {}: {error}", source.display()))?;
        }
        assemble(&source, &object)?;
        fixed_objects.push(object);
    }

    let probe_source = temp.path().join("probe.asm");
    let probe_object = temp.path().join("probe.o");
    let probe_map = temp.path().join("probe.map");
    fs::write(&probe_source, link::bank_one(&linked))
        .map_err(|error| format!("could not write {}: {error}", probe_source.display()))?;
    assemble(&probe_source, &probe_object)?;
    run_command(
        Command::new("rgblink")
            .arg("-m")
            .arg(&probe_map)
            .arg("-o")
            .arg(temp.path().join("probe.gb"))
            .args(&fixed_objects)
            .arg(&probe_object),
    )?;
    let (sizes, rom0_free) = link::parse_map(&read_text(&probe_map)?);

    let game_source = temp.path().join("game_placed.asm");
    let game_object = temp.path().join("game.o");
    fs::write(&game_source, link::place(&kept, &sizes, rom0_free))
        .map_err(|error| format!("could not write {}: {error}", game_source.display()))?;
    assemble(&game_source, &game_object)?;
    run_command(
        Command::new("rgblink")
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

    fn externs(&self, with_builtins: bool) -> Vec<OsString> {
        let mut args = vec![
            "-L".into(),
            format!("dependency={}", self.0.display()).into(),
        ];
        let crates = [
            ("core", "libcore.rmeta"),
            ("compiler_builtins", "libcompiler_builtins.rmeta"),
        ];
        for (name, file) in crates.into_iter().take(if with_builtins { 2 } else { 1 }) {
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

fn core_libraries(root: &Path, backend: &Path, opt_level: &str) -> Result<CoreLibraries, String> {
    let libraries = CoreLibraries(root.join("build/core").join(format!("opt-{opt_level}")));
    let backend_time = modified(backend);
    let fresh = [libraries.core_asm(), libraries.builtins_asm()]
        .iter()
        .all(|path| modified(path).is_some_and(|time| Some(time) >= backend_time));
    if fresh {
        return Ok(libraries);
    }
    fs::create_dir_all(&libraries.0)
        .map_err(|error| format!("could not create {}: {error}", libraries.0.display()))?;
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
    let mut extra = libraries.externs(false);
    for cfg in ["compiler-builtins", "unmangled-names", "mem"] {
        extra.push("--cfg".into());
        extra.push(format!("feature=\"{cfg}\"").into());
    }
    compile(
        "compiler_builtins",
        library.join("compiler-builtins/compiler-builtins/src/lib.rs"),
        extra,
    )?;
    Ok(libraries)
}

fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

fn assemble(source: &Path, object: &Path) -> Result<(), String> {
    run_command(Command::new("rgbasm").arg("-o").arg(object).arg(source))
}

fn absolute_path(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        env::current_dir()
            .map(|current_dir| current_dir.join(path))
            .map_err(|error| format!("could not resolve path {}: {error}", path.display()))
    }
}

fn run_command(command: &mut Command) -> Result<(), String> {
    let description = format!("{command:?}");
    let status = command
        .status()
        .map_err(|error| format!("could not run {description}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{description} exited with {status}"))
    }
}

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Result<Self, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("system clock error: {error}"))?
            .as_nanos();
        let path = env::temp_dir().join(format!("sangoku-{}-{timestamp}", std::process::id()));
        fs::create_dir(&path)
            .map_err(|error| format!("could not create {}: {error}", path.display()))?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
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
                let (paths, selected) =
                    parse_build_args(args.into_iter().map(OsString::from)).unwrap();
                assert_eq!(paths, [PathBuf::from("game.rs"), PathBuf::from("game.gb")]);
                assert_eq!(selected, level);
            }
        }
        let (_, selected) = parse_build_args([OsString::from("--opt-level=z")]).unwrap();
        assert_eq!(selected, "z");
        let (paths, selected) = parse_build_args([]).unwrap();
        assert!(paths.is_empty());
        assert_eq!(selected, "0");
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
