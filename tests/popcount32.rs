use sangoku_test_support::{Result, ensure, harness, random::Random};

fn values(width: u32) -> Vec<u32> {
    let mask = u32::MAX >> (32 - width);
    let mut values = vec![0, mask, 0x55555555 & mask, 0xaaaaaaaa & mask];
    for bit in 0..width {
        values.extend([1u32 << bit, !(1u32 << bit) & mask]);
    }
    let mut random = Random::new(0x706f7063);
    for _ in 0..24 {
        values.push(random.below(u32::MAX) & mask);
    }
    values
}

#[test]
fn popcount_variants_preserve_abi_and_share_only_the_speed_core() -> Result<()> {
    let mut independent_speed_bytes = 0;
    for width in [8, 16, 32] {
        let mut measurements = vec![];
        for variant in ["size", "opt"] {
            let mut sources = vec![(
                "helper.asm",
                harness::runtime(&format!("popcnt{width}_{variant}"))?,
            )];
            if variant == "opt" {
                sources.push(("byte.asm", harness::runtime("popcnt_byte")?));
            }
            sources.push((
                "stop.asm",
                "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
            ));
            let program = harness::assemble_tiny(&sources, None)?;
            let bytes = program.size("ROM0", &[]) - 0x101;
            let mut costs = vec![];
            for value in values(width) {
                let mut e = program.emulator(&format!("__gbc_popcnt{width}"))?;
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
                let actual = match width {
                    8 => u32::from(e.a),
                    16 => u32::from(u16::from_le_bytes([e.a, e.b])),
                    _ => u32::from_le_bytes([e.a, e.b, e.c, e.d]),
                };
                ensure!(
                    actual == value.count_ones(),
                    "{width}/{variant}({value:#x}) = {actual}"
                );
                ensure!(
                    e.sp() == 0xde02 && e.memory[0xde02..0xde06] == [input[3], 0xaa, 0xbb, 0xcc]
                );
                ensure!(
                    e.written_addresses
                        .iter()
                        .all(|address| (0xddfe..0xde00).contains(address)),
                    "unexpected non-stack write"
                );
                ensure!(e.peak_stack_bytes == if variant == "opt" && width != 8 { 2 } else { 0 });
                costs.push(e.cycles);
            }
            ensure!(
                costs.iter().all(|&cycles| cycles == costs[0]),
                "input-dependent timing"
            );
            println!(
                "popcnt{width}_{variant}: {bytes} linked ROM bytes, {} T-cycles including RET",
                costs[0]
            );
            measurements.push((bytes, costs[0]));
        }
        independent_speed_bytes += measurements[1].0;
        ensure!(measurements[1].1 < measurements[0].1);
        if width != 32 {
            ensure!(measurements[0].0 < measurements[1].0);
        }
    }
    let sources: Vec<_> = ["popcnt8_opt", "popcnt16_opt", "popcnt32_opt", "popcnt_byte"]
        .into_iter()
        .map(|name| Ok((name, harness::runtime(name)?)))
        .collect::<Result<_>>()?;
    let program = harness::assemble_tiny(&sources, None)?;
    println!(
        "all speed widths: {} linked ROM bytes",
        program.size("ROM0", &[])
    );
    ensure!(program.size("ROM0", &[]) < independent_speed_bytes);
    Ok(())
}

#[test]
fn clz32_candidate_matches_host_and_preserves_argument() -> Result<()> {
    let program = harness::assemble_tiny(
        &[
            ("helper.asm", harness::runtime("clz32")?),
            ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
        ],
        None,
    )?;
    for value in values(32) {
        let mut e = program.emulator("__gbc_clz32")?;
        let input = value.to_le_bytes();
        [e.b, e.c, e.a] = [input[0], input[1], input[2]];
        e.set_sp(0xde00);
        e.write16(0xde00, program.symbols["done"]);
        e.write8(0xde02, input[3]);
        e.reset_metrics();
        e.run_until(program.symbols["done"], 4096)?;
        ensure!(u32::from_le_bytes([e.a, e.b, e.c, e.d]) == value.leading_zeros());
        ensure!(e.sp() == 0xde02 && e.memory[0xde02] == input[3]);
        ensure!(e.written_addresses.is_empty());
    }
    Ok(())
}

#[test]
fn shared_byte_core_preserves_wrapper_registers() -> Result<()> {
    let program = harness::assemble_tiny(
        &[
            ("byte.asm", harness::runtime("popcnt_byte")?),
            ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
        ],
        None,
    )?;
    for value in values(8) {
        let mut e = program.emulator("__gbc_popcnt_byte")?;
        e.a = value as u8;
        [e.b, e.c, e.d, e.e] = [0x12, 0x34, 0x56, 0x78];
        e.set_sp(0xde00);
        e.write16(0xde00, program.symbols["done"]);
        e.reset_metrics();
        e.run_until(program.symbols["done"], 128)?;
        ensure!(u32::from(e.a) == value.count_ones());
        ensure!([e.b, e.c, e.d, e.e] == [0x12, 0x34, 0x56, 0x78]);
        ensure!(e.sp() == 0xde02 && e.written_addresses.is_empty());
    }
    Ok(())
}

