use sangoku_test_support::{Result, emu::Emulator, ensure};
use std::{fs, path::Path, process::Command};

#[test]
fn all_levels_remove_panic_formatting_and_preserve_checks() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    for level in ["0", "1", "2", "3", "s", "z"] {
        println!("checking immediate-abort behavior at opt-level {level}");
        for (name, source) in [
            ("sort", "examples/sort.rs"),
            ("panic", "tests/core_rom/panic_format.rs"),
            ("range", "tests/core_rom/range_check.rs"),
        ] {
            let rom = directory.path().join(format!("{name}-{level}.gb"));
            let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .args(["build", source, "--opt-level", level])
                .arg(&rom)
                .output()?;
            let report = String::from_utf8(output.stderr)?;
            ensure!(
                output.status.success(),
                "{name}, opt-level {level}: {report}"
            );
            ensure!(
                !report.contains("sangoku::retained_formatting"),
                "{name}, opt-level {level}: {report}"
            );
            let artifacts = report
                .lines()
                .find_map(|line| line.strip_prefix("Build artifacts: "))
                .ok_or("build did not report its artifact directory")?;
            let map = fs::read_to_string(Path::new(artifacts).join("game.map"))?;
            ensure!(
                !map.contains("4core3fmt"),
                "{name}, opt-level {level}: retained core::fmt"
            );
            let image = fs::read(&rom)?;
            let inputs: &[u8] = match name {
                "range" => &[2, 4],
                "panic" => &[0, 1, 2, 3],
                _ => &[0],
            };
            for &input in inputs {
                let mut emulator = Emulator::new(&image, 0x100)?;
                emulator.memory[0xc000] = input;
                if name == "panic" {
                    emulator.write16(0xc000, 12345);
                    emulator.memory[0xc020] = input;
                }
                let mut halted = false;
                for _ in 0..2_000_000 {
                    let before = emulator.pc;
                    emulator.step()?;
                    if emulator.pc == before {
                        halted = true;
                        break;
                    }
                }
                ensure!(halted, "{name}, opt-level {level}: did not halt");
                match name {
                    "sort" => {
                        ensure!(emulator.a == 0, "sort returned {}", emulator.a);
                        ensure!(emulator.serial == b"before: [42, 7, 1999, 3, 256, 88, 1024, 5]\nafter:  [3, 5, 7, 42, 88, 256, 1024, 1999]\nsum = 3424, max = Some(1999)\n", "sort output differs at opt-level {level}");
                    }
                    "panic" => {
                        ensure!(emulator.pc == 0x11, "formatted panic did not trap");
                        ensure!(
                            emulator.memory[0xc010] == 0x5a,
                            "panic argument effect was lost"
                        );
                    }
                    _ if input > 3 => ensure!(emulator.pc == 0x11, "range failure did not trap"),
                    _ => ensure!(
                        emulator.a == input && emulator.pc != 0x11,
                        "valid range failed"
                    ),
                }
            }
        }
    }
    Ok(())
}
