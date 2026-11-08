use sangoku_test_support::{Result, emu::Emulator, ensure};
use std::{fs, process::Command};

fn build_example(name: &str) -> Result<Vec<u8>> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join(format!("{name}.gb"));
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", &format!("examples/{name}.rs")])
        .arg(&rom)
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    ensure!(
        !String::from_utf8_lossy(&output.stderr).contains("sangoku::retained_formatting"),
        "unexpected formatting warning for {name}"
    );
    let report = String::from_utf8_lossy(&output.stderr);
    let artifacts = report
        .lines()
        .find_map(|line| line.strip_prefix("Build artifacts: "))
        .ok_or("build did not report its artifact directory")?;
    let map = fs::read_to_string(std::path::Path::new(artifacts).join("game.map"))?;
    ensure!(!map.contains("4core3fmt"), "{name} retained core::fmt");
    Ok(fs::read(&rom)?)
}

fn run_example(name: &str) -> Result<(String, u8)> {
    let mut emulator = Emulator::new(&build_example(name)?, 0x100)?;
    emulator.set_sp(0xe000);
    for _ in 0..200_000_000u64 {
        let before = emulator.pc;
        emulator.step()?;
        ensure!(
            (0xc000..=0xe000).contains(&emulator.sp()),
            "{name} stack left work RAM: SP={:#06x}",
            emulator.sp()
        );
        if emulator.pc == before {
            return Ok((String::from_utf8(emulator.serial.clone())?, emulator.a));
        }
    }
    sangoku_test_support::bail!("{name} did not halt")
}

#[test]
fn hello_prints_over_serial() -> Result<()> {
    let (text, _) = run_example("hello")?;
    ensure!(text == "Hello, Game Boy!\n6 * 7 = 42\n", "{text:?}");
    Ok(())
}

#[test]
fn primes_prints_the_primes_below_fifty() -> Result<()> {
    let (text, count) = run_example("primes")?;
    ensure!(
        text == "2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 \n15 primes below 50\n",
        "{text:?}"
    );
    ensure!(count == 15, "returned {count}");
    Ok(())
}

#[test]
fn oversized_program_reports_retained_formatting_and_rom_capacity() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("oversized.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "build",
            "tests/fixtures/core/programs/oversized_formatting.rs",
        ])
        .arg(&rom)
        .output()?;
    ensure!(
        !output.status.success(),
        "oversized program unexpectedly linked"
    );
    let report = String::from_utf8(output.stderr)?;
    for expected in [
        "sangoku::retained_formatting",
        "sangoku::rom_capacity",
        "pad_integral",
        "32768",
        "Largest retained sections",
        "probe.map",
    ] {
        ensure!(report.contains(expected), "missing {expected}: {report}");
    }
    ensure!(
        report.find("sangoku::retained_formatting") < report.find("sangoku::rom_capacity"),
        "formatting warning must precede capacity failure: {report}"
    );
    Ok(())
}

#[test]
fn sort_prints_before_and_after() -> Result<()> {
    let (text, result) = run_example("sort")?;
    ensure!(
        text == "before: [42, 7, 1999, 3, 256, 88, 1024, 5]\nafter:  [3, 5, 7, 42, 88, 256, 1024, 1999]\nsum = 3424, max = Some(1999)\n",
        "{text:?}"
    );
    ensure!(result == 0, "returned {result}");
    Ok(())
}

#[test]
fn alloc_fixture_links_heap() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let rom = directory.path().join("heap.gb");
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", "tests/fixtures/alloc/heap.rs", "--alloc"])
        .arg(&rom)
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    ensure!(rom.is_file(), "heap ROM missing");
    Ok(())
}

#[test]
fn screen_draws_text_with_the_lcd_disabled() -> Result<()> {
    let rom = build_example("screen")?;
    for lcdc in [0, 0x91] {
        let mut emulator = Emulator::new(&rom, 0x100)?;
        emulator.memory[0x8000..0xa000].fill(0xa5);
        emulator.memory[0xff40] = lcdc;
        if lcdc != 0 {
            for _ in 0..1_000 {
                emulator.step()?;
            }
            ensure!(
                emulator.memory[0xff40] == lcdc,
                "disabled LCD before vblank"
            );
            ensure!(
                emulator
                    .written_addresses
                    .range(0x8000..0xa000)
                    .next()
                    .is_none(),
                "wrote VRAM before vblank"
            );
            emulator.memory[0xff44] = 144;
        }
        let mut halted = false;
        for _ in 0..2_000_000 {
            let before = emulator.pc;
            let lcd_on = emulator.memory[0xff40] & 0x80 != 0;
            let vram_writes = if lcd_on {
                emulator.written_addresses.range(0x8000..0xa000).count()
            } else {
                0
            };
            emulator.step()?;
            if lcd_on {
                ensure!(
                    emulator.written_addresses.range(0x8000..0xa000).count() == vram_writes,
                    "wrote VRAM while LCD was enabled"
                );
            }
            if emulator.pc == before {
                halted = true;
                break;
            }
        }
        ensure!(halted && emulator.a == 0, "screen setup did not finish");
        ensure!(emulator.memory[0xff40] == 0x91, "incorrect LCD mode");
        ensure!(emulator.memory[0xff47] == 0xe4, "incorrect palette");
        ensure!(
            emulator.memory[0xff42..0xff44] == [0, 0],
            "incorrect scroll"
        );
        let alphabet = b" ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!*=";
        for (row, text) in [
            (4, "SANGOKU"),
            (6, "HELLO GAME BOY!"),
            (8, "RUST ON SCREEN"),
            (10, "6 * 7 = 42"),
        ] {
            let start = 0x9800 + row * 32 + 3;
            let actual: String = emulator.memory[start..start + text.len()]
                .iter()
                .map(|&tile| alphabet[tile as usize] as char)
                .collect();
            ensure!(actual == text, "row {row}: {actual:?}");
        }
        ensure!(
            emulator.memory[0x9800..0x9820] == [0; 32],
            "background was not cleared"
        );
        ensure!(
            emulator.memory[0x8000..0x8010] == [0; 16],
            "space tile is not blank"
        );
        let h = 0x8000 + 8 * 16;
        ensure!(
            emulator.memory[h..h + 6] == [0, 0, 0x22, 0x22, 0x22, 0x22],
            "font bitplanes are incorrect"
        );
    }
    Ok(())
}
