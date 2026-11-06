use std::process::Command;

#[test]
fn fixtures_match_filecheck_expectations() {
    let root = env!("CARGO_MANIFEST_DIR");
    let output = Command::new("python3")
        .current_dir(root)
        .arg("tools/update_filecheck.py")
        .arg("--check")
        .env("SANGOKU", env!("CARGO_BIN_EXE_sangoku"))
        .output()
        .expect("python3 and FileCheck are required");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
