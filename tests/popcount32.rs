use sangoku_test_support::{Result, ensure, harness, random::Random};

#[test]
fn popcount32_variants_preserve_abi_and_trade_rom_for_speed() -> Result<()> {
    let mut values = vec![0, u32::MAX, 0x55555555, 0xaaaaaaaa, 0x01234567, 0x89abcdef];
    for bit in 0..32 {
        values.extend([1u32 << bit, !(1u32 << bit)]);
    }
    for byte in [0u32, 1, 0x55, 0x7f, 0x80, 0xaa, 0xfe, 0xff] {
        for shift in [0, 8, 16, 24] {
            values.push(byte << shift);
        }
    }
    let mut random = Random::new(0x706f7063);
    for _ in 0..40 {
        values.push(random.below(u32::MAX));
    }
    let mut measurements = vec![];
    for variant in ["size", "opt"] {
        let program = harness::assemble_tiny(
            &[
                (
                    "helper.asm",
                    harness::read(format!("runtime/popcnt32_{variant}.asm"))?,
                ),
                (
                    "stop.asm",
                    "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
                ),
            ],
            None,
        )?;
        let bytes = program.size("ROM0", &["32-bit population count"]);
        let mut costs = vec![];
        for &value in &values {
            let mut e = program.emulator("__gbc_popcnt32")?;
            let input = value.to_le_bytes();
            [e.b, e.c, e.a] = [input[0], input[1], input[2]];
            e.d = 0xa5;
            e.e = 0x5a;
            e.set_hl(0x1234);
            e.set_sp(0xde00);
            e.write16(0xde00, program.symbols["done"]);
            e.memory[0xde02..0xde06].copy_from_slice(&[input[3], 0xaa, 0xbb, 0xcc]);
            e.reset_metrics();
            e.run_until(program.symbols["done"], 4096)?;
            let actual = u32::from_le_bytes([e.a, e.b, e.c, e.d]);
            ensure!(
                actual == value.count_ones(),
                "{variant}({value:#x}) = {actual}"
            );
            ensure!(e.sp() == 0xde02 && e.memory[0xde02..0xde06] == [input[3], 0xaa, 0xbb, 0xcc]);
            ensure!(
                e.written_addresses.is_empty(),
                "{variant}: unexpected memory write"
            );
            ensure!(e.peak_stack_bytes == 0, "{variant}: unexpected stack use");
            costs.push(e.cycles);
        }
        ensure!(
            costs.iter().all(|&cycles| cycles == costs[0]),
            "{variant}: input-dependent timing"
        );
        println!(
            "popcnt32_{variant}: {bytes} ROM bytes, {} T-cycles including RET, 0 additional stack bytes, {} inputs",
            costs[0],
            values.len()
        );
        measurements.push((bytes, costs[0]));
    }
    ensure!(measurements[0].0 < measurements[1].0);
    ensure!(measurements[1].1 < measurements[0].1);
    Ok(())
}
