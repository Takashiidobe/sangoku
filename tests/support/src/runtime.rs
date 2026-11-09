use crate::{Result, Value, ensure, json, number};
use crate::{
    emu::Emulator,
    harness::{self, Program},
};
use std::path::Path;

pub fn caller(entry: &str, lhs: u16, rhs: u16) -> String {
    format!(
        "SECTION \"Audit caller\", ROM0[$100]\naudit_start::\n    ld b, {}\n    ld c, {}\n    ld a, {}\n    add sp, -2\n    ld hl, sp + 0\n    ld [hl], a\n    ld a, {}\n    call {entry}\n    add sp, 2\naudit_done::\n",
        lhs & 255,
        lhs >> 8,
        rhs >> 8,
        rhs & 255
    )
}
pub fn execute(
    program: &Program,
    entry: &str,
    lhs: u16,
    rhs: u16,
    body: bool,
    stop_symbol: &str,
    initialize: impl FnOnce(&mut Emulator),
) -> Result<Value> {
    let mut emu = program.emulator(entry)?;
    emu.d = 0x55;
    emu.e = 0xAA;
    emu.set_hl(0x1234);
    emu.f = 0xF0;
    emu.set_sp(0xD000);
    let entry_stack = if body {
        emu.b = lhs as u8;
        emu.c = (lhs >> 8) as u8;
        emu.a = rhs as u8;
        emu.set_sp(emu.sp().wrapping_sub(4));
        emu.write16(emu.sp(), program.symbols[stop_symbol]);
        emu.memory[emu.sp() as usize + 2] = (rhs >> 8) as u8;
        4
    } else {
        0
    };
    initialize(&mut emu);
    let initial: Vec<_> = "abcdefhl".chars().map(|r| (r, emu.reg(r))).collect();
    emu.reset_metrics();
    let stop = program.symbols[stop_symbol];
    for _ in 0..1000 {
        if emu.pc == stop || emu.pc == 0x10 {
            break;
        }
        emu.step()?;
    }
    let status = if emu.pc == stop {
        "returned"
    } else if emu.pc == 0x10 {
        "rst_10"
    } else {
        crate::bail!("{entry} exceeded 1000 instructions at {:04X}", emu.pc)
    };
    let scratch_written = emu
        .written_addresses
        .iter()
        .filter(|&&addr| {
            program.sections.values().any(|s| {
                s.area == "WRAM0"
                    && s.address <= addr as usize
                    && (addr as usize) < s.address + s.size
            })
        })
        .count();
    let mut regs = serde_json::Map::new();
    for r in "abcdefhl".chars() {
        regs.insert(r.to_string(), json!(emu.reg(r)));
    }
    regs.insert("sp".into(), json!(emu.sp()));
    regs.insert("pc".into(), json!(emu.pc));
    let changes: Vec<_> = initial
        .into_iter()
        .filter(|(r, v)| emu.reg(*r) != *v)
        .map(|(r, _)| r.to_string())
        .collect();
    Ok(json!({
        "cycles":emu.cycles,
        "written_addresses":emu.written_addresses,
        "entry_stack_bytes":entry_stack,
        "peak_additional_stack_bytes":emu.peak_stack_bytes,
        "peak_stack_bytes":entry_stack+emu.peak_stack_bytes,
        "scratch_written_bytes":scratch_written,
        "status":status,
        "registers":regs,
        "observed_register_changes":changes,
        "stack_restored":status=="returned"&&emu.sp()==if body{0xCFFE}else{0xD000}
    }))
}
pub fn measure(helper: &str, lhs: u16, rhs: u16, artifacts: Option<&Path>) -> Result<Value> {
    measure_using(helper, lhs, rhs, artifacts, |p, entry, lhs, rhs, body| {
        execute(p, entry, lhs, rhs, body, "audit_done", |_| {})
    })
}
pub fn measure_using(
    helper: &str,
    lhs: u16,
    rhs: u16,
    artifacts: Option<&Path>,
    mut execute_body: impl FnMut(&Program, &str, u16, u16, bool) -> Result<Value>,
) -> Result<Value> {
    ensure!(
        ["mul16", "udiv16", "urem16", "stack-fixture"].contains(&helper),
        "unknown helper {helper}"
    );
    let (entry, source, expected) = if helper == "stack-fixture" {
        (
            "audit_fixture".to_owned(),
            crate::asm::STACK_FIXTURE.to_owned(),
            Some(42),
        )
    } else {
        let source = harness::runtime(if helper == "mul16" {
            "mul16"
        } else {
            "division16"
        })?
        .replacen("ROM0", "ROM0[$200]", 1);
        let expected = if helper == "mul16" {
            Some(lhs.wrapping_mul(rhs))
        } else if rhs == 0 {
            None
        } else if helper == "udiv16" {
            Some(lhs / rhs)
        } else {
            Some(lhs % rhs)
        };
        (format!("__gbc_{helper}"), source, expected)
    };
    let program = harness::assemble(
        &[
            ("helper.asm", source),
            ("caller.asm", caller(&entry, lhs, rhs)),
        ],
        artifacts,
    )?;
    let sizes = json!({
        "helper_code_bytes":program.size("ROM0",&[])-program.size("ROM0",&["Audit caller","Audit table"]),
        "table_rom_bytes":program.size("ROM0",&["Audit table"]),
        "caller_code_bytes":program.size("ROM0",&["Audit caller"]),
        "scratch_ram_bytes":program.size("WRAM0",&[])
    });
    let mut measurements = serde_json::Map::new();
    for (mode, start) in [("body", entry.as_str()), ("caller", "audit_start")] {
        let mut r = execute_body(&program, start, lhs, rhs, mode == "body")?;
        let regs = &r["registers"];
        let actual = number(&regs["a"])
            | if helper == "stack-fixture" {
                0
            } else {
                number(&regs["b"]) << 8
            };
        let correct = if let Some(expected) = expected {
            r["status"] == "returned" && r["stack_restored"] == true && actual == expected as u64
        } else {
            r["status"] == "rst_10"
        };
        r["actual"] = json!(actual);
        r["correct"] = json!(correct);
        measurements.insert(mode.into(), r);
    }
    Ok(json!({
        "helper":helper,
        "lhs":lhs,
        "rhs":rhs,
        "expected":expected,
        "sizes":sizes,
        "measurements":measurements,
        "provenance":program.provenance,
        "abi":if helper=="stack-fixture"{"stack fixture returns A and preserves BC; uses the same argument frame"}else{"lhs low/high B/C; rhs low/high A/[SP+2]; result low/high A/B"},
        "input_class":if rhs==0{"zero-rhs"}else if lhs==0{"zero-dividend"}else if rhs==1{"unit-rhs"}else if rhs<256{"narrow-rhs"}else{"full-width-rhs"},
        "allowed_clobbers":["a","f","b","c","d","e","h","l"]
    }))
}
