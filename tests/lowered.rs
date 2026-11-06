use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

const STACK: u16 = 0xD000;
const DATA: u16 = 0xC100;

fn program(fixture: &str) -> Result<harness::Program> {
    let path = format!(
        "{}/tests/fixtures/emulate/{fixture}.clif",
        harness::root().display()
    );
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .args(["lower", &path, "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    harness::assemble(
        &[
            ("lowered.s", String::from_utf8(output.stdout)?),
            (
                "stop.s",
                "SECTION \"done\", ROM0\nEXPORT done\ndone::\n  nop\n".into(),
            ),
        ],
        None,
    )
}

#[test]
fn i32_load_and_store_round_trip() -> Result<()> {
    let program = program("load-store-i32")?;
    let stop = program.symbols["done"];

    let mut load = program.emulator("__rust_ld")?;
    load.memory[DATA as usize..DATA as usize + 4].copy_from_slice(&0xA1B2C3D4u32.to_le_bytes());
    load.b = DATA as u8;
    load.c = (DATA >> 8) as u8;
    load.set_sp(STACK - 2);
    load.write16(STACK - 2, stop);
    load.run_until(stop, 4096)?;
    let value = u32::from(load.a)
        | u32::from(load.b) << 8
        | u32::from(load.c) << 16
        | u32::from(load.d) << 24;
    assert_eq!(value, 0xA1B2C3D4);

    let mut store = program.emulator("__rust_st")?;
    store.memory[DATA as usize..DATA as usize + 6].fill(0xEE);
    store.b = DATA as u8;
    store.c = (DATA >> 8) as u8;
    store.set_sp(STACK - 2);
    store.write16(STACK - 2, stop);
    store.run_until(stop, 4096)?;
    assert_eq!(
        store.memory[DATA as usize..DATA as usize + 6],
        [0x78, 0x56, 0x34, 0x12, 0xEE, 0xEE]
    );
    Ok(())
}
