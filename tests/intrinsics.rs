use std::process::Command;

#[test]
fn intrinsic_lowering_and_call_routes_are_current() {
    let output = Command::new("python3")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["tools/intrinsics_audit.py", "--check"])
        .output()
        .expect("python3 is required");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