#[test]
fn original_candidates_have_valid_counts_but_different_abis() -> Result<()> {
    let compact = harness::read("tests/fixtures/popcount/candidates/popcnt8_size.asm")?;
    let pointer = harness::read("tests/fixtures/popcount/candidates/popcnt.asm")?;
    let table = harness::read("tests/fixtures/popcount/candidates/popcnt8_opt.asm")?;
    for (name, source) in [
        (
            "compact",
            format!("SECTION \"candidate\", ROM0[$400]\n{compact}{pointer}"),
        ),
        (
            "table",
            format!("SECTION \"candidate\", ROM0[$400]\n{table}"),
        ),
    ] {
        let source = source
            .replace("popcount8:", "popcount8::")
            .replace("popcount16:", "popcount16::")
            .replace("popcount32:", "popcount32::")
            .replace("popcount8_fast:", "popcount8_fast::")
            .replace("ALIGN 8", "ds (-@) & $ff");
        let program = harness::assemble_tiny(
            &[
                ("candidate.asm", source),
                ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
            ],
            None,
        )?;
        println!(
            "original {name} candidate: {} ROM bytes including internal alignment",
            program.size("ROM0", &["candidate"])
        );
        for value in values(8) {
            let entry = if name == "compact" {
                "popcount8"
            } else {
                "popcount8_fast"
            };
            let mut e = program.emulator(entry)?;
            e.a = value as u8;
            e.set_sp(0xde00);
            e.write16(0xde00, program.symbols["done"]);
            e.run_until(program.symbols["done"], 128)?;
            ensure!(u32::from(e.a) == value.count_ones());
        }
        if name == "compact" {
            for (entry, bytes) in [("popcount16", 2), ("popcount32", 4)] {
                for value in values(32) {
                    let mut e = program.emulator(entry)?;
                    e.set_hl(0xc000);
                    e.memory[0xc000..0xc004].copy_from_slice(&value.to_le_bytes());
                    e.set_sp(0xde00);
                    e.write16(0xde00, program.symbols["done"]);
                    e.run_until(program.symbols["done"], 4096)?;
                    ensure!(
                        u32::from(e.a) == (value & (u32::MAX >> (32 - 8 * bytes))).count_ones()
                    );
                }
            }
        }
    }
    Ok(())
}

#[test]
fn rust_popcounts_compile_without_core_and_preserve_live_inputs() -> Result<()> {
    use std::process::Command;
    let directory = sangoku_test_support::tempfile::tempdir()?;
    for level in ["0", "2", "z"] {
        let output_path = directory.path().join(format!("native-{level}.asm"));
        let output = Command::new("rustc")
            .current_dir(harness::root())
            .args(["+gbc-lr35902", "-Zunstable-options", "-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so", "--target", "targets/lr35902.json", "--emit=asm", "--crate-type=lib", "-Cpanic=immediate-abort"])
            .arg(format!("-Copt-level={level}"))
            .arg("tests/fixtures/popcount/native.rs").arg("-o").arg(&output_path).output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let assembly = std::fs::read_to_string(&output_path)?;
        ensure!(assembly.contains("__gbc_popcnt8") && assembly.contains("__gbc_popcnt32"));
        ensure!(assembly.contains("__gbc_popcnt16") == (level != "2"));
        let mut sources = vec![
            ("native.asm", assembly.clone()),
            ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
        ];
        for width in [8, 16, 32] {
            if assembly.contains(&format!("__gbc_popcnt{width}")) {
                sources.push((
                    match width {
                        8 => "i8.asm",
                        16 => "i16.asm",
                        _ => "i32.asm",
                    },
                    harness::runtime(&format!(
                        "popcnt{width}_{}",
                        if level == "z" { "size" } else { "opt" }
                    ))?,
                ));
            }
        }
        if level != "z" {
            sources.push(("byte.asm", harness::runtime("popcnt_byte")?));
        }
        sources.push(("clz.asm", harness::runtime("clz32")?));
        let program = harness::assemble_tiny(&sources, None)?;
        for width in [8, 16, 32] {
            let mask = u32::MAX >> (32 - width);
            for value in [0, mask, 0x8001 & mask, 0xa5a5a5a5 & mask] {
                for suffix in ["", "_signed", "_live", "_clz"] {
                    let mut e =
                        program.emulator(&format!("__rust_audit_popcount{width}{suffix}"))?;
                    let input = value.to_le_bytes();
                    [e.b, e.c, e.a] = [input[0], input[1], input[2]];
                    e.set_sp(0xde00);
                    e.write16(0xde00, program.symbols["done"]);
                    e.write8(0xde02, input[3]);
                    e.run_until(program.symbols["done"], 4096)?;
                    ensure!(e.sp() == 0xde02 && e.memory[0xde02] == input[3]);
                    let mut actual = u32::from_le_bytes([e.a, e.b, e.c, e.d]);
                    let expected = match suffix {
                        "_clz" => value.leading_zeros() - (32 - width),
                        "_live" => {
                            actual &= mask;
                            value.wrapping_add(value.count_ones()) & mask
                        }
                        _ => value.count_ones(),
                    };
                    ensure!(
                        actual == expected,
                        "{width}/{level}/{suffix}: {value:#x}: {actual} != {expected}"
                    );
                }
            }
        }
    }
    Ok(())
}
