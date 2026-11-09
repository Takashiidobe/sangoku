use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

fn pairs() -> [(u16, u16); 9] {
    [
        (0, 0xffff),
        (0xffff, 0),
        (1, 1),
        (0xffff, 0xffff),
        (0x8000, 0xffff),
        (0x8000, 0x8000),
        (0x7fff, 2),
        (0x1234, 0xabcd),
        (0xff00, 0x0101),
    ]
}

#[test]
fn widening_helpers_produce_full_signed_and_unsigned_products() -> Result<()> {
    for variant in ["size", "opt"] {
        for signed in [false, true] {
            let mut sources = vec![(
                "unsigned.asm",
                harness::runtime(&format!("umul16wide_{variant}"))?,
            )];
            if signed {
                sources.push(("signed.asm", harness::runtime("smul16wide")?));
            }
            sources.push(("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()));
            let program = harness::assemble_tiny(&sources, None)?;
            let mut costs = vec![];
            for (lhs, rhs) in pairs() {
                let mut e = program.emulator(if signed {
                    "__gbc_smul16wide"
                } else {
                    "__gbc_umul16wide"
                })?;
                [e.b, e.c] = lhs.to_le_bytes();
                e.a = rhs as u8;
                e.d = 0x55;
                e.e = 0xaa;
                e.set_hl(0xbeef);
                e.set_sp(0xde00);
                e.write16(0xde00, program.symbols["done"]);
                let args = [(rhs >> 8) as u8, 0x12, 0x34];
                e.memory[0xde02..0xde05].copy_from_slice(&args);
                e.reset_metrics();
                e.run_until(program.symbols["done"], 4096)?;
                let actual = u32::from_le_bytes([e.a, e.b, e.c, e.d]);
                let expected = if signed {
                    (i32::from(lhs as i16) * i32::from(rhs as i16)) as u32
                } else {
                    u32::from(lhs) * u32::from(rhs)
                };
                ensure!(
                    actual == expected,
                    "{variant}/{signed}: {lhs:#x} * {rhs:#x} -> {actual:#x}, expected {expected:#x}"
                );
                ensure!(e.sp() == 0xde02 && e.memory[0xde02..0xde05] == args);
                ensure!(e.peak_stack_bytes == if signed { 8 } else { 0 });
                ensure!(
                    e.written_addresses
                        .iter()
                        .all(|address| (0xddf8..0xde00).contains(address))
                );
                costs.push(e.cycles);
            }
            println!(
                "widen16/{variant}/signed={signed}: {} linked bytes; T-cycles {costs:?}",
                program.size("ROM0", &[]) - 1
            );
        }
    }
    Ok(())
}

fn run(program: &harness::Program, entry: &str, width: u32, lhs: u32, rhs: u32) -> Result<[u8; 5]> {
    let mut e = program.emulator(entry)?;
    let n = (width / 8) as usize;
    let args: Vec<_> = lhs.to_le_bytes()[..n]
        .iter()
        .chain(&rhs.to_le_bytes()[..n])
        .copied()
        .collect();
    [e.b, e.c, e.a] = [args[0], args[1], args.get(2).copied().unwrap_or(0xa5)];
    e.set_hl(0xbeef);
    e.set_sp(0xde00);
    e.write16(0xde00, program.symbols["done"]);
    let mut stack = args.get(3..).unwrap_or(&[]).to_vec();
    stack.extend([0x12, 0x34]);
    e.memory[0xde02..0xde02 + stack.len()].copy_from_slice(&stack);
    e.reset_metrics();
    e.run_until(program.symbols["done"], 100_000)?;
    ensure!(
        e.sp() == 0xde02 && e.memory[0xde02..0xde02 + stack.len()] == stack,
        "{entry}: caller stack changed"
    );
    ensure!(
        e.written_addresses
            .iter()
            .all(|address| (0xdc00..0xde00).contains(address)),
        "{entry}: non-stack write"
    );
    Ok([e.a, e.b, e.c, e.d, e.e])
}

fn runtime_sources(variant: &str) -> Result<Vec<(&'static str, String)>> {
    Ok(vec![
        (
            "widen.asm",
            harness::runtime(&format!("umul16wide_{variant}"))?,
        ),
        ("signed.asm", harness::runtime("smul16wide")?),
        ("mul8.asm", harness::runtime(&format!("mul8_{variant}"))?),
        ("mul16.asm", harness::runtime(&format!("mul16_{variant}"))?),
        ("mul32.asm", harness::runtime(&format!("mul32_{variant}"))?),
        ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
    ])
}

#[test]
fn rust_widening_checked_and_mixed_sign_products_use_correct_paths() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    for level in ["0", "z"] {
        let path = directory.path().join(format!("native-{level}.asm"));
        let output=Command::new("rustc").current_dir(harness::root())
            .args(["+gbc-lr35902","-Zunstable-options","-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so","--target","targets/lr35902.json","--emit=asm","--crate-type=lib","-Cpanic=immediate-abort"])
            .arg(format!("-Copt-level={level}"))
            .arg("tests/fixtures/widen-multiply/native.rs").arg("-o").arg(&path).output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let assembly = std::fs::read_to_string(path)?;
        for (entry, symbol) in [
            ("audit_widen_u", "__gbc_umul16wide"),
            ("audit_widen_s", "__gbc_smul16wide"),
            ("audit_widen_mixed", "__gbc_mul32"),
            ("audit_checked_u", "__gbc_umul16wide"),
            ("audit_checked_s", "__gbc_smul16wide"),
        ] {
            let header = format!("SECTION \"__rust_{entry}\"");
            let body = assembly
                .split(&header)
                .nth(1)
                .ok_or("missing Rust probe")?
                .split("\nSECTION ")
                .next()
                .unwrap();
            ensure!(
                body.contains(&format!("call {symbol}")),
                "{entry}/{level} did not select {symbol}"
            );
        }
        let mut sources = runtime_sources(if level == "z" { "size" } else { "opt" })?;
        sources.push(("native.asm", assembly));
        let program = harness::assemble_tiny(&sources, None)?;
        for (lhs, rhs) in pairs() {
            let unsigned = u32::from(lhs) * u32::from(rhs);
            let signed = (i32::from(lhs as i16) * i32::from(rhs as i16)) as u32;
            let mixed = (i32::from(lhs as i16) * i32::from(rhs)) as u32;
            for (entry, expected) in [
                ("audit_widen_u", unsigned),
                ("audit_widen_s", signed),
                ("audit_widen_mixed", mixed),
                ("audit_widen_live", unsigned.wrapping_add(u32::from(lhs))),
            ] {
                let result = run(
                    &program,
                    &format!("__rust_{entry}"),
                    16,
                    u32::from(lhs),
                    u32::from(rhs),
                )?;
                ensure!(
                    u32::from_le_bytes(result[..4].try_into()?) == expected,
                    "{entry}/{level}: {lhs:#x} * {rhs:#x}"
                );
            }
            for (entry, expected) in [
                ("audit_checked_u", lhs.overflowing_mul(rhs).1),
                (
                    "audit_checked_s",
                    (lhs as i16).overflowing_mul(rhs as i16).1,
                ),
            ] {
                ensure!(
                    run(
                        &program,
                        &format!("__rust_{entry}"),
                        16,
                        u32::from(lhs),
                        u32::from(rhs)
                    )?[0]
                        == u8::from(expected)
                );
            }
        }
    }
    Ok(())
}

#[test]
fn clif_high_halves_and_overflow_match_representative_8_16_32_bit_cases() -> Result<()> {
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(harness::root())
        .args(["lower", "tests/fixtures/widen-multiply/shapes.clif", "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembly = String::from_utf8(output.stdout)?;
    ensure!(
        assembly.contains("call __gbc_umul16wide") && assembly.contains("call __gbc_smul16wide")
    );
    for (entry, symbol) in [
        ("umulhi16", "__gbc_umul16wide"),
        ("smulhi16", "__gbc_smul16wide"),
        ("umul_overflow16", "__gbc_umul16wide"),
        ("smul_overflow16", "__gbc_smul16wide"),
        ("umulhi32", "__gbc_umul16wide"),
        ("smulhi32", "__gbc_umul16wide"),
        ("umul_overflow32", "__gbc_umul16wide"),
        ("smul_overflow32", "__gbc_umul16wide"),
    ] {
        let header = format!("SECTION \"__rust_{entry}\"");
        let body = assembly
            .split(&header)
            .nth(1)
            .ok_or("missing CLIF shape")?
            .split("\nSECTION ")
            .next()
            .unwrap();
        ensure!(
            body.contains(&format!("call {symbol}")),
            "{entry} did not select {symbol}"
        );
    }

    for variant in ["size", "opt"] {
        let mut sources = runtime_sources(variant)?;
        sources.push(("callers.asm", assembly.clone()));
        let program = harness::assemble_tiny(&sources, None)?;
        for width in [8, 16, 32] {
            let mask = u32::MAX >> (32 - width);
            let maximum = (1u32 << (width - 1)) - 1;
            for (lhs, rhs) in [
                (0, mask),
                (1, 1),
                (mask, mask),
                (1 << (width - 1), mask),
                (maximum, 2),
                (0x12345678 & mask, 0x89abcdef & mask),
            ] {
                let unsigned = u64::from(lhs) * u64::from(rhs);
                let a = ((i64::from(lhs) << (64 - width)) >> (64 - width)) as i64;
                let b = ((i64::from(rhs) << (64 - width)) >> (64 - width)) as i64;
                let signed = a * b;
                for (op, high, overflow) in [
                    ("u", (unsigned >> width) as u32, unsigned > u64::from(mask)),
                    (
                        "s",
                        ((signed >> width) as u32) & mask,
                        signed < -(1i64 << (width - 1)) || signed > i64::from(maximum),
                    ),
                ] {
                    let result = run(
                        &program,
                        &format!("__rust_{op}mulhi{width}"),
                        width,
                        lhs,
                        rhs,
                    )?;
                    let actual = u32::from_le_bytes(result[..4].try_into()?) & mask;
                    ensure!(
                        actual == high,
                        "{op}mulhi{width}/{variant}: {lhs:#x} * {rhs:#x}, {actual:#x} != {high:#x}"
                    );
                    let result = run(
                        &program,
                        &format!("__rust_{op}mul_overflow{width}"),
                        width,
                        lhs,
                        rhs,
                    )?;
                    let actual = u32::from_le_bytes(result[..4].try_into()?) & mask;
                    ensure!(
                        actual == (unsigned as u32) & mask
                            && result[(width / 8) as usize] == u8::from(overflow),
                        "{op}overflow{width}/{variant}: {lhs:#x} * {rhs:#x}"
                    );
                }
            }
        }
        for (lhs, rhs) in pairs().into_iter().take(6) {
            for (op, expected) in [
                ("u", u32::from(lhs) * u32::from(rhs)),
                ("s", (i32::from(lhs as i16) * i32::from(rhs as i16)) as u32),
            ] {
                let result = run(
                    &program,
                    &format!("__rust_full_{op}16"),
                    16,
                    u32::from(lhs),
                    u32::from(rhs),
                )?;
                ensure!(u32::from_le_bytes(result[..4].try_into()?) == expected);
            }
        }
    }
    Ok(())
}
