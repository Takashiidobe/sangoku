use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

fn values(width: u32) -> Vec<u32> {
    let mask = u32::MAX >> (32 - width);
    let mut values = vec![
        0,
        mask,
        0x55555555 & mask,
        0xaaaaaaaa & mask,
        0x12345678 & mask,
    ];
    values.extend((0..width).map(|bit| 1 << bit));
    values.sort_unstable();
    values.dedup();
    values
}

fn expected(op: &str, width: u32, value: u32) -> u32 {
    match op {
        "clz" => value.leading_zeros() - (32 - width),
        "ctz" => value.trailing_zeros().min(width),
        _ => value.reverse_bits() >> (32 - width),
    }
}

fn run(
    program: &harness::Program,
    entry: &str,
    width: u32,
    value: u32,
    result_width: u32,
) -> Result<(u32, u64, i32)> {
    let mut e = program.emulator(entry)?;
    let input = value.to_le_bytes();
    [e.b, e.c, e.a] = [
        input[0],
        if width > 8 { input[1] } else { 0xaa },
        if width == 32 { input[2] } else { 0xa5 },
    ];
    e.d = 0x55;
    e.e = 0xaa;
    e.set_hl(0xbeef);
    e.set_sp(0xde00);
    e.write16(0xde00, program.symbols["done"]);
    let arguments = [input[3], 0x12, 0x34];
    e.memory[0xde02..0xde05].copy_from_slice(&arguments);
    e.reset_metrics();
    e.run_until(program.symbols["done"], 4096)?;
    ensure!(
        e.sp() == 0xde02 && e.memory[0xde02..0xde05] == arguments,
        "{entry}: stack/arguments changed"
    );
    ensure!(
        e.written_addresses
            .iter()
            .all(|address| (0xdd00..0xde00).contains(address)),
        "{entry}: non-stack write"
    );
    Ok((
        u32::from_le_bytes([e.a, e.b, e.c, e.d]) & (u32::MAX >> (32 - result_width)),
        e.cycles,
        e.peak_stack_bytes,
    ))
}

#[test]
fn native_bit_variants_match_zero_single_bits_and_patterns() -> Result<()> {
    for op in ["clz", "ctz", "bitrev"] {
        for width in [8, 16, 32] {
            for variant in ["size", "opt"] {
                let mut sources = vec![(
                    "entry.asm",
                    harness::runtime(&format!("{op}{width}_{variant}"))?,
                )];
                if op == "bitrev" {
                    sources.push((
                        "byte.asm",
                        harness::runtime(&format!("bitrev_byte_{variant}"))?,
                    ));
                } else if variant == "opt" {
                    sources.push(("byte.asm", harness::runtime(&format!("{op}_byte_opt"))?));
                }
                sources.push(("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()));
                let program = harness::assemble_tiny(&sources, None)?;
                let mut costs = std::collections::BTreeMap::new();
                let mut peak = 0;
                for value in values(width) {
                    let (actual, cycles, stack) =
                        run(&program, &format!("__gbc_{op}{width}"), width, value, width)?;
                    ensure!(
                        actual == expected(op, width, value),
                        "{op}{width}/{variant}: {value:#x} -> {actual:#x}"
                    );
                    ensure!(stack <= 2 && (stack == 0 || variant == "opt" || op == "bitrev"));
                    peak = peak.max(stack);
                    costs.insert(value, cycles);
                }
                let mask = u32::MAX >> (32 - width);
                println!(
                    "{op}{width}_{variant}: {} linked bytes; T-cycles zero/one/msb/all={}/{}/{}/{}; extra stack {peak}",
                    program.size("ROM0", &[]) - 1,
                    costs[&0],
                    costs[&1],
                    costs[&(1 << (width - 1))],
                    costs[&mask]
                );
            }
        }
    }
    Ok(())
}

#[test]
fn rust_bit_operations_use_native_widths_and_preserve_live_inputs() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    for level in ["0", "z"] {
        let path = directory.path().join(format!("native-{level}.asm"));
        let output=Command::new("rustc").current_dir(harness::root())
            .args(["+gbc-lr35902","-Zunstable-options","-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so","--target","targets/lr35902.json","--emit=asm","--crate-type=lib","-Cpanic=immediate-abort"])
            .arg(format!("-Copt-level={level}"))
            .arg("tests/fixtures/bitops/native.rs").arg("-o").arg(&path).output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let assembly = std::fs::read_to_string(path)?;
        let variant = if level == "z" { "size" } else { "opt" };
        let mut contents = vec![
            ("native.asm".to_owned(), assembly.clone()),
            (
                "stop.asm".to_owned(),
                "SECTION \"stop\", ROM0\ndone::\nnop\n".into(),
            ),
        ];
        for op in ["clz", "ctz", "bitrev"] {
            for width in [8, 16, 32] {
                ensure!(assembly.contains(&format!("call __gbc_{op}{width}")));
                contents.push((
                    format!("{op}{width}.asm"),
                    harness::runtime(&format!("{op}{width}_{variant}"))?,
                ));
            }
        }
        if level != "z" {
            for op in ["clz", "ctz"] {
                contents.push((
                    format!("{op}-byte.asm"),
                    harness::runtime(&format!("{op}_byte_opt"))?,
                ));
            }
        }
        contents.push((
            "byte.asm".to_owned(),
            harness::runtime(&format!("bitrev_byte_{variant}"))?,
        ));
        let sources: Vec<_> = contents
            .iter()
            .map(|(name, source)| (name.as_str(), source.clone()))
            .collect();
        let program = harness::assemble_tiny(&sources, None)?;
        for op in ["clz", "ctz", "bitrev"] {
            for width in [8, 16, 32] {
                let mask = u32::MAX >> (32 - width);
                for value in [0, mask, 1, 1 << (width - 1), 0x12345678 & mask] {
                    for suffix in ["", "_signed", "_live"] {
                        let result_width = if suffix == "_live" || op == "bitrev" {
                            width
                        } else {
                            32
                        };
                        let (actual, _, _) = run(
                            &program,
                            &format!("__rust_audit_{op}{width}{suffix}"),
                            width,
                            value,
                            result_width,
                        )?;
                        let result = expected(op, width, value);
                        let result = if suffix == "_live" {
                            result.wrapping_add(value) & mask
                        } else {
                            result
                        };
                        ensure!(
                            actual == result,
                            "{op}{width}/{level}/{suffix}: {value:#x} -> {actual:#x}, expected {result:#x}"
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
