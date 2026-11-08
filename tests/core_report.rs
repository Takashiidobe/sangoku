use sangoku_test_support::{Result, ensure};
use std::process::Command;

#[test]
fn core_lowers_to_lr35902_without_failures() -> Result<()> {
    let output = Command::new("python3")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["tools/core_report.py", "--json"])
        .output()?;
    ensure!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: sangoku_test_support::Value =
        sangoku_test_support::serde_json::from_slice(&output.stdout)?;
    let reports = &document["reports"];
    for report in reports.as_array().unwrap() {
        let label = &report["label"];
        ensure!(
            report["backend_errors"].as_array().unwrap().is_empty(),
            "{label}: {}",
            report["backend_errors"]
        );
        ensure!(report["failed"] == 0, "{label}: {}", report["groups"]);
    }
    let driver = &reports[2];
    ensure!(
        driver["functions"].as_u64().unwrap() >= 700,
        "driver instantiated only {} functions",
        driver["functions"]
    );
    ensure!(
        document["unmapped_libcalls"].as_array().unwrap().is_empty(),
        "unmapped backend libcalls: {}",
        document["unmapped_libcalls"]
    );
    let inventory = document["runtime_libcalls"].as_array().unwrap();
    ensure!(inventory.len() >= 34, "incomplete runtime inventory");
    for entry in inventory {
        ensure!(
            !entry["provider"].is_null(),
            "missing runtime definition: {entry}"
        );
    }
    let integer_libcalls: Vec<_> = document["undefined"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|name| {
            let name = name.as_str().unwrap();
            name.ends_with("ti3") || name.contains("mulo")
        })
        .collect();
    ensure!(
        integer_libcalls.is_empty(),
        "undefined integer libcalls: {integer_libcalls:?}"
    );
    Ok(())
}
