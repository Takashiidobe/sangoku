use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

fn sources() -> Result<Vec<(&'static str, String)>> {
    let mut result = Vec::new();
    for name in [
        "sdiv8",
        "srem8",
        "div8_core",
        "sdiv16",
        "srem16",
        "div16_core",
        "sdiv32",
        "srem32",
        "div32_core",
    ] {
        result.push((name, harness::runtime(name)?));
    }
    result.push(("stop", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()));
    Ok(result)
}

fn pairs(bits: u32) -> Vec<(i64, i64)> {
    let min = -(1i64 << (bits - 1));
    let max = -min - 1;
    let mut pairs = vec![
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (7, 3),
        (-7, 3),
        (7, -3),
        (-7, -3),
        (min, 1),
        (min, -1),
        (min, -2),
        (min, min),
        (max, -1),
        (max, min),
        (min + 1, max),
        (1, 0),
        (0, 0),
    ];
    let mut seed = 0x8149_7623u32;
    for _ in 0..8 {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let a = (i64::from(seed) << (64 - bits)) >> (64 - bits);
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let b = (i64::from(seed) << (64 - bits)) >> (64 - bits);
        pairs.push((a, b));
    }
    pairs
}

fn check(program: &harness::Program, prefix: &str) -> Result<()> {
    for bits in [8, 16, 32] {
        for op in ["sdiv", "srem"] {
            let entry = format!("{prefix}{op}{bits}");
            let mut costs = Vec::new();
            let mut stack = 0;
            for (a, b) in pairs(bits) {
                let trap = b == 0 || op == "sdiv" && a == -(1i64 << (bits - 1)) && b == -1;
                if prefix == "__rust_audit_" && (trap || a == -(1i64 << (bits - 1)) && b == -1) {
                    continue;
                }
                let mut e = program.emulator(&entry)?;
                let n = (bits / 8) as usize;
                let args: Vec<_> = (a as u32).to_le_bytes()[..n]
                    .iter()
                    .chain(&(b as u32).to_le_bytes()[..n])
                    .copied()
                    .collect();
                e.b = args[0];
                e.c = args[1];
                e.a = args.get(2).copied().unwrap_or(0xa5);
                e.set_sp(0xde00);
                e.write16(0xde00, program.symbols["done"]);
                let mut saved = args.get(3..).unwrap_or(&[]).to_vec();
                saved.extend([0x12, 0x34]);
                e.memory[0xde02..0xde02 + saved.len()].copy_from_slice(&saved);
                e.reset_metrics();
                e.run_until(if trap { 0x10 } else { program.symbols["done"] }, 65536)?;
                ensure!(
                    e.memory[0xde02..0xde02 + saved.len()] == saved,
                    "{entry}: arguments overwritten"
                );
                ensure!(
                    e.written_addresses
                        .iter()
                        .all(|address| (0xdc00..0xde00).contains(address)),
                    "{entry}: non-stack write"
                );
                if !trap {
                    let expected = if op == "sdiv" { a / b } else { a % b };
                    let mask = (1u64 << bits) - 1;
                    let got = u32::from_le_bytes([e.a, e.b, e.c, e.d]);
                    ensure!(
                        u64::from(got) & mask == expected as u64 & mask,
                        "{entry}: {a} {b}: {got:x}, expected {expected:x}"
                    );
                    ensure!(e.sp() == 0xde02, "{entry}: stack not restored");
                }
                costs.push(e.cycles);
                stack = stack.max(e.peak_stack_bytes);
            }
            println!("{entry}: cycles {costs:?}; peak local stack {stack}");
        }
    }
    Ok(())
}

#[test]
fn signed_helpers_handle_signs_boundaries_traps_and_abi() -> Result<()> {
    for bits in [8, 16, 32] {
        let names = [
            format!("sdiv{bits}"),
            format!("srem{bits}"),
            format!("div{bits}_core"),
        ];
        let objects: Vec<_> = sources()?
            .into_iter()
            .filter(|(name, _)| *name == "stop" || names.iter().any(|n| n == name))
            .collect();
        let program = harness::assemble_tiny(&objects, None)?;
        println!(
            "signed i{bits} pair and core: {} ROM bytes",
            program.size("ROM0", &[]) - 1
        );
    }
    let program = harness::assemble_tiny(&sources()?, None)?;
    println!(
        "signed helpers and shared cores: {} bytes",
        program.size("ROM0", &[]) - 1
    );
    check(&program, "__gbc_")
}

#[test]
fn rust_signed_division_selects_native_width_helpers() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    for level in ["0", "z"] {
        let path = directory.path().join("native.asm");
        let output = Command::new("rustc").current_dir(harness::root())
            .args(["+gbc-lr35902", "-Zunstable-options", "-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so", "--target", "targets/lr35902.json", "--emit=asm", "--crate-type=lib", "-Cpanic=immediate-abort"])
            .arg(format!("-Copt-level={level}"))
            .arg("tests/fixtures/signed-division/native.rs").arg("-o").arg(&path).output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let assembly = std::fs::read_to_string(path)?;
        for bits in [8, 16, 32] {
            for op in ["sdiv", "srem"] {
                let header = format!("SECTION \"__rust_audit_{op}{bits}\"");
                let body = assembly
                    .split(&header)
                    .nth(1)
                    .ok_or("missing probe")?
                    .split("\nSECTION ")
                    .next()
                    .unwrap();
                ensure!(
                    body.contains(&format!("call __gbc_{op}{bits}")),
                    "{level}: {header}: wrong helper\n{body}"
                );
                ensure!(!body.contains("__div") && !body.contains("__mod"));
            }
        }
        let mut objects = sources()?;
        objects.push(("native", assembly));
        let program = harness::assemble_tiny(&objects, None)?;
        check(&program, "__rust_audit_")?;
    }
    Ok(())
}
