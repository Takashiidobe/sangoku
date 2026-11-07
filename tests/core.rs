use sangoku_test_support::{Result, ensure, harness};
use std::path::Path;
use std::process::{Command, Output};

fn compile(
    source: &str,
    name: &str,
    output: &Path,
    dependencies: &[(&str, &Path)],
    flags: &[&str],
) -> Result<Output> {
    let mut command = Command::new("rustup");
    command
        .current_dir(harness::root())
        .args([
            "run",
            "gbc-lr35902",
            "rustc",
            "--edition=2024",
            "--crate-type=rlib",
            "--target",
            "targets/lr35902.json",
            "-Zunstable-options",
            "-Cpanic=abort",
            "--emit=metadata",
            "--crate-name",
            name,
        ])
        .arg(source)
        .arg("-o")
        .arg(output)
        .arg("-L")
        .arg(format!("dependency={}", output.parent().unwrap().display()))
        .args(flags);
    for (name, path) in dependencies {
        command
            .arg("--extern")
            .arg(format!("{name}={}", path.display()));
    }
    Ok(command.output()?)
}

fn success(output: Output) -> Result<()> {
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

#[test]
fn target_core_preserves_upstream_api_surface() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let metadata = directory.path().join("libcore.rmeta");
    success(compile(
        "third_party/rust/library/core/src/lib.rs",
        "core",
        &metadata,
        &[],
        &[],
    )?)?;
    success(compile(
        "tests/fixtures/core/restored-apis.rs",
        "restored_apis",
        &directory.path().join("apis.rmeta"),
        &[("core", &metadata)],
        &[],
    )?)
}

#[test]
fn target_core_and_alloc_respect_16_bit_size_limits() -> Result<()> {
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let core = directory.path().join("libcore.rmeta");
    let builtins = directory.path().join("libcompiler_builtins.rmeta");
    let alloc = directory.path().join("liballoc.rmeta");
    success(compile(
        "third_party/rust/library/core/src/lib.rs",
        "core",
        &core,
        &[],
        &[],
    )?)?;
    success(compile(
        "third_party/rust/library/compiler-builtins/compiler-builtins/src/lib.rs",
        "compiler_builtins",
        &builtins,
        &[("core", &core)],
        &["--cfg", "feature=\"compiler-builtins\""],
    )?)?;
    success(compile(
        "third_party/rust/library/alloc/src/lib.rs",
        "alloc",
        &alloc,
        &[("core", &core), ("compiler_builtins", &builtins)],
        &[],
    )?)?;
    success(compile(
        "tests/fixtures/core/pointer-width.rs",
        "pointer_width",
        &directory.path().join("limits.rmeta"),
        &[("core", &core), ("alloc", &alloc)],
        &[],
    )?)?;
    for (source, expected) in [
        ("oversized-array", "too big for the target architecture"),
        ("oversized-usize", "out of range for `usize`"),
    ] {
        let output = compile(
            &format!("tests/fixtures/core/{source}.rs"),
            "oversized",
            &directory.path().join("oversized.rmeta"),
            &[("core", &core)],
            &[],
        )?;
        ensure!(!output.status.success(), "{source} unexpectedly compiled");
        ensure!(
            String::from_utf8_lossy(&output.stderr).contains(expected),
            "{source}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn wide_integer_libcall_returns_lower_through_the_backend() -> Result<()> {
    let root = harness::root();
    success(
        Command::new("cargo")
            .current_dir(&root)
            .args([
                "+gbc-lr35902",
                "build",
                "--release",
                "--locked",
                "--manifest-path",
            ])
            .arg(root.join("third_party/rustc_codegen_cranelift/Cargo.toml"))
            .output()?,
    )?;
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let output = Command::new("rustup")
        .current_dir(&root)
        .args([
            "run",
            "gbc-lr35902",
            "rustc",
            "--edition=2024",
            "--crate-type=lib",
        ])
        .args([
            "--target",
            "targets/lr35902.json",
            "-Zunstable-options",
            "-Cpanic=abort",
        ])
        .arg(format!(
            "-Zcodegen-backend={}",
            root.join(
                "third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so"
            )
            .display()
        ))
        .args(["--emit=asm", "tests/fixtures/core/wide-return.rs", "-o"])
        .arg(directory.path().join("wide-return.s"))
        .output()?;
    success(output)
}
