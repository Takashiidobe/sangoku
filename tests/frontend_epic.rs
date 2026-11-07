use sangoku_test_support::{Result, emu::Emulator, ensure};
use std::{fs, process::Command};

fn build_fixture(
    name: &str,
    level: &str,
) -> Result<(
    sangoku_test_support::tempfile::TempDir,
    std::path::PathBuf,
    std::process::Output,
)> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join(format!("{name}.gb"));
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", &format!("tests/fixtures/clif/{name}.rs")])
        .arg(&rom)
        .args(["--opt-level", level])
        .output()?;
    Ok((directory, rom, output))
}

fn run_fixture(name: &str) -> Result<()> {
    run_fixture_at_level(name, "0")
}

fn run_fixture_at_level(name: &str, level: &str) -> Result<()> {
    let (_directory, rom, output) = build_fixture(name, level)?;
    ensure!(
        output.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut emulator = Emulator::new(&fs::read(rom)?, 0x100)?;
    emulator.set_sp(0xe000);
    for _ in 0..2_000_000 {
        let before = emulator.pc;
        emulator.step()?;
        if emulator.pc == before {
            ensure!(emulator.a == 1, "{name} returned {}", emulator.a);
            return Ok(());
        }
    }
    sangoku_test_support::bail!("{name} did not halt")
}

#[test]
fn abi_pass_modes() -> Result<()> {
    run_fixture("abi-pass-modes")
}

#[test]
fn data_relocations() -> Result<()> {
    run_fixture("data-relocations")
}

#[test]
fn indirect_dispatch() -> Result<()> {
    run_fixture("indirect-dispatch")
}

#[test]
fn wide_scalars() -> Result<()> {
    run_fixture("wide-scalars")
}

#[test]
fn runtime_calls() -> Result<()> {
    run_fixture("runtime-calls")
}

#[test]
fn core_intrinsics() -> Result<()> {
    run_fixture("core-intrinsics")
}

#[test]
fn frontend_optimization() -> Result<()> {
    run_fixture("frontend-optimization")
}

#[test]
fn wide_float16() -> Result<()> {
    run_fixture("wide-float16")
}

#[test]
fn wide_float64() -> Result<()> {
    run_fixture("wide-float64")
}

#[test]
fn wide_float128() -> Result<()> {
    run_fixture("wide-float128")
}

#[test]
fn wide_overflow_multiply() -> Result<()> {
    for level in ["0", "z"] {
        run_fixture_at_level("wide-overflow-mul", level)?;
    }
    Ok(())
}
