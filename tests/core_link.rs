use sangoku_test_support::{Result, emu::Emulator, ensure};
use std::{fs, process::Command};

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
