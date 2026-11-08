use sangoku_test_support::{Result, emu::Emulator, ensure};
use std::{fs, process::Command};

#[test]
fn bounds_check_aborts_without_a_panic_handler() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("immediate_abort.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", "tests/core_rom/immediate_abort.rs"])
        .arg(&rom)
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut emulator = Emulator::new(&fs::read(&rom)?, 0x100)?;
    emulator.set_sp(0xe000);
    emulator.write8(0xc000, 1);
    for _ in 0..100_000 {
        let before = emulator.pc;
        emulator.step()?;
        if emulator.pc == before {
            ensure!(emulator.pc == 0x11, "did not reach the abort trap");
            ensure!(emulator.serial.is_empty(), "abort wrote serial output");
            return Ok(());
        }
    }
    sangoku_test_support::bail!("bounds check did not abort")
}

#[test]
fn program_using_core_links_and_runs() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("core_rom.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", "tests/core_rom/rom.rs"])
        .arg(&rom)
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut emulator = Emulator::new(&fs::read(&rom)?, 0x100)?;
    emulator.set_sp(0xe000);
    for _ in 0..2_000_000 {
        let before = emulator.pc;
        emulator.step()?;
        if emulator.pc == before {
            ensure!(emulator.a == 28, "gbc_main returned {}", emulator.a);
            return Ok(());
        }
    }
    sangoku_test_support::bail!("program did not halt")
}

fn run_core_rom(source: &str, opt_level: &str) -> Result<u8> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("core_rom.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", source])
        .arg(&rom)
        .args(["--opt-level", opt_level])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut emulator = Emulator::new(&fs::read(&rom)?, 0x100)?;
    emulator.set_sp(0xe000);
    for _ in 0..2_000_000 {
        let before = emulator.pc;
        emulator.step()?;
        if emulator.pc == before {
            return Ok(emulator.a);
        }
    }
    sangoku_test_support::bail!("{source} did not halt")
}

#[test]
fn utf8_iteration_matches_at_every_width_boundary() -> Result<()> {
    let result = run_core_rom("tests/core_rom/utf8.rs", "z")?;
    ensure!(result == 25, "gbc_main returned {result}");
    Ok(())
}

#[test]
fn unicode_properties_match_upstream() -> Result<()> {
    let result = run_core_rom("tests/core_rom/unicode_properties.rs", "z")?;
    ensure!(result == 3, "gbc_main returned {result}");
    Ok(())
}
