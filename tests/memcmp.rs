use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

fn expected(left: &[u8], right: &[u8]) -> i32 {
    match left.cmp(right) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn run(
    program: &harness::Program,
    entry: &str,
    left: &[u8],
    right: &[u8],
    offset: usize,
) -> Result<i32> {
    let mut e = program.emulator(entry)?;
    let lhs = 0xc100 + offset;
    let rhs = 0xc800 + offset;
    e.memory[lhs..lhs + left.len()].copy_from_slice(left);
    e.memory[rhs..rhs + right.len()].copy_from_slice(right);
    [e.b, e.c] = (lhs as u16).to_le_bytes();
    e.a = rhs as u8;
    e.d = 0x55;
    e.e = 0xaa;
    e.set_hl(0xbeef);
    e.set_sp(0xde00);
    e.write16(0xde00, program.symbols["done"]);
    let len = (left.len() as u16).to_le_bytes();
    let arguments = [(rhs >> 8) as u8, len[0], len[1], 0xaa, 0x55];
    e.memory[0xde02..0xde07].copy_from_slice(&arguments);
    e.reset_metrics();
    e.run_until(program.symbols["done"], 50_000)?;
    ensure!(e.sp() == 0xde02 && e.memory[0xde02..0xde07] == arguments);
    ensure!(
        e.written_addresses
            .iter()
            .all(|address| (0xdd00..0xde00).contains(address)),
        "comparison wrote outside its stack"
    );
    Ok(i32::from_le_bytes([e.a, e.b, e.c, e.d]))
}

fn cases(len: usize) -> Vec<(Vec<u8>, Vec<u8>)> {
    let pattern: Vec<_> = (0..len)
        .map(|i| (i as u8).wrapping_mul(37).wrapping_add(128))
        .collect();
    let mut cases = vec![(pattern.clone(), pattern.clone())];
    if len > 0 {
        for index in [0, len / 2, len - 1]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
        {
            for (a, b) in [(0, 255), (255, 0)] {
                let mut left = pattern.clone();
                let mut right = pattern.clone();
                left[index] = a;
                right[index] = b;
                cases.push((left, right));
            }
        }
    }
    cases
}

#[test]
fn memcmp_variants_preserve_unsigned_ordering_and_abi() -> Result<()> {
    for variant in ["size", "opt"] {
        let program = harness::assemble_tiny(
            &[
                (
                    "compare.asm",
                    harness::runtime(&format!("memcmp_{variant}"))?,
                ),
                ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
            ],
            None,
        )?;
        println!(
            "memcmp_{variant}: {} ROM bytes",
            program.size("ROM0", &["memcmp"])
        );
        for len in [0, 1, 2, 3, 4, 7, 8, 16, 255, 256, 257, 1024] {
            for (left, right) in cases(len) {
                for offset in [0, 1] {
                    ensure!(
                        run(&program, "__rust_memcmp", &left, &right, offset)?.signum()
                            == expected(&left, &right)
                    );
                }
            }
        }
        let mut e = program.emulator("__rust_memcmp")?;
        [e.b, e.c, e.a] = [0xff, 0xff, 0xfe];
        e.memory[0xffff] = 0;
        e.memory[0xfffe] = 255;
        e.set_sp(0xde00);
        e.write16(0xde00, program.symbols["done"]);
        e.memory[0xde02..0xde05].copy_from_slice(&[0xff, 0, 0]);
        e.run_until(program.symbols["done"], 128)?;
        ensure!(
            [e.a, e.b, e.c, e.d] == [0; 4],
            "zero length compared invalid pointers"
        );
    }
    Ok(())
}

#[test]
fn rust_and_clif_memcmp_calls_use_selected_helpers_without_core() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let output = Command::new(env!("CARGO_BIN_EXE_sangoku"))
        .current_dir(harness::root())
        .args(["lower", "tests/fixtures/memcmp/libcall.clif", "-"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let clif = String::from_utf8(output.stdout)?;
    ensure!(clif.contains("call __rust_memcmp"));
    for level in ["0", "z"] {
        let path = directory.path().join(format!("native-{level}.asm"));
        let output = Command::new("rustc").current_dir(harness::root())
            .args(["+gbc-lr35902", "-Zunstable-options", "-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so", "--target", "targets/lr35902.json", "--emit=asm", "--crate-type=lib", "-Cpanic=immediate-abort"])
            .arg(format!("-Copt-level={level}"))
            .arg("tests/fixtures/memcmp/native.rs").arg("-o").arg(&path).output()?;
        ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let assembly = std::fs::read_to_string(path)?;
        let small = assembly
            .split("SECTION \"__rust_audit_equal_small\"")
            .nth(1)
            .ok_or("missing small equality")?
            .split("\nSECTION ")
            .next()
            .unwrap();
        ensure!(
            !small.contains("call __rust_memcmp"),
            "small equality stopped being inline"
        );
        let program = harness::assemble_tiny(
            &[
                ("native.asm", assembly),
                ("clif.asm", clif.clone()),
                (
                    "compare.asm",
                    harness::runtime(if level == "z" {
                        "memcmp_size"
                    } else {
                        "memcmp_opt"
                    })?,
                ),
                ("stop.asm", "SECTION \"stop\", ROM0\ndone::\nnop\n".into()),
            ],
            None,
        )?;
        for entry in [
            "__rust_audit_order",
            "__rust_audit_named",
            "__rust_audit_libcall",
        ] {
            for len in [0, 1, 4, 16, 257] {
                for (left, right) in cases(len).into_iter().take(3) {
                    ensure!(
                        run(&program, entry, &left, &right, 0)?.signum() == expected(&left, &right),
                        "{entry} failed at {level}/{len}"
                    );
                }
            }
        }
        for (entry, len) in [("__rust_audit_equal", 5), ("__rust_audit_equal_small", 2)] {
            for (left, right) in cases(len) {
                ensure!(
                    run(&program, entry, &left, &right, 0)? as u8 == u8::from(left == right),
                    "{entry} failed at {level}"
                );
            }
        }
    }
    Ok(())
}
