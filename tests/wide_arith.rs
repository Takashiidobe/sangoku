use sangoku_test_support::{Result, ensure, harness};

const OPS: [&str; 7] = ["udiv", "urem", "sdiv", "srem", "umulhi", "smulhi", "imul"];

fn sext(value: u128, bits: u32) -> i128 {
    ((value << (128 - bits)) as i128) >> (128 - bits)
}

fn expected(op: usize, bits: u32, a: u128, b: u128) -> std::result::Result<u128, u8> {
    let mask = u128::MAX >> (128 - bits);
    let (a, b) = (a & mask, b & mask);
    let (sa, sb) = (sext(a, bits), sext(b, bits));
    match op {
        0 | 1 | 2 | 3 if b == 0 => Err(1),
        2 if sb == -1 && sa == sext(1 << (bits - 1), bits) => Err(2),
        0 => Ok(a / b),
        1 => Ok(a % b),
        2 => Ok(sa.wrapping_div(sb) as u128 & mask),
        3 => Ok(sa.wrapping_rem(sb) as u128 & mask),
        4 | 5 | 6 => {
            let (low, high) = if bits == 128 {
                wide_mul(a, b, op == 5, sa, sb)
            } else if op == 5 {
                let product = sa * sb;
                (product as u128 & mask, (product >> bits) as u128 & mask)
            } else {
                let product = a * b;
                (product & mask, (product >> bits) & mask)
            };
            Ok(if op == 6 { low } else { high })
        }
        _ => unreachable!(),
    }
}

fn wide_mul(a: u128, b: u128, signed: bool, sa: i128, sb: i128) -> (u128, u128) {
    let low = a.wrapping_mul(b);
    let (a0, a1) = (a & u128::from(u64::MAX), a >> 64);
    let (b0, b1) = (b & u128::from(u64::MAX), b >> 64);
    let mid = (a0 * b0 >> 64) + (a1 * b0 & u128::from(u64::MAX)) + (a0 * b1 & u128::from(u64::MAX));
    let mut high = a1 * b1 + (a1 * b0 >> 64) + (a0 * b1 >> 64) + (mid >> 64);
    if signed {
        if sa < 0 {
            high = high.wrapping_sub(b);
        }
        if sb < 0 {
            high = high.wrapping_sub(a);
        }
    }
    (low, high)
}

fn values(bits: u32) -> Vec<u128> {
    let mask = u128::MAX >> (128 - bits);
    let min = 1u128 << (bits - 1);
    [
        0,
        1,
        2,
        3,
        7,
        255,
        256,
        0x1234_5678,
        0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        min,
        min + 1,
        min - 1,
        mask,
        mask - 1,
        mask >> 1,
    ]
    .into_iter()
    .map(|v| v & mask)
    .collect()
}

#[test]
fn wide_arithmetic_helpers_match_host() -> Result<()> {
    for bits in [64u32, 128] {
        let width = (bits / 8) as usize;
        let program = harness::assemble_tiny(
            &[
                ("entry.asm", harness::runtime(&format!("wide{bits}"))?),
                ("core.asm", harness::runtime("wide_core")?),
                ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
            ],
            None,
        )?;
        let vals = values(bits);
        let mut worst = 0;
        for (op, name) in OPS.iter().enumerate() {
            for &a in &vals {
                for &b in &vals {
                    let mut e = program.emulator(&format!("__gbc_wide{bits}"))?;
                    let context = 0xdc00u16;
                    let base = usize::from(context);
                    e.memory[base..][..width].copy_from_slice(&a.to_le_bytes()[..width]);
                    e.memory[base + width..][..width].copy_from_slice(&b.to_le_bytes()[..width]);
                    e.memory[base + 2 * width] = op as u8;
                    e.memory[base + 2 * width + 1] = 0xee;
                    [e.b, e.c] = context.to_le_bytes();
                    e.set_sp(0xde00);
                    e.write16(0xde00, program.symbols["done"]);
                    e.reset_metrics();
                    e.run_until(program.symbols["done"], 4_000_000)?;
                    ensure!(e.sp() == 0xde02, "{name}/{bits}: stack unbalanced");
                    let status = e.memory[base + 2 * width + 1];
                    match expected(op, bits, a, b) {
                        Err(code) => ensure!(
                            status == code,
                            "{name}/{bits}: {a:#x},{b:#x} status {status}, expected {code}"
                        ),
                        Ok(want) => {
                            let mut actual = [0u8; 16];
                            actual[..width].copy_from_slice(&e.memory[base..][..width]);
                            let actual = u128::from_le_bytes(actual);
                            ensure!(status == 0, "{name}/{bits}: {a:#x},{b:#x} status {status}");
                            ensure!(
                                actual == want,
                                "{name}/{bits}: {a:#x},{b:#x} -> {actual:#x}, expected {want:#x}"
                            );
                        }
                    }
                    worst = worst.max(e.cycles);
                }
            }
        }
        println!(
            "wide{bits}: {} linked bytes; worst sampled T-cycles {worst}",
            program.size("ROM0", &[]) - 1
        );
    }
    Ok(())
}

