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

const BASE: u16 = 0xC200;

fn call(
    program: &harness::Program,
    name: &str,
    memory: &[(u16, u8)],
) -> Result<sangoku_test_support::emu::Emulator> {
    let stop = program.symbols["done"];
    let mut emulator = program.emulator(name)?;
    for (address, byte) in memory {
        emulator.memory[*address as usize] = *byte;
    }
    emulator.b = BASE as u8;
    emulator.c = (BASE >> 8) as u8;
    emulator.set_sp(STACK - 2);
    emulator.write16(STACK - 2, stop);
    emulator.run_until(stop, 8192)?;
    Ok(emulator)
}

#[test]
fn memory_offsets_beyond_a_signed_byte() -> Result<()> {
    let program = program("load-store-offsets")?;

    let e = call(&program, "__rust_ld8_300", &[(BASE + 300, 0x9c)])?;
    assert_eq!(e.a, 0x9c);

    let e = call(
        &program,
        "__rust_ld16_n200",
        &[(BASE - 200, 0x34), (BASE - 199, 0x12)],
    )?;
    assert_eq!((e.a, e.b), (0x34, 0x12));

    let bytes = [0x11u8, 0x22, 0x33, 0x44];
    let memory: Vec<_> = bytes
        .iter()
        .enumerate()
        .map(|(i, b)| (BASE + 126 + i as u16, *b))
        .collect();
    let e = call(&program, "__rust_ld32_126", &memory)?;
    assert_eq!((e.a, e.b, e.c, e.d), (0x11, 0x22, 0x33, 0x44));

    let e = call(
        &program,
        "__rust_uld16_128",
        &[(BASE + 128, 0xef), (BASE + 129, 0xbe)],
    )?;
    assert_eq!((e.a, e.b, e.c, e.d), (0xef, 0xbe, 0, 0));

    let e = call(&program, "__rust_uld8_n5", &[(BASE - 5, 0xf0)])?;
    assert_eq!((e.a, e.b), (0xf0, 0));

    let e = call(&program, "__rust_st8_300", &[])?;
    assert_eq!(e.memory[(BASE + 300) as usize], 0x5a);

    let e = call(&program, "__rust_ist16_n5", &[(BASE - 3, 0xee)])?;
    assert_eq!(
        e.memory[(BASE - 5) as usize..(BASE - 2) as usize],
        [0x78, 0x56, 0xee]
    );

    let e = call(&program, "__rust_ist8_127", &[(BASE + 128, 0xee)])?;
    assert_eq!(
        e.memory[(BASE + 127) as usize..(BASE + 129) as usize],
        [0x66, 0xee]
    );
    Ok(())
}
