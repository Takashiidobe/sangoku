use sangoku_test_support::{Result, optimization};
use std::{env, path::PathBuf};

#[test]
fn selected_optimization_cases_match_host_and_report_size() -> Result<()> {
    let output = env::var_os("SANGOKU_OPT_OUTPUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("build/optimizations/current"));
    let baseline = env::var_os("SANGOKU_OPT_BASELINE").map(PathBuf::from);
    let cases: Vec<String> = env::var("SANGOKU_OPT_CASES")
        .map(|s| s.split(',').map(str::to_owned).collect())
        .unwrap_or_else(|_| {
            optimization::CASES
                .iter()
                .map(|s| (*s).to_owned())
                .collect()
        });
    optimization::run(&output, baseline.as_deref(), &cases)?;
    Ok(())
}