#[test]
fn rust_wide_division_and_multiplication_call_the_helpers_and_match_host() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let probes: [(&str, u32, usize); 9] = [
        ("div_u64", 64, 0),
        ("rem_u64", 64, 1),
        ("div_i64", 64, 2),
        ("rem_i64", 64, 3),
        ("div_u128", 128, 0),
        ("rem_u128", 128, 1),
        ("div_i128", 128, 2),
        ("rem_i128", 128, 3),
        ("mul_u128", 128, 6),
    ];
    for level in ["0", "z"] {
        let path = directory.path().join(format!("native-{level}.asm"));
        let output = std::process::Command::new("rustc")
            .current_dir(harness::root())
            .args(["+gbc-lr35902","-Zunstable-options","-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so","--target","targets/lr35902.json","--emit=asm","--crate-type=lib","-Cpanic=immediate-abort"])
            .arg(format!("-Copt-level={level}"))
            .arg("tests/fixtures/wide-arith/native.rs")
            .arg("-o")
            .arg(&path)
            .output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let assembly = std::fs::read_to_string(path)?;
        for (entry, bits, _) in probes {
            let header = format!("SECTION \"__rust_{entry}\"");
            let body = assembly
                .split(&header)
                .nth(1)
                .ok_or("missing Rust probe")?
                .split("\nSECTION ")
                .next()
                .unwrap();
            ensure!(
                body.contains(&format!("call __gbc_wide{bits}")),
                "{entry}/{level} did not call the shared helper"
            );
        }
        let program = harness::assemble(
            &[
                ("native.asm", assembly),
                ("wide64.asm", harness::runtime("wide64")?),
                ("wide128.asm", harness::runtime("wide128")?),
                ("core.asm", harness::runtime("wide_core")?),
                ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
            ],
            None,
        )?;
        for (entry, _, _) in probes {
            println!(
                "{entry}/{level}: {} caller bytes",
                program.size("ROM0", &[&format!("__rust_{entry}")])
            );
        }
        for (entry, bits, op) in probes {
            let width = (bits / 8) as usize;
            let values = values(bits);
            for &a in &values {
                for &b in &values {
                    let Ok(want) = expected(op, bits, a, b) else {
                        continue;
                    };
                    let mut e = program.emulator(&format!("__rust_{entry}"))?;
                    let context = 0xc000u16;
                    let base = usize::from(context);
                    e.memory[base..][..width].copy_from_slice(&a.to_le_bytes()[..width]);
                    e.memory[base + width..][..width].copy_from_slice(&b.to_le_bytes()[..width]);
                    [e.b, e.c] = context.to_le_bytes();
                    e.set_sp(0xde00);
                    e.write16(0xde00, program.symbols["done"]);
                    e.run_until(program.symbols["done"], 6_000_000)?;
                    let mut actual = [0u8; 16];
                    actual[..width].copy_from_slice(&e.memory[base..][..width]);
                    let actual = u128::from_le_bytes(actual);
                    ensure!(
                        actual == want,
                        "{level}/{entry}: {a:#x},{b:#x} -> {actual:#x}, expected {want:#x}"
                    );
                }
            }
        }
    }
    Ok(())
}
