use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

const TOOLCHAIN: &str = "nightly-2026-11-05";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("gbc-rs: {error}");
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
    match command.to_str() {
        Some("example") => {
            let output = args
                .next()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("build/blank.gb"));
            if args.next().is_some() {
                return Err(usage());
            }
            build_rom(&root, &root.join("examples/blank.rs"), &output)
        }
        Some("build") => {
            let Some(source) = args.next().map(PathBuf::from) else {
                return Err(usage());
            };
            let output = args.next().map(PathBuf::from).unwrap_or_else(|| {
                let name = source.file_stem().unwrap_or_else(|| OsStr::new("game"));
                PathBuf::from("build").join(name).with_extension("gb")
            });
            if args.next().is_some() {
                return Err(usage());
            }
            build_rom(&root, &source, &output)
        }
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "usage: gbc-rs example [output.gb] | gbc-rs build <source.rs> [output.gb]".into()
}

fn build_rom(root: &Path, source: &Path, output: &Path) -> Result<(), String> {
    let source = absolute_path(source)?;
    if !source.is_file() {
        return Err(format!("Rust source does not exist: {}", source.display()));
    }

    let output = absolute_path(output)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }

    run_command(
        Command::new("cargo")
            .current_dir(root)
            .arg(format!("+{TOOLCHAIN}"))
            .args(["build", "--release", "--locked", "--manifest-path"])
            .arg(root.join("third_party/rustc_codegen_cranelift/Cargo.toml")),
    )?;

    let backend = root
        .join("third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so");
    let temp = TempDir::new()?;
    let rust_asm = temp.path().join("game.asm");

    run_command(
        Command::new("rustc")
            .current_dir(root)
            .arg(format!("+{TOOLCHAIN}"))
            .arg("-Zunstable-options")
            .arg(format!("-Zcodegen-backend={}", backend.display()))
            .arg("--target")
            .arg(root.join("targets/lr35902.json"))
            .args(["--emit=asm", "--crate-type=lib", "-Cpanic=abort"])
            .arg(source)
            .arg("-o")
            .arg(&rust_asm),
    )?;

    let header_object = temp.path().join("header.o");
    let game_object = temp.path().join("game.o");
    run_command(
        Command::new("rgbasm")
            .arg("-o")
            .arg(&header_object)
            .arg(root.join("rom/header.asm")),
    )?;
    run_command(
        Command::new("rgbasm")
            .arg("-o")
            .arg(&game_object)
            .arg(rust_asm),
    )?;
    run_command(
        Command::new("rgblink")
            .arg("-o")
            .arg(&output)
            .arg(header_object)
            .arg(game_object),
    )?;
    run_command(
        Command::new("rgbfix")
            .args(["-v", "-p", "0", "-t", "GBC-RS"])
            .arg(&output),
    )?;

    println!("Wrote {}", output.display());
    Ok(())
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
        let path = env::temp_dir().join(format!("gbc-rs-{}-{timestamp}", std::process::id()));
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
