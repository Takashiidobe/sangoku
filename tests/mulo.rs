use sangoku_test_support::{Result, ensure, harness, random::Random};

#[test]
fn u128_overflow_multiply_matches_host() -> Result<()> {
    let program = harness::assemble_tiny(
        &[
            (
                "stop.asm",
                "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
            ),
            ("mulo128.asm", harness::read("runtime/mulo128.asm")?),
        ],
        None,
    )?;
    ensure!(program.size("ROM0", &["u128 overflow multiply"]) <= 256);
    let mut cases = vec![
        (0, u128::MAX),
        (u128::MAX, 0),
        (1, u128::MAX),
        (u128::MAX, 1),
        (u128::MAX, 2),
        (u128::MAX, u128::MAX),
        (1 << 127, 2),
        (1 << 126, 2),
        (1 << 64, 1 << 64),
        ((1 << 64) - 1, (1 << 64) - 1),
        (3, 5),
        (255, 257),
        (u128::MAX / 3, 3),
        (u128::MAX / 3 + 1, 3),
        (0xaaaa_aaaa_aaaa_aaaa, 0x5555_5555_5555_5555),
    ];
    let mut random = Random::new(0x128);
    for _ in 0..32 {
        let mut next = || {
            (0..4).fold(0u128, |value, _| {
                (value << 32) | u128::from(random.bits(32))
            })
        };
        cases.push((next(), next()));
    }
    for (lhs, rhs) in cases {
        let mut emulator = program.emulator("__rust___gbc_u128_mulo")?;
        let (left, right, out, stack) = (0xc101u16, 0xc201u16, 0xc301u16, 0xdf00u16);
        emulator.memory[left as usize..left as usize + 16].copy_from_slice(&lhs.to_le_bytes());
        emulator.memory[right as usize..right as usize + 16].copy_from_slice(&rhs.to_le_bytes());
        emulator.memory[out as usize - 1..out as usize + 18].fill(0xa5);
        emulator.b = out as u8;
        emulator.c = (out >> 8) as u8;
        emulator.a = left as u8;
        emulator.set_sp(stack);
        emulator.write16(stack, program.symbols["done"]);
        emulator.write8(stack + 2, (left >> 8) as u8);
        emulator.write16(stack + 3, right);
        emulator.run_until(program.symbols["done"], 200_000)?;
        let expected = lhs.overflowing_mul(rhs);
        let actual = u128::from_le_bytes(
            emulator.memory[out as usize..out as usize + 16]
                .try_into()
                .unwrap(),
        );
        ensure!(actual == expected.0, "{lhs:#x} * {rhs:#x}: {actual:#x}");
        ensure!(
            emulator.memory[out as usize + 16] == u8::from(expected.1),
            "{lhs:#x} * {rhs:#x}: overflow"
        );
        ensure!(emulator.memory[out as usize - 1] == 0xa5);
        ensure!(emulator.memory[out as usize + 17] == 0xa5);
        ensure!(emulator.memory[left as usize..left as usize + 16] == lhs.to_le_bytes());
        ensure!(emulator.memory[right as usize..right as usize + 16] == rhs.to_le_bytes());
        ensure!(emulator.sp() == stack + 2);
        ensure!(emulator.memory[usize::from(stack + 2)] == (left >> 8) as u8);
        ensure!(emulator.read16(stack + 3) == right);
    }
    Ok(())
}
