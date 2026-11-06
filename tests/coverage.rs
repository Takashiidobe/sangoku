use std::process::Command;

#[test]
fn lowering_coverage_matrix_is_current() {
    let output = Command::new("python3")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["tools/coverage_matrix.py", "--check"])
        .output()
        .expect("python3 is required");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
