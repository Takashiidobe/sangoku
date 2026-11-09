use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

const OPS: [&str; 5] = ["shl", "ushr", "sshr", "rotl", "rotr"];
const COUNTS: [u8; 12] = [0, 1, 7, 8, 9, 31, 32, 33, 63, 64, 127, 255];

fn expected(op: usize, bits: u32, value: u128, count: u8) -> u128 {
    let mask = u128::MAX >> (128 - bits);
    let value = value & mask;
    let n = u32::from(count) % bits;
    let signed = ((value << (128 - bits)) as i128) >> (128 - bits);
    let rotl = |n: u32| {
        if n == 0 {
            value
        } else {
            ((value << n) | (value >> (bits - n))) & mask
        }
    };
    match op {
        0 => (value << n) & mask,
        1 => value >> n,
        2 => (signed >> n) as u128 & mask,
        3 => rotl(n),
        _ => rotl((bits - n) % bits),
    }
}

fn values(bits: u32) -> [u128; 4] {
    let mask = u128::MAX >> (128 - bits);
    [
        0,
        1,
        0x8000_0000_0000_0000_0000_0000_0000_0001 & mask,
        0x0123_4567_89ab_cdef_fedc_ba98_7654_3210 & mask,
    ]
}

#[test]
fn wide_shift_helpers_match_host_for_boundary_counts() -> Result<()> {
    for bits in [64u32, 128] {
        let width = (bits / 8) as usize;
        for variant in ["size", "opt"] {
            let mut sources = vec![
                (
                    "entry.asm",
                    harness::runtime(&format!("shift{bits}_{variant}"))?,
                ),
                (
                    "bit.asm",
                    harness::runtime(&format!("shift_bit{bits}_{variant}"))?,
                ),
                ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
            ];
            if variant == "opt" {
                sources.push((
                    "byte.asm",
                    harness::runtime(&format!("shift_byte{bits}_opt"))?,
                ));
            }
            let program = harness::assemble_tiny(&sources, None)?;
            let mut worst = 0;
            for (op, name) in OPS.iter().enumerate() {
                for value in values(bits) {
                    for count in COUNTS {
                        let mut e = program.emulator(&format!("__gbc_shift{bits}"))?;
                        let context = 0xdc00u16;
                        e.memory[usize::from(context)..][..width]
                            .copy_from_slice(&value.to_le_bytes()[..width]);
                        e.memory[usize::from(context) + width] = op as u8;
                        e.memory[usize::from(context) + width + 1] = count;
                        [e.b, e.c] = context.to_le_bytes();
                        e.d = 0x55;
                        e.e = 0xaa;
                        e.set_sp(0xde00);
                        e.write16(0xde00, program.symbols["done"]);
                        e.reset_metrics();
                        e.run_until(program.symbols["done"], 500_000)?;
                        let mut actual = [0u8; 16];
                        actual[..width].copy_from_slice(&e.memory[usize::from(context)..][..width]);
                        let actual = u128::from_le_bytes(actual);
                        let want = expected(op, bits, value, count);
                        ensure!(
                            actual == want,
                            "{variant}/{name}/{bits}: {value:#x} by {count} -> {actual:#x}, expected {want:#x}"
                        );
                        ensure!(e.sp() == 0xde02, "{variant}/{name}: stack unbalanced");
                        worst = worst.max(e.cycles);
                    }
                }
            }
            println!(
                "shift{bits}/{variant}: {} linked bytes; worst sampled T-cycles {worst}",
                program.size("ROM0", &[]) - 1
            );
        }
    }
    Ok(())
}

#[test]
fn rust_wide_shifts_and_rotates_call_the_helpers_and_match_host() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    for level in ["0", "z"] {
        let variant = if level == "z" { "size" } else { "opt" };
        let path = directory.path().join(format!("native-{level}.asm"));
        let output = Command::new("rustc")
            .current_dir(harness::root())
            .args(["+gbc-lr35902","-Zunstable-options","-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so","--target","targets/lr35902.json","--emit=asm","--crate-type=lib","-Cpanic=immediate-abort"])
            .arg(format!("-Copt-level={level}"))
            .arg("tests/fixtures/wide-shift/native.rs")
            .arg("-o")
            .arg(&path)
            .output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let assembly = std::fs::read_to_string(path)?;
        for bits in [64u32, 128] {
            for name in OPS {
                let entry = format!("{name}{bits}");
                let header = format!("SECTION \"__rust_{entry}\"");
                let body = assembly
                    .split(&header)
                    .nth(1)
                    .ok_or("missing Rust probe")?
                    .split("\nSECTION ")
                    .next()
                    .unwrap();
                ensure!(
                    body.contains(&format!("call __gbc_shift{bits}")),
                    "{entry}/{level} did not call the shared helper"
                );
            }
        }
        let mut sources = vec![("native.asm", assembly)];
        for bits in [64, 128] {
            sources.push((
                "entry.asm",
                harness::runtime(&format!("shift{bits}_{variant}"))?,
            ));
            sources.push((
                "bit.asm",
                harness::runtime(&format!("shift_bit{bits}_{variant}"))?,
            ));
            if variant == "opt" {
                sources.push((
                    "byte.asm",
                    harness::runtime(&format!("shift_byte{bits}_opt"))?,
                ));
            }
        }
        sources.push(("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()));
        let program = harness::assemble(&sources, None)?;
        for bits in [64u32, 128] {
            let width = (bits / 8) as usize;
            for (op, name) in OPS.iter().enumerate() {
                for value in values(bits) {
                    for count in [0u8, 1, 8, 33, (bits - 1) as u8] {
                        let mut e = program.emulator(&format!("__rust_{name}{bits}"))?;
                        let context = 0xc000u16;
                        e.memory[usize::from(context)..][..width]
                            .copy_from_slice(&value.to_le_bytes()[..width]);
                        [e.b, e.c] = context.to_le_bytes();
                        e.a = count;
                        e.set_sp(0xde00);
                        e.write16(0xde00, program.symbols["done"]);
                        e.memory[0xde02..0xde05].copy_from_slice(&[0, 0, 0]);
                        e.run_until(program.symbols["done"], 1_000_000)?;
                        let mut actual = [0u8; 16];
                        actual[..width].copy_from_slice(&e.memory[usize::from(context)..][..width]);
                        let actual = u128::from_le_bytes(actual);
                        let want = expected(op, bits, value, count);
                        ensure!(
                            actual == want,
                            "{level}/{name}{bits}: {value:#x} by {count} -> {actual:#x}, expected {want:#x}"
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
