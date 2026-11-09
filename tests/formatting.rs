use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

#[test]
fn upstream_u64_decimal_formatting_matches_host() -> Result<()> {
    let root = harness::root();
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let metadata = directory.path().join("libcore.rmeta");
    let assembly = directory.path().join("format-u64.asm");
    let compile = |source: &str, name: &str, output: &std::path::Path, core: bool| -> Result<()> {
        let mut command = Command::new("rustup");
        command
            .current_dir(&root)
            .args([
                "run",
                "gbc-lr35902",
                "rustc",
                "--edition=2024",
                "--crate-type=lib",
                "--target",
                "targets/lr35902.json",
                "-Zunstable-options",
                "-Cpanic=immediate-abort",
                "-Copt-level=z",
                "-Ccodegen-units=1",
                "-Zmir-opt-level=3",
                "-Zinline-mir=yes",
                "-Zcross-crate-inline-threshold=always",
                "--crate-name",
                name,
            ])
            .arg(source)
            .arg("-o")
            .arg(output);
        if core {
            command.args([
                "--emit=metadata",
                "-Zalways-encode-mir",
                "--cfg",
                "feature=\"optimize_for_size\"",
            ]);
        } else {
            command
                .args(["--emit=asm", "--extern"])
                .arg(format!("core={}", metadata.display()));
            command.arg(format!("-Zcodegen-backend={}", root.join("third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so").display()));
        }
        let result = command.output()?;
        ensure!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        Ok(())
    };
    compile(
        "third_party/rust/library/core/src/lib.rs",
        "core",
        &metadata,
        true,
    )?;
    compile(
        "tests/fixtures/core/format-u64.rs",
        "format_u64",
        &assembly,
        false,
    )?;
    let mut sources = vec![
        ("format.asm", std::fs::read_to_string(assembly)?),
        (
            "stop.asm",
            "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
        ),
    ];
    for name in ["wide32", "mul16", "memset_opt", "memmove_opt", "memcpy_opt"] {
        sources.push((name, harness::runtime(name)?));
    }
    let program = harness::assemble_tiny(&sources, None)?;
    let stop = program.symbols["done"];
    for value in [
        0u64,
        1,
        9,
        10,
        9999,
        10000,
        u32::MAX as u64 + 1,
        1u64 << 63,
        u64::MAX,
    ] {
        let mut e = program.emulator("__rust_format_u64")?;
        e.memory[0xc100..0xc108].copy_from_slice(&value.to_le_bytes());
        e.memory[0xc200..0xc250].fill(0xa5);
        e.b = 0;
        e.c = 0xc1;
        e.a = 0;
        e.memory[0xd000] = 0xc2;
        e.set_sp(0xcffe);
        e.write16(0xcffe, stop);
        e.run_until(stop, 20_000_000)
            .map_err(|error| std::io::Error::other(format!("{value}: {error}")))?;
        let len = usize::from(e.a) | usize::from(e.b) << 8;
        let expected = value.to_string();
        ensure!(
            len == expected.len(),
            "{value}: got length {len}, expected {}",
            expected.len()
        );
        ensure!(
            &e.memory[0xc200..0xc200 + len] == expected.as_bytes(),
            "{value}: wrong output"
        );
        ensure!(
            e.memory[0xc200 + len..0xc250]
                .iter()
                .all(|&byte| byte == 0xa5),
            "{value}: output overrun"
        );
    }
    let data_assembly = directory.path().join("data.asm");
    compile(
        "tests/fixtures/core/data-relocations.rs",
        "data_relocations",
        &data_assembly,
        false,
    )?;
    let data_program = harness::assemble(
        &[
            ("data.asm", std::fs::read_to_string(data_assembly)?),
            (
                "stop.asm",
                "SECTION \"reserved\", ROM0[$0000]\nds $100, 0\ndone::\nnop\n".into(),
            ),
        ],
        None,
    )?;
    for (index, expected) in [(0, 23), (1, 34), (2, 45)] {
        let mut e = data_program.emulator("__rust_read_record")?;
        e.b = index;
        e.c = 0;
        e.set_sp(0xcffe);
        e.write16(0xcffe, data_program.symbols["done"]);
        e.run_until(data_program.symbols["done"], 10000)?;
        ensure!(
            e.a == expected,
            "data relocation index {index}: got {}",
            e.a
        );
    }
    Ok(())
}
