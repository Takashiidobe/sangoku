use sangoku_test_support::{self as support, Result, json, number};
use std::{collections::BTreeMap, sync::OnceLock};
use support::{division as d, harness as h, popcount as p, random::Random, runtime as r};
fn good(v: &support::Value) {
    assert_eq!(v["correct"], true, "{v}");
}
fn divs() -> &'static BTreeMap<&'static str, h::Program> {
    static P: OnceLock<BTreeMap<&str, h::Program>> = OnceLock::new();
    P.get_or_init(|| {
        d::VARIANTS
            .iter()
            .map(|&v| (v, d::build(v, None).unwrap()))
            .collect()
    })
}
fn pops() -> &'static BTreeMap<&'static str, h::Program> {
    static P: OnceLock<BTreeMap<&str, h::Program>> = OnceLock::new();
    P.get_or_init(|| {
        p::VARIANTS
            .iter()
            .map(|&v| (v, p::build(v, None).unwrap()))
            .collect()
    })
}
fn runtime() -> &'static h::Program {
    static P: OnceLock<h::Program> = OnceLock::new();
    P.get_or_init(|| d::build("runtime", None).unwrap())
}

mod multiply {
    use super::*;
    fn product(lhs: u16, rhs: u16) -> (u16, u64) {
        static P: OnceLock<h::Program> = OnceLock::new();
        let p = P.get_or_init(|| {
            h::assemble(
                &[(
                    "multiply.asm",
                    h::read("runtime/integer/mul/size/i16.asm")
                        .unwrap()
                        .replacen("ROM0", "ROM0[$100]", 1),
                )],
                None,
            )
            .unwrap()
        });
        let mut e = support::emu::Emulator::new(&p.rom, 0x100).unwrap();
        e.b = lhs as u8;
        e.c = (lhs >> 8) as u8;
        e.a = rhs as u8;
        e.d = 0x55;
        e.e = 0xAA;
        e.set_hl(0x1234);
        e.f = 0xF0;
        e.set_sp(0xD000);
        e.write16(e.sp(), 0x150);
        e.memory[e.sp() as usize + 2] = (rhs >> 8) as u8;
        let ticks = e.run_until(0x150, 1000).unwrap();
        assert_eq!(e.sp(), 0xD002);
        assert_eq!(e.memory[0xD002], (rhs >> 8) as u8);
        let result = e.a as u16 | (e.b as u16) << 8;
        assert_eq!(result, lhs.wrapping_mul(rhs));
        (result, ticks)
    }
    #[test]
    fn boundaries_and_byte_order() {
        let values = [
            0, 1, 2, 127, 128, 255, 256, 257, 0x1234, 0x5678, 32767, 32768, 65535, -1, -2, -32768,
        ];
        for a in values {
            for b in values {
                product(a as u16, b as u16);
            }
        }
    }
    #[test]
    fn random_products() {
        let mut rng = Random::new(0);
        for _ in 0..10000 {
            product(rng.range(0, 65536), rng.range(0, 65536));
        }
    }
    #[test]
    fn shortcut_timing() {
        for b in [0u16, 1, 255, 256, 65535] {
            let (_, t) = product(0x1234, b);
            assert_eq!(
                t,
                if b < 256 { 480 } else { 848 } + 4 * b.count_ones() as u64
            );
        }
    }
}
mod runtime_division {
    use super::*;
    #[test]
    fn public_entries_against_host_divmod() -> Result<()> {
        for (_, pairs) in d::input_classes(128, 2) {
            for (a, b) in pairs {
                for op in ["q", "r"] {
                    for body in [true, false] {
                        let v = d::run_case(runtime(), op, a, b, body, 0x55)?;
                        good(&v);
                        assert_eq!(v["peak_stack_bytes"], 6);
                    }
                }
                let v = d::run_case(runtime(), "separate", a, b, false, 0x55)?;
                good(&v);
                assert_eq!(v["peak_stack_bytes"], 8);
            }
        }
        Ok(())
    }
    #[test]
    fn entries_ignore_scratch_state() -> Result<()> {
        for poison in [0, 0xA5, 255] {
            for (a, b) in [
                (1, 1),
                (0, 65535),
                (65535, 127),
                (65535, 128),
                (65535, 32768),
            ] {
                for op in ["q", "r"] {
                    good(&d::run_case(runtime(), op, a, b, true, poison)?);
                }
            }
        }
        Ok(())
    }
    #[test]
    fn zero_divisors_reach_rst_vector() -> Result<()> {
        for a in [0, 1, 32768, 65535] {
            for op in ["q", "r"] {
                for body in [true, false] {
                    let v = d::run_case(runtime(), op, a, 0, body, 0x55)?;
                    good(&v);
                    assert_eq!(v["status"], "rst_10");
                    assert_eq!(v["peak_stack_bytes"], 8);
                }
            }
        }
        Ok(())
    }
    #[test]
    fn runtime_costs() -> Result<()> {
        assert_eq!(
            runtime().size(
                "ROM0",
                &[
                    "Audit division",
                    "Unsigned 16-bit remainder",
                    "Unsigned 16-bit division core"
                ]
            ),
            95
        );
        assert!(!runtime().symbols.contains_key("audit_both"));
        for (op, cost) in [("q", 1236), ("r", 1240)] {
            let v = d::run_case(runtime(), op, 1, 1, false, 0x55)?;
            good(&v);
            assert_eq!(v["cycles"], cost);
        }
        Ok(())
    }
}
mod division_audit {
    use super::*;
    #[test]
    fn variants_against_host_divmod() -> Result<()> {
        let mut pairs: Vec<_> = d::BOUNDARIES
            .iter()
            .flat_map(|&a| {
                d::BOUNDARIES
                    .iter()
                    .filter(|&&b| b != 0)
                    .map(move |&b| (a, b))
            })
            .collect();
        let mut rng = Random::new(1);
        pairs.extend((0..128).map(|_| (rng.range(0, 65536), rng.range(1, 65536))));
        for p in divs().values() {
            for &(a, b) in &pairs {
                for &op in d::OPERATIONS {
                    good(&d::run_case(p, op, a, b, false, 0x55)?);
                }
                good(&d::run_case(p, "both", a, b, true, 0x55)?);
            }
        }
        Ok(())
    }
    #[test]
    fn wrappers_ignore_scratch_register_state() -> Result<()> {
        for p in divs().values() {
            for poison in [0, 0xA5, 255] {
                for op in ["q", "r", "both"] {
                    good(&d::run_case(p, op, 0xFEDC, 129, true, poison)?);
                }
            }
        }
        Ok(())
    }
    #[test]
    fn zero_divisors_trap_for_every_entry() -> Result<()> {
        for p in divs().values() {
            for a in [0, 1, 32768, 65535] {
                for &op in d::OPERATIONS {
                    let r = d::run_case(p, op, a, 0, false, 0x55)?;
                    good(&r);
                    assert_eq!(r["status"], "rst_10");
                }
            }
        }
        Ok(())
    }
    #[test]
    fn shared_divmod_avoids_second_division() -> Result<()> {
        for p in divs().values() {
            let a = d::run_case(p, "both", 65535, 127, false, 0x55)?;
            let b = d::run_case(p, "separate", 65535, 127, false, 0x55)?;
            assert_eq!(a["actual"], b["actual"]);
            assert!(number(&a["cycles"]) < number(&b["cycles"]));
            assert!(number(&a["peak_stack_bytes"]) < number(&b["peak_stack_bytes"]));
            assert_eq!(p.size("WRAM0", &[]), 4);
        }
        Ok(())
    }
    #[test]
    fn frozen_baseline_defects() -> Result<()> {
        let p = d::build("raw", None)?;
        let r = d::run_case(&p, "q", 1, 1, true, 0x55)?;
        assert_eq!(r["correct"], false);
        assert_eq!(r["actual"], 256);
        assert!(
            d::build("raw-alt", None)
                .unwrap_err()
                .to_string()
                .contains("__gbc_udiv16.divide")
        );
        Ok(())
    }
    #[test]
    fn quotient_remainder_only_link_size() -> Result<()> {
        for (v, p) in divs() {
            assert_eq!(
                p.size("ROM0", &["Audit division"]) - d::quotient_remainder_size(v, None)?,
                14
            );
        }
        assert_eq!(d::quotient_remainder_size("sdcc-z80", None)?, 95);
        Ok(())
    }
    #[test]
    fn input_distribution_is_reproducible() {
        let a = d::input_classes(10, 0);
        assert_eq!(a, d::input_classes(10, 0));
        assert_ne!(a, d::input_classes(10, 1));
        for (name, pairs) in a {
            match name.as_str() {
                "less-than" => assert!(pairs.iter().all(|(a, b)| a < b)),
                "narrow-random" => assert!(pairs.iter().all(|&(_, b)| b > 0 && b < 128)),
                "wide-random" => assert!(pairs.iter().all(|&(_, b)| b >= 128)),
                _ => {}
            }
        }
    }
}
mod popcount_audit {
    use super::*;
    #[test]
    fn candidates_against_host_bit_count() -> Result<()> {
        let mut rng = Random::new(1);
        let mut values: std::collections::BTreeSet<u16> = (0..256)
            .chain((0..256).map(|v| v << 8))
            .chain([65535, 0x5555, 0xAAAA, 0x8001, 0x1234])
            .collect();
        values.extend((0..128).map(|_| rng.range(0, 65536)));
        for (v, program) in pops() {
            for &value in &values {
                for &mode in p::modes_for(v) {
                    good(&p::run_case(program, v, value, mode, 0x55, 0xD000)?);
                }
            }
        }
        Ok(())
    }
    #[test]
    fn frozen_compiler_spill_defect_depends_on_stack() -> Result<()> {
        let p = p::build("raw", None)?;
        let a = p::run_case(&p, "raw", 0, "body", 0x55, 0xD000)?;
        let b = p::run_case(&p, "raw", 0, "body", 0x55, 0xD002)?;
        assert_eq!(p.size("ROM0", &["Audit helper"]), 207);
        assert_eq!(a["correct"], false);
        assert_eq!(b["correct"], false);
        assert_ne!(a["actual"], b["actual"]);
        Ok(())
    }
    #[test]
    fn scratch_and_stack_state_independence() -> Result<()> {
        for (v, p) in pops() {
            for poison in [0, 0xA5, 255] {
                for stack in [0xC900, 0xD002] {
                    good(&p::run_case(p, v, 0x8001, "body", poison, stack)?);
                }
            }
        }
        Ok(())
    }
    #[test]
    fn tables_and_alignment() {
        for (v, size, alignment) in [
            ("nibble", 16, 16),
            ("nibble-page", 16, 256),
            ("byte", 256, 256),
        ] {
            let p = &pops()[v];
            let address = p.symbols["audit_table"] as usize;
            assert_eq!(address % alignment, 0);
            assert_eq!(p.size("ROM0", &["Audit table"]), size);
            assert_eq!(
                &p.rom[address..address + size],
                (0..size).map(|v| v.count_ones() as u8).collect::<Vec<_>>()
            );
        }
    }
    #[test]
    fn inline_and_call_overhead() -> Result<()> {
        for (v, p) in pops() {
            if !p::modes_for(v).contains(&"inline") {
                continue;
            }
            let a = p::run_case(p, v, 0x1234, "call", 0x55, 0xD000)?;
            let b = p::run_case(p, v, 0x1234, "inline", 0x55, 0xD000)?;
            assert_eq!(a["actual"], b["actual"]);
            assert_eq!(number(&a["cycles"]) - number(&b["cycles"]), 40);
        }
        let p = &pops()["swar"];
        let a = p::run_case(p, "swar", 65535, "call-live", 0x55, 0xD000)?;
        let b = p::run_case(p, "swar", 65535, "inline-live", 0x55, 0xD000)?;
        assert_eq!(a["actual"], 15);
        assert_eq!(b["actual"], 15);
        assert_eq!(a["peak_stack_bytes"], 4);
        assert_eq!(b["peak_stack_bytes"], 0);
        Ok(())
    }
    #[test]
    fn fixed_cost_candidates() -> Result<()> {
        for (v, size, cost) in [
            ("swar", 51, 216),
            ("nibble-page", 34, 164),
            ("byte", 10, 60),
        ] {
            let p = &pops()[v];
            assert_eq!(p.size("ROM0", &["Audit helper"]), size);
            for value in [0, 1, 65535] {
                assert_eq!(
                    p::run_case(p, v, value, "body", 0x55, 0xD000)?["cycles"],
                    cost
                );
            }
        }
        Ok(())
    }
    #[test]
    fn input_distributions() {
        let a = p::input_classes(128, 0);
        assert_eq!(a, p::input_classes(128, 0));
        assert_ne!(a, p::input_classes(128, 1));
        for (name, values) in a {
            match name.as_str() {
                "all-8-bit" => assert_eq!(values, (0..256).collect::<Vec<_>>()),
                "sparse" => assert!(values.iter().all(|v| v.count_ones() <= 2)),
                "dense" => assert!(values.iter().all(|v| v.count_ones() >= 14)),
                _ => {}
            }
        }
    }
}
mod harness {
    use super::*;
    #[test]
    fn map_sizes_include_zero_table_bytes_and_exclude_padding() -> Result<()> {
        let p=h::assemble(&[("sizes.asm","SECTION \"code\", ROM0[$100]\nentry::\n    ret\nSECTION \"table\", ROM0[$400]\n    db 0, 0\nSECTION \"scratch\", WRAM0[$c000]\n    ds 7\n".into())],None)?;
        assert_eq!(p.symbols["entry"], 0x100);
        assert_eq!(p.size("ROM0", &[]), 3);
        assert_eq!(p.size("ROM0", &["table"]), 2);
        assert_eq!(p.size("WRAM0", &[]), 7);
        assert!(p.provenance["artifacts"].is_null());
        Ok(())
    }
    #[test]
    fn mul16_body_and_complete_caller() -> Result<()> {
        let r = r::measure("mul16", 1, 1, None)?;
        assert_eq!(
            r["sizes"],
            json!({"helper_code_bytes":30,"table_rom_bytes":0,"caller_code_bytes":18,"scratch_ram_bytes":0})
        );
        for (mode, cycles, entry, extra) in [("body", 484, 4, 0), ("caller", 592, 0, 4)] {
            let r = &r["measurements"][mode];
            good(r);
            assert_eq!(r["cycles"], cycles);
            assert_eq!(r["entry_stack_bytes"], entry);
            assert_eq!(r["peak_additional_stack_bytes"], extra);
            assert_eq!(r["peak_stack_bytes"], 4);
            assert_eq!(r["scratch_written_bytes"], 0);
        }
        Ok(())
    }
    #[test]
    fn nested_stack_fixture_and_scratch_accounting() -> Result<()> {
        let r = r::measure("stack-fixture", 1, 1, None)?;
        assert_eq!(
            r["sizes"],
            json!({"helper_code_bytes":24,"table_rom_bytes":2,"caller_code_bytes":18,"scratch_ram_bytes":4})
        );
        for (mode, cycles, entry, extra) in [("body", 180, 4, 6), ("caller", 288, 0, 10)] {
            let r = &r["measurements"][mode];
            good(r);
            assert_eq!(r["cycles"], cycles);
            assert_eq!(r["entry_stack_bytes"], entry);
            assert_eq!(r["peak_additional_stack_bytes"], extra);
            assert_eq!(r["peak_stack_bytes"], 10);
            assert_eq!(r["scratch_written_bytes"], 1);
            assert_eq!(r["registers"]["a"], 42);
        }
        Ok(())
    }
    #[test]
    fn division_zero_trap_report() -> Result<()> {
        let r = r::measure("udiv16", 1, 0, None)?;
        for v in r["measurements"].as_object().unwrap().values() {
            good(v);
            assert_eq!(v["status"], "rst_10");
            assert_eq!(v["peak_stack_bytes"], 8);
        }
        Ok(())
    }
    #[test]
    fn retained_artifacts_and_incorrect_stack() -> Result<()> {
        let dir = support::tempfile::tempdir()?;
        let output = dir.path().join("report.json");
        let args = vec![
            "--helper".into(),
            "mul16".into(),
            "--lhs".into(),
            "1".into(),
            "--rhs".into(),
            "1".into(),
            "--output".into(),
            output.display().to_string(),
            "--artifacts".into(),
            dir.path().join("artifacts").display().to_string(),
        ];
        support::cli::run_args("audit-runtime", &args)?;
        let report: support::Value =
            support::serde_json::from_str(&std::fs::read_to_string(output)?)?;
        let p = std::path::Path::new(
            report["reports"][0]["provenance"]["artifacts"]
                .as_str()
                .unwrap(),
        );
        for name in [
            "helper.asm",
            "caller.asm",
            "runtime.gb",
            "runtime.map",
            "runtime.sym",
        ] {
            assert!(p.join(name).is_file());
        }
        assert_eq!(
            report["reports"][0]["provenance"]["source_sha256"]
                .as_object()
                .unwrap()
                .len(),
            2
        );
        let p=h::assemble(&[("incorrect.asm","SECTION \"Audit helper\", ROM0[$200]\nbad::\n    inc sp\n    ret\nSECTION \"Audit caller\", ROM0[$100]\naudit_done::\n    nop\n".into())],None)?;
        let result = r::execute(&p, "bad", 1, 1, true, "audit_done", |_| {})?;
        assert_eq!(result["stack_restored"], false);
        Ok(())
    }
    #[test]
    fn cli_fails_on_incorrect_output() -> Result<()> {
        let report = r::measure_using("mul16", 1, 1, None, |p, entry, lhs, rhs, body| {
            let mut result = r::execute(p, entry, lhs, rhs, body, "audit_done", |_| {})?;
            result["registers"]["a"] = json!(number(&result["registers"]["a"]) ^ 1);
            Ok(result)
        })?;
        let report = json!({"reports":[report]});
        let dir = support::tempfile::tempdir()?;
        let output = dir.path().join("bad.json");
        assert!(support::cli::finish_runtime(&report, Some(&output)).is_err());
        let saved: support::Value =
            support::serde_json::from_str(&std::fs::read_to_string(output)?)?;
        assert!(
            saved["reports"][0]["measurements"]
                .as_object()
                .unwrap()
                .values()
                .all(|r| r["correct"] == false)
        );
        Ok(())
    }
}
#[test]
fn assembly_rejects_banked_regions_and_path_escape() {
    assert!(h::assemble(&[("../escape.asm", "nop".into())], None).is_err());
    assert!(
        h::assemble(
            &[(
                "bank.asm",
                "SECTION \"bank\", ROMX,BANK[1]\nentry::\n ret\n".into()
            )],
            None
        )
        .is_err()
    );
}
#[test]
fn random_sequences_match_historical_python_seeds() -> Result<()> {
    let vectors: support::Value =
        support::serde_json::from_str(include_str!("fixtures/random.json"))?;
    for v in vectors.as_array().unwrap() {
        let mut rng = Random::new(number(&v["seed"]));
        for expected in v["words"].as_array().unwrap() {
            assert_eq!(rng.range(0, 65536) as u64, number(expected));
        }
        for expected in v["narrow"].as_array().unwrap() {
            assert_eq!(rng.range(1, 128) as u64, number(expected));
        }
        for i in 0..2048 {
            let value = rng.bits(32);
            if let Some(expected) = v["bit_checkpoints"].get(i.to_string()) {
                assert_eq!(value as u64, number(expected));
            }
        }
    }
    Ok(())
}
#[test]
fn audit_cli_rejects_invalid_inputs() {
    for (tool, args) in [
        ("audit-runtime", vec!["--lhs", "1"]),
        ("audit-runtime", vec!["--lhs", "65536", "--rhs", "1"]),
        ("audit-division", vec!["--samples", "0"]),
        ("audit-division", vec!["--variant", "invalid"]),
        ("audit-popcount", vec!["--exhaustive8"]),
        ("check-spills", vec!["--unknown"]),
        ("generate-popcount-baselines", vec![]),
    ] {
        assert!(
            support::cli::run_args(
                tool,
                &args.into_iter().map(str::to_owned).collect::<Vec<_>>()
            )
            .is_err()
        );
    }
}
