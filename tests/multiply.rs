use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

fn cases(width: u32) -> Vec<(u32, u32)> {
    let mask = u32::MAX >> (32 - width);
    [
        (0, mask),
        (1, 1),
        (mask, 0),
        (mask, mask),
        (1 << (width - 1), 3),
        (0x12345678, 0x89abcdef),
        (0xff, 0x1234),
        (0x1234, 0xff),
    ]
    .into_iter()
    .map(|(a, b)| (a & mask, b & mask))
    .collect()
}

fn run(
    program: &harness::Program,
    entry: &str,
    width: u32,
    lhs: u32,
    rhs: u32,
) -> Result<(u32, u64, i32)> {
    let mut e = program.emulator(entry)?;
    let n = (width / 8) as usize;
    let arguments: Vec<_> = lhs.to_le_bytes()[..n]
        .iter()
        .chain(&rhs.to_le_bytes()[..n])
        .copied()
        .collect();
    [e.b, e.c, e.a] = [
        arguments[0],
        arguments[1],
        arguments.get(2).copied().unwrap_or(0xa5),
    ];
    e.d = 0x55;
    e.e = 0xaa;
    e.set_hl(0xbeef);
    e.set_sp(0xde00);
    e.write16(0xde00, program.symbols["done"]);
    let mut stack = arguments.get(3..).unwrap_or(&[]).to_vec();
    stack.extend([0x12, 0x34]);
    e.memory[0xde02..0xde02 + stack.len()].copy_from_slice(&stack);
    e.reset_metrics();
    e.run_until(program.symbols["done"], 20_000)?;
    ensure!(
        e.sp() == 0xde02 && e.memory[0xde02..0xde02 + stack.len()] == stack,
        "{entry}: stack/arguments changed"
    );
    ensure!(
        e.written_addresses
            .iter()
            .all(|address| (0xdd00..0xde00).contains(address)),
        "{entry}: non-stack write"
    );
    Ok((
        u32::from_le_bytes([e.a, e.b, e.c, e.d]) & (u32::MAX >> (32 - width)),
        e.cycles,
        e.peak_stack_bytes,
    ))
}

#[test]
fn multiply_variants_match_a_few_wrapping_cases_and_preserve_abi() -> Result<()> {
    for width in [8, 16, 32] {
        let mut costs = vec![];
        for variant in ["size", "opt"] {
            let program = harness::assemble_tiny(
                &[
                    (
                        "multiply.asm",
                        harness::runtime(&format!("mul{width}_{variant}"))?,
                    ),
                    ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
                ],
                None,
            )?;
            let mut cycles = vec![];
            for (lhs, rhs) in cases(width) {
                let (actual, cost, stack) =
                    run(&program, &format!("__gbc_mul{width}"), width, lhs, rhs)?;
                ensure!(
                    actual == lhs.wrapping_mul(rhs) & (u32::MAX >> (32 - width)),
                    "i{width}/{variant}: {lhs:#x} * {rhs:#x} = {actual:#x}"
                );
                ensure!(stack == if width == 32 { 8 } else { 0 });
                cycles.push(cost);
            }
            let bytes = program.size("ROM0", &[&format!("{width}-bit multiplication")]);
            println!("mul{width}_{variant}: {bytes} ROM bytes; case T-cycles {cycles:?}");
            costs.push((bytes, cycles));
        }
        ensure!(costs[0].0 < costs[1].0);
        ensure!(
            costs[1].1[3] < costs[0].1[3],
            "speed variant did not improve full-width multiplication"
        );
    }
    Ok(())
}

#[test]
fn rust_wrapping_multiply_uses_native_helpers_with_live_inputs() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    for level in ["0", "z"] {
        let path = directory.path().join(format!("native-{level}.asm"));
        let output = Command::new("rustc").current_dir(harness::root())
            .args(["+gbc-lr35902","-Zunstable-options","-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so","--target","targets/lr35902.json","--emit=asm","--crate-type=lib","-Cpanic=immediate-abort"])
            .arg(format!("-Copt-level={level}"))
            .arg("tests/fixtures/multiply/native.rs").arg("-o").arg(&path).output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let assembly = std::fs::read_to_string(path)?;
        for width in [8, 16, 32] {
            ensure!(assembly.contains(&format!("call __gbc_mul{width}")));
        }
        let variant = if level == "z" { "size" } else { "opt" };
        let program = harness::assemble_tiny(
            &[
                ("native.asm", assembly),
                ("i8.asm", harness::runtime(&format!("mul8_{variant}"))?),
                ("i16.asm", harness::runtime(&format!("mul16_{variant}"))?),
                ("i32.asm", harness::runtime(&format!("mul32_{variant}"))?),
                ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
            ],
            None,
        )?;
        for width in [8, 16, 32] {
            for (lhs, rhs) in cases(width).into_iter().take(6) {
                for suffix in ["", "_signed", "_live"] {
                    let (actual, _, _) = run(
                        &program,
                        &format!("__rust_audit_mul{width}{suffix}"),
                        width,
                        lhs,
                        rhs,
                    )?;
                    let product = lhs.wrapping_mul(rhs);
                    let expected = if suffix == "_live" {
                        product.wrapping_add(lhs)
                    } else {
                        product
                    };
                    ensure!(
                        actual == expected & (u32::MAX >> (32 - width)),
                        "i{width}/{level}/{suffix}: {lhs:#x} * {rhs:#x}"
                    );
                }
            }
        }
    }
    Ok(())
}
