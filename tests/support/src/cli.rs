use crate::harness;
use crate::{Result, Value, ensure, json, number};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::ExitCode,
};
#[derive(Default)]
pub struct Args {
    pub values: BTreeMap<String, Vec<String>>,
}
impl Args {
    pub fn parse(args: &[String], values: &[&str], switches: &[&str]) -> Result<Self> {
        let mut result = Self::default();
        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            let (key, inline) = arg
                .split_once('=')
                .map_or((arg.as_str(), None), |(k, v)| (k, Some(v)));
            ensure!(key.starts_with("--"), "unexpected argument {arg}");
            let key = &key[2..];
            if switches.contains(&key) {
                ensure!(inline.is_none(), "--{key} takes no value");
                result
                    .values
                    .entry(key.into())
                    .or_default()
                    .push("true".into());
            } else {
                ensure!(values.contains(&key), "unknown option --{key}");
                let value = if let Some(v) = inline {
                    v.to_owned()
                } else {
                    i += 1;
                    args.get(i)
                        .ok_or_else(|| anyhow::anyhow!("missing value for --{key}"))?
                        .clone()
                };
                result.values.entry(key.into()).or_default().push(value);
            }
            i += 1;
        }
        Ok(result)
    }
    pub fn get(&self, k: &str) -> Option<&str> {
        self.values
            .get(k)
            .and_then(|v| v.last())
            .map(String::as_str)
    }
    pub fn many(&self, k: &str, default: &[&str]) -> Vec<String> {
        self.values
            .get(k)
            .cloned()
            .unwrap_or_else(|| default.iter().map(|x| (*x).into()).collect())
    }
    pub fn has(&self, k: &str) -> bool {
        self.values.contains_key(k)
    }
    pub fn integer(&self, k: &str, default: u64) -> Result<u64> {
        self.get(k).map_or(Ok(default), parse_number)
    }
    pub fn path(&self, k: &str, default: &str) -> PathBuf {
        PathBuf::from(self.get(k).unwrap_or(default))
    }
}
pub fn parse_number(s: &str) -> Result<u64> {
    let s = s.strip_prefix('+').unwrap_or(s);
    for (prefix, base) in [
        ("0x", 16),
        ("0X", 16),
        ("0b", 2),
        ("0B", 2),
        ("0o", 8),
        ("0O", 8),
    ] {
        if let Some(digits) = s.strip_prefix(prefix) {
            return Ok(u64::from_str_radix(digits, base)?);
        }
    }
    Ok(s.parse()?)
}
pub fn statistics(rows: &[Value]) -> Value {
    let mut cycles: Vec<_> = rows.iter().map(|r| number(&r["cycles"])).collect();
    cycles.sort_unstable();
    let n = cycles.len();
    let sum: u64 = cycles.iter().sum();
    let median = if n % 2 == 1 {
        json!(cycles[n / 2])
    } else {
        json!((cycles[n / 2 - 1] + cycles[n / 2]) as f64 / 2.0)
    };
    let mean = if sum.is_multiple_of(n as u64) {
        json!(sum / n as u64)
    } else {
        json!(sum as f64 / n as f64)
    };
    json!({
        "count":n,
        "min_cycles":cycles[0],
        "median_cycles":median,
        "max_cycles":cycles[n-1],
        "mean_cycles":mean,
        "peak_stack_bytes":rows.iter().map(|r|number(&r["peak_stack_bytes"])).max().unwrap()
    })
}
pub fn write_csv(report: &Value, path: &Path, division: bool) -> Result<()> {
    let fields = if division {
        vec![
            "variant",
            "code_bytes",
            "quotient_remainder_code_bytes",
            "class",
            "operation",
            "mode",
            "count",
            "min_cycles",
            "median_cycles",
            "max_cycles",
            "mean_cycles",
            "peak_stack_bytes",
        ]
    } else {
        vec![
            "variant",
            "code_bytes",
            "table_bytes",
            "class",
            "mode",
            "count",
            "min_cycles",
            "median_cycles",
            "max_cycles",
            "mean_cycles",
            "peak_stack_bytes",
        ]
    };
    let mut text = fields.join(",") + "\n";
    for candidate in report["results"].as_array().unwrap() {
        for row in candidate["summary"].as_array().unwrap() {
            let values: Vec<_> = fields
                .iter()
                .map(|key| {
                    let v = if row.get(*key).is_some() {
                        &row[*key]
                    } else {
                        &candidate[*key]
                    };
                    let s = if let Some(s) = v.as_str() {
                        s.to_owned()
                    } else {
                        v.to_string()
                    };
                    if s.contains([',', '"', '\n']) {
                        format!("\"{}\"", s.replace('"', "\"\""))
                    } else {
                        s
                    }
                })
                .collect();
            text += &(values.join(",") + "\n");
        }
    }
    harness::write(path, text)
}
pub fn successful(report: &Value) -> bool {
    if let Some(rows) = report["reports"].as_array() {
        rows.iter().all(|r| {
            r["measurements"]
                .as_object()
                .unwrap()
                .values()
                .all(|m| m["correct"] == true)
        })
    } else {
        report["results"].as_array().is_some_and(|rows| {
            rows.iter().all(|r| {
                r["error_count"] == 0
                    && r.get("exhaustive8")
                        .or_else(|| r.get("exhaustive16"))
                        .is_none_or(|v| v["correct"] == true)
                    && r.get("zero_divisor")
                        .is_none_or(|v| v.as_array().unwrap().iter().all(|t| t["correct"] == true))
            })
        })
    }
}
pub fn finish_runtime(report: &Value, output: Option<&Path>) -> Result<()> {
    if let Some(path) = output {
        harness::write_json(path, report)?;
    } else {
        println!("{}", serde_json::to_string_pretty(report)?);
    }
    ensure!(successful(report), "runtime audit found incorrect results");
    Ok(())
}
pub fn run_args(tool: &str, args: &[String]) -> Result<()> {
    if tool == "audit-shifts" {
        return crate::shifts::main(args);
    }
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        let flags = match tool {
            "audit-runtime" => {
                "[--helper mul16|udiv16|urem16|stack-fixture] [--lhs N --rhs N] [--output FILE] [--artifacts DIR]"
            }
            "audit-division" => {
                "[--variant NAME] [--samples N] [--seed N] [--exhaustive8] [--output FILE] [--artifacts DIR] [--csv FILE]"
            }
            "audit-popcount" => {
                "[--variant NAME] [--samples N] [--seed N] [--exhaustive16] [--output FILE] [--artifacts DIR] [--csv FILE]"
            }
            "generate-popcount-baselines" => "--sdcc PATH [--output DIR]",
            _ => "[--skip-build] [--exhaustive16] [--output DIR]",
        };
        println!("cargo run --example {tool} -- {flags}");
        return Ok(());
    }
    if [
        "check-spills",
        "check-popcount",
        "generate-popcount-baselines",
    ]
    .contains(&tool)
    {
        return crate::compiler::run(tool, args);
    }
    if tool == "audit-runtime" {
        let a = Args::parse(args, &["helper", "lhs", "rhs", "output", "artifacts"], &[])?;
        ensure!(
            a.has("lhs") == a.has("rhs"),
            "--lhs and --rhs must be supplied together"
        );
        let pairs = if a.has("lhs") {
            let lhs = a.integer("lhs", 0)?;
            let rhs = a.integer("rhs", 0)?;
            ensure!(
                lhs <= 65535 && rhs <= 65535,
                "operands must be unsigned 16-bit values"
            );
            vec![(lhs as u16, rhs as u16)]
        } else {
            vec![(0, 1), (1, 1), (0x1234, 16), (65535, 255), (65535, 32768)]
        };
        let mut rows = vec![];
        for helper in a.many("helper", &["mul16", "stack-fixture"]) {
            for (index, &(lhs, rhs)) in pairs.iter().enumerate() {
                let artifacts = a
                    .get("artifacts")
                    .map(|p| Path::new(p).join(&helper).join(index.to_string()));
                rows.push(crate::runtime::measure(
                    &helper,
                    lhs,
                    rhs,
                    artifacts.as_deref(),
                )?);
            }
        }
        let report = json!({
            "timing_unit":"CPU T-cycles",
            "limitations":["flat ROM0/WRAM0 memory; instruction subset; no peripherals or interrupts","synthetic ABI callers, not compiler-emitted callers","body entry includes a 2-byte return address and a 2-byte argument slot","scratch RAM is linked WRAM0 allocation; writes are counted separately","observed changes do not establish a callee-saved register contract"],
            "reports":rows
        });
        return finish_runtime(&report, a.get("output").map(Path::new));
    }
    ensure!(
        tool == "audit-division" || tool == "audit-popcount",
        "unknown tool {tool}"
    );
    let a = Args::parse(
        args,
        &["variant", "samples", "seed", "artifacts", "output", "csv"],
        if tool == "audit-division" {
            &["exhaustive8"]
        } else {
            &["exhaustive16"]
        },
    )?;
    let samples = a.integer("samples", 256)? as usize;
    ensure!(samples > 0, "--samples must be positive");
    let seed = a
        .get("seed")
        .map(|s| s.strip_prefix('-').unwrap_or(s))
        .map_or(Ok(0), parse_number)?;
    let division = tool == "audit-division";
    let variants = a.many(
        "variant",
        if division {
            crate::division::VARIANTS
        } else {
            crate::popcount::VARIANTS
        },
    );
    ensure!(
        variants.iter().all(|v| if division {
            crate::division::VARIANTS.contains(&v.as_str()) || v == "runtime"
        } else {
            crate::popcount::VARIANTS.contains(&v.as_str())
        }),
        "unknown audit variant"
    );
    let report = if division {
        crate::division::audit(
            &variants,
            samples,
            seed,
            a.has("exhaustive8"),
            &a.path("artifacts", "build/division-audit"),
        )?
    } else {
        crate::popcount::audit(
            &variants,
            samples,
            seed,
            a.has("exhaustive16"),
            &a.path("artifacts", "build/popcount-audit/artifacts"),
        )?
    };
    harness::write_json(
        &a.path(
            "output",
            if division {
                "build/division-audit.json"
            } else {
                "build/popcount-audit/results.json"
            },
        ),
        &report,
    )?;
    if let Some(path) = a.get("csv") {
        write_csv(&report, Path::new(path), division)?;
    }
    ensure!(successful(&report), "audit found incorrect results");
    Ok(())
}
pub fn main(tool: &str) -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match run_args(tool, &args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{tool}: {e:#}");
            ExitCode::FAILURE
        }
    }
}
