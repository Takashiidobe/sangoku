use sangoku_test_support::{Result, ensure, harness};
use std::process::Command;

#[test]
fn target_core_preserves_upstream_api_surface() -> Result<()> {
    let root = harness::root();
    let directory = sangoku_test_support::tempfile::tempdir()?;
    let metadata = directory.path().join("libcore.rmeta");
    let compile = |source: &str, name: &str, output: &std::path::Path, core: bool| -> Result<()> {
        let mut command = Command::new("rustup");
        command
            .current_dir(&root)
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
            .arg(output);
        if !core {
            command
                .arg("--extern")
                .arg(format!("core={}", metadata.display()));
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
        "tests/fixtures/core/restored-apis.rs",
        "restored_apis",
        &directory.path().join("apis.rmeta"),
        false,
    )
}
