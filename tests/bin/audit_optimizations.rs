use sangoku_test_support::{Result, cli::Args, optimization};

fn main() -> Result<()> {
    let args = Args::parse(
        &std::env::args().skip(1).collect::<Vec<_>>(),
        &["output", "baseline", "case"],
        &[],
    )?;
    optimization::run(
        &args.path("output", "build/optimizations/current"),
        args.get("baseline").map(std::path::Path::new),
        &args.many("case", optimization::CASES),
    )?;
    Ok(())
}
