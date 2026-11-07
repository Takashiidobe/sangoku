use sangoku_test_support::{Result, emu::Emulator, ensure};
use std::{fs, process::Command};

fn run_example(name: &str) -> Result<(String, u8)> {
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
    let mut emulator = Emulator::new(&fs::read(&rom)?, 0x100)?;
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
fn sort_prints_before_and_after() -> Result<()> {
    let (text, _) = run_example("sort")?;
    ensure!(
        text == "before: [42, 7, 1999, 3, 256, 88, 1024, 5]\nafter:  [3, 5, 7, 42, 88, 256, 1024, 1999]\nsum = 3424, max = Some(1999)\n",
        "{text:?}"
    );
    Ok(())
}
