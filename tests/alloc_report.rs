use sangoku_test_support::{Result, ensure};
use std::process::Command;

#[test]
fn alloc_lowers_to_lr35902_without_failures() -> Result<()> {
    let output = Command::new("python3")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["tools/core_report.py", "--alloc", "--json"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: sangoku_test_support::Value =
        sangoku_test_support::serde_json::from_slice(&output.stdout)?;
    let reports = document["reports"].as_array().unwrap();
    ensure!(reports.len() == 5, "missing alloc reports: {document}");
    for report in reports {
        ensure!(
            report["backend_errors"].as_array().unwrap().is_empty(),
            "{}: {}",
            report["label"],
            report["backend_errors"]
        );
        ensure!(report["failed"] == 0, "{report}");
    }
    for (report, label, minimum) in [
        (&reports[3], "alloc", 1000),
        (&reports[4], "alloc generics (driver)", 1000),
    ] {
        ensure!(report["label"] == label, "{report}");
        ensure!(report["returncode"] == 0, "{report}");
        ensure!(
            report["functions"].as_u64().unwrap() >= minimum,
            "incomplete coverage: {report}"
        );
    }
    Ok(())
}
