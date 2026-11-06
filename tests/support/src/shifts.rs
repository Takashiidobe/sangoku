use crate::{Result, Value, ensure, json};
use crate::{
    cli::Args,
    harness::{self, Program},
    random::Random,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
pub const OPS: &[&str] = &["shl", "shr", "sar", "rol", "ror"];
const REGISTERS: &[&str] = &["b", "c", "d", "e"];
#[derive(Clone, Debug)]
pub struct Spec {
    pub op: String,
    pub width: u32,
    pub count: Option<u8>,
}
impl Spec {
    pub fn name(&self) -> String {
        format!(
            "{}{}-{}",
            self.op,
            self.width,
            self.count.map_or("var".into(), |n| n.to_string())
        )
    }
    pub fn bytes(&self) -> usize {
        self.width as usize / 8
    }
    pub fn mask(&self) -> u32 {
        if self.width == 16 { 65535 } else { u32::MAX }
    }
}
pub fn expected(s: &Spec, value: u32, count: u8) -> u32 {
    let n = u32::from(s.count.unwrap_or(count)) & (s.width - 1);
    let v = value & s.mask();
    match (s.op.as_str(), s.width) {
        ("shl", _) => v.wrapping_shl(n) & s.mask(),
        ("shr", _) => v >> n,
        ("sar", 16) => ((v as u16 as i16) >> n) as u16 as u32,
        ("sar", 32) => ((v as i32) >> n) as u32,
        ("rol", 16) => (v as u16).rotate_left(n) as u32,
        ("ror", 16) => (v as u16).rotate_right(n) as u32,
        ("rol", 32) => v.rotate_left(n),
        ("ror", 32) => v.rotate_right(n),
        _ => unreachable!(),
    }
}
fn instruction(lines: &mut String, text: impl AsRef<str>) {
    lines.push_str("    ");
    lines.push_str(text.as_ref());
    lines.push('\n');
}
fn one_bit(op: &str, regs: &[&str]) -> String {
    let mut out = String::new();
    match op {
        "shl" => {
            instruction(&mut out, format!("sla {}", regs[0]));
            for reg in &regs[1..] {
                instruction(&mut out, format!("rl {reg}"));
            }
        }
        "shr" | "sar" => {
            instruction(
                &mut out,
                format!(
                    "{} {}",
                    if op == "sar" { "sra" } else { "srl" },
                    regs.last().unwrap()
                ),
            );
            for reg in regs[..regs.len() - 1].iter().rev() {
                instruction(&mut out, format!("rr {reg}"));
            }
        }
        "rol" => {
            instruction(&mut out, format!("ld a, {}", regs.last().unwrap()));
            instruction(&mut out, "rla");
            for reg in regs {
                instruction(&mut out, format!("rl {reg}"));
            }
        }
        "ror" => {
            instruction(&mut out, format!("ld a, {}", regs[0]));
            instruction(&mut out, "rra");
            for reg in regs.iter().rev() {
                instruction(&mut out, format!("rr {reg}"));
            }
        }
        _ => unreachable!(),
    };
    out
}
fn byte_step(op: &str, regs: &[&str], k: usize) -> String {
    let mut out = String::new();
    let m = regs.len();
    if k == 0 || k == m {
        return out;
    }
    if op == "rol" || op == "ror" {
        let mut visited = vec![false; m];
        for start in 0..m {
            if visited[start] {
                continue;
            }
            instruction(&mut out, format!("ld a, {}", regs[start]));
            let mut i = start;
            loop {
                visited[i] = true;
                let j = if op == "rol" {
                    (i + m - k) % m
                } else {
                    (i + k) % m
                };
                if j == start {
                    instruction(&mut out, format!("ld {}, a", regs[i]));
                    break;
                }
                instruction(&mut out, format!("ld {}, {}", regs[i], regs[j]));
                i = j;
            }
        }
    } else if op == "shl" {
        for i in (0..m).rev() {
            instruction(
                &mut out,
                if i >= k {
                    format!("ld {}, {}", regs[i], regs[i - k])
                } else {
                    format!("ld {}, 0", regs[i])
                },
            );
        }
    } else {
        if op == "sar" {
            instruction(&mut out, format!("ld a, {}", regs[m - 1]));
            instruction(&mut out, "rlca");
            instruction(&mut out, "sbc a, a");
        }
        for i in 0..m {
            instruction(
                &mut out,
                if i + k < m {
                    format!("ld {}, {}", regs[i], regs[i + k])
                } else {
                    format!("ld {}, {}", regs[i], if op == "sar" { "a" } else { "0" })
                },
            );
        }
    }
    out
}
fn kernel(s: &Spec, variant: &str) -> Result<(String, bool)> {
    let regs = &REGISTERS[..s.bytes()];
    let mut out = String::new();
    let mut pair = false;
    if variant == "bits" || variant == "bytes" || variant == "pair" {
        let n = u32::from(
            s.count
                .ok_or_else(|| anyhow::anyhow!("constant required"))?,
        ) & (s.width - 1);
        if variant == "pair" {
            ensure!(
                s.width == 16 && s.op == "shl",
                "pair specialization is shl16 only"
            );
            instruction(&mut out, "ld l, b");
            instruction(&mut out, "ld h, c");
            for _ in 0..n {
                instruction(&mut out, "add hl, hl");
            }
            pair = true;
        } else if variant == "bits" {
            for _ in 0..n {
                out += &one_bit(&s.op, regs);
            }
        } else {
            let (mut k, mut r) = (n as usize / 8, n % 8);
            let mut op = s.op.as_str();
            if op == "rol" || op == "ror" {
                if r > 4 {
                    k = (k + 1) % s.bytes();
                    r = 8 - r;
                    op = if op == "rol" { "ror" } else { "rol" };
                }
                out += &byte_step(&s.op, regs, k);
                for _ in 0..r {
                    out += &one_bit(op, regs);
                }
            } else {
                out += &byte_step(op, regs, k);
                let active = if op == "shl" {
                    &regs[k..]
                } else {
                    &regs[..regs.len() - k]
                };
                for _ in 0..r {
                    out += &one_bit(op, active);
                }
            }
        }
    } else {
        ensure!(
            variant == "loop" || variant == "byte-loop",
            "unknown shift variant {variant}"
        );
        if let Some(n) = s.count {
            instruction(&mut out, format!("ld a, {}", u32::from(n) & (s.width - 1)));
        } else {
            instruction(&mut out, format!("and {}", s.width - 1));
        }
        instruction(&mut out, "ld h, a");
        if variant == "byte-loop" {
            out += ".byte_check:\n";
            instruction(&mut out, "ld a, h");
            instruction(&mut out, "cp 8");
            instruction(&mut out, "jr c, .residual");
            instruction(&mut out, "sub 8");
            instruction(&mut out, "ld h, a");
            out += &byte_step(&s.op, regs, 1);
            instruction(&mut out, "jr .byte_check");
            out += ".residual:\n";
            instruction(&mut out, "ld a, h");
        }
        instruction(&mut out, "or a");
        instruction(&mut out, "jr z, .done");
        out += ".bit_loop:\n";
        out += &one_bit(&s.op, regs);
        instruction(&mut out, "dec h");
        instruction(&mut out, "jr nz, .bit_loop");
        out += ".done:\n";
    }
    Ok((out, pair))
}
fn finish(width: u32, pair: bool) -> String {
    if pair {
        "    ld a, l\n    ld b, h\n".into()
    } else if width == 16 {
        "    ld a, b\n    ld b, c\n".into()
    } else {
        "    ld a, b\n    ld b, c\n    ld c, d\n    ld d, e\n".into()
    }
}
fn sdcc_body(s: &Spec, variant: &str) -> Result<String> {
    let raw = harness::read(format!("tests/fixtures/shifts/{variant}.s"))?;
    normalized_sdcc_body(s, &raw)
}
fn normalized_sdcc_body(s: &Spec, raw: &str) -> Result<String> {
    let name = format!(
        "_{}{}_{}::",
        s.op,
        s.width,
        s.count
            .map_or("var".into(), |n| (u32::from(n) & (s.width - 1)).to_string())
    );
    let body = raw
        .split_once(&name)
        .ok_or_else(|| anyhow::anyhow!("missing SDCC function {name}"))?
        .1;
    let body = body.split("\n_").next().unwrap();
    let label = regex::Regex::new(r"\b(\d+)\$")?;
    let mut lines = vec!["sdcc_native:".to_owned()];
    for line in body.lines() {
        let line = line.split(';').next().unwrap().trim();
        if line.is_empty() || line.starts_with('.') {
            continue;
        }
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        let mut line = label.replace_all(&line, "sdcc_$1").replace('#', "");
        line = line
            .replace("(hl+)", "[hli]")
            .replace("(hl-)", "[hld]")
            .replace("(hl)", "[hl]");
        if let Some(args) = line.strip_prefix("ldhl ") {
            let offset = args.split(',').nth(1).unwrap().trim();
            line = format!("ld hl, sp + {offset}");
        }
        if line == "jp [hl]" {
            line = "jp hl".into();
        }
        lines.push(line);
    }
    Ok(lines.join("\n") + "\n")
}
fn sdcc_wrapper(s: &Spec) -> String {
    if s.width == 16 {
        "    ld e, b\n    ld d, c\n    call sdcc_native\n    ld a, c\n    ret\n".into()
    } else {
        let mut out="    ld e, a\n    ld hl, sp + 2\n    ld d, [hl]\n    ld a, b\n    ld b, c\n    ld c, a\n".to_owned();
        if s.count.is_none() {
            out += "    ld hl, sp + 3\n    ld a, [hl]\n    dec sp\n    ld hl, sp + 0\n    ld [hl], a\n";
        }
        out += "    call sdcc_native\n    ld a, c\n    ld c, e\n    ret\n";
        out
    }
}
fn load(s: &Spec, inline: bool) -> String {
    let mut out = "    ld hl, audit_input\n    ld b, [hl]\n    inc hl\n    ld c, [hl]\n".to_owned();
    if s.width == 32 {
        out += "    inc hl\n    ld d, [hl]\n    inc hl\n    ld e, [hl]\n";
    }
    if s.count.is_none() {
        instruction(&mut out, format!("ld a, [audit_input + {}]", s.bytes()));
    }
    if !inline && s.width == 32 {
        if s.count.is_none() {
            out +=
                "    add sp, -2\n    ld hl, sp + 1\n    ld [hl], a\n    dec hl\n    ld [hl], e\n";
        } else {
            out += "    add sp, -2\n    ld hl, sp + 0\n    ld [hl], e\n";
        }
        out += "    ld a, d\n";
    }
    out
}
fn add_live(width: u32) -> &'static str {
    if width == 16 {
        "    pop de\n    add d\n    ld c, a\n    ld a, b\n    adc e\n    ld b, a\n    ld a, c\n"
    } else {
        "    ld hl, sp + 3\n    add [hl]\n    ld e, a\n    dec hl\n    ld a, b\n    adc [hl]\n    ld b, a\n    dec hl\n    ld a, c\n    adc [hl]\n    ld c, a\n    dec hl\n    ld a, d\n    adc [hl]\n    ld d, a\n    ld a, e\n    add sp, 4\n"
    }
}
pub struct Built {
    pub program: Program,
    pub modes: Vec<&'static str>,
    pub helper: usize,
    pub caller: BTreeMap<String, usize>,
    pub native_live: bool,
}
pub fn build(
    s: &Spec,
    variant: &str,
    artifacts: Option<&Path>,
    native: Option<&Path>,
) -> Result<Built> {
    let sdcc = variant.starts_with("sdcc-");
    let native_live = variant == "native-live";
    let is_native = variant.starts_with("native");
    let (mut source, inline) = if is_native {
        let dir = native.ok_or_else(|| anyhow::anyhow!("native artifacts required"))?;
        let name = format!(
            "{}-{}",
            s.name(),
            if native_live { "live" } else { "plain" }
        );
        let vcode = fs::read_to_string(dir.join(format!("{name}.vcode")))?;
        let labels = regex::Regex::new(r"\b(?:block|label)(\d+)")?;
        (
            format!(
                "audit_shift::\n{}\naudit_shift_end::\n",
                labels.replace_all(&vcode, ".block$1")
            ),
            None,
        )
    } else if sdcc {
        (
            format!(
                "audit_shift::\n{}audit_shift_end::\n{}",
                sdcc_wrapper(s),
                sdcc_body(s, variant)?
            ),
            None,
        )
    } else {
        let (k, pair) = if variant == "lanes" {
            (String::new(), false)
        } else {
            kernel(s, variant)?
        };
        let flat = if variant == "lanes" {
            lane_kernel(s)?
        } else {
            k.clone() + &finish(s.width, pair)
        };
        let prep = if s.width == 32
            && variant == "lanes"
            && s.op == "shl"
            && s.count.is_some_and(|n| (u32::from(n) & 31) >= 8)
        {
            if (u32::from(s.count.unwrap()) & 31) < 16 {
                "    ld d, a\n"
            } else {
                ""
            }
        } else if s.width == 32
            && variant == "lanes"
            && (s.op == "shr" || s.op == "sar")
            && s.count.is_some_and(|n| (u32::from(n) & 31) >= 24)
        {
            "    ld hl, sp + 2\n    ld e, [hl]\n"
        } else if s.width == 32 {
            if s.count.is_none() {
                "    ld d, a\n    ld hl, sp + 2\n    ld e, [hl]\n    inc hl\n    ld a, [hl]\n"
            } else {
                "    ld d, a\n    ld hl, sp + 2\n    ld e, [hl]\n"
            }
        } else {
            ""
        };
        (
            format!("audit_shift::\n{prep}{flat}    ret\naudit_shift_end::\n"),
            Some(flat),
        )
    };
    source = "SECTION \"Shift helper\", ROM0[$1000]\n".to_owned() + &source;
    let mut callers = "SECTION \"Shift callers\", ROM0[$100]\n".to_owned();
    let mut modes = vec!["body", "call"];
    if !native_live {
        modes.push("call-live");
    }
    if let Some(inline) = &inline {
        modes.extend(["inline", "inline-live"]);
        callers += "MACRO SHIFT_INLINE\n";
        callers += inline;
        callers += "ENDM\n";
    }
    for &mode in &modes[1..] {
        let key = mode.replace('-', "_");
        let inlined = mode.starts_with("inline");
        let live = mode.ends_with("live");
        callers += &format!("audit_{key}_start::\n");
        // save original inputs before packing outgoing stack arguments.
        if live && !inlined && s.width == 32 {
            callers += &load(s, true);
            callers += "    push bc\n    push de\n";
            if s.count.is_none() {
                callers += "    add sp, -2\n    ld hl, sp + 1\n    ld [hl], a\n    dec hl\n    ld [hl], e\n";
            } else {
                callers += "    add sp, -2\n    ld hl, sp + 0\n    ld [hl], e\n";
            }
            callers += "    ld a, d\n";
        } else {
            callers += &load(s, inlined);
            if live {
                callers += "    push bc\n";
                if s.width == 32 {
                    callers += "    push de\n";
                }
            }
        }
        callers += if inlined {
            "    SHIFT_INLINE\n"
        } else {
            "    call audit_shift\n"
        };
        if !inlined && s.width == 32 {
            callers += "    add sp, 2\n";
        }
        if live {
            callers += add_live(s.width);
        }
        callers += &format!("audit_{key}_done::\n");
    }
    callers += "SECTION \"Shift input\", WRAM0[$c100]\naudit_input::\n    ds 5\n";
    let p = harness::assemble(
        &[("helper.asm", source), ("callers.asm", callers)],
        artifacts,
    )?;
    if is_native {
        let name = format!(
            "{}-{}",
            s.name(),
            if native_live { "live" } else { "plain" }
        );
        let original = fs::read(native.unwrap().join(format!("{name}.bin")))?;
        ensure!(
            p.rom[0x1000..0x1000 + original.len()] == original,
            "native/RGBDS mismatch {name}"
        );
    }
    let caller = modes[1..]
        .iter()
        .map(|m| {
            let key = m.replace('-', "_");
            (
                (*m).to_owned(),
                (p.symbols[&format!("audit_{key}_done")] - p.symbols[&format!("audit_{key}_start")])
                    as usize,
            )
        })
        .collect();
    let helper = p.size("ROM0", &["Shift helper"]);
    Ok(Built {
        program: p,
        modes,
        helper,
        caller,
        native_live,
    })
}
#[derive(Debug)]
pub struct Measurement {
    pub cycles: u64,
    pub peak: i32,
    pub correct: bool,
    pub actual: u32,
    pub expected: u32,
    pub changes: BTreeSet<String>,
}
pub fn run_case(
    b: &Built,
    s: &Spec,
    value: u32,
    count: u8,
    mode: &str,
    stack: u16,
    poison: u8,
) -> Result<Measurement> {
    let body = mode == "body";
    let key = mode.replace('-', "_");
    let stop = b.program.symbols[if body {
        "audit_call_done".to_owned()
    } else {
        format!("audit_{key}_done")
    }
    .as_str()];
    let entry = if body {
        "audit_shift".into()
    } else {
        format!("audit_{key}_start")
    };
    let mut e = b.program.emulator(&entry)?;
    for (i, r) in "adehl".chars().enumerate() {
        e.set_reg(r, poison.wrapping_add((i * 31) as u8));
    }
    e.f = poison & 0xF0;
    e.set_sp(stack);
    let input = value.to_le_bytes();
    e.memory[0xC100..0xC100 + s.bytes()].copy_from_slice(&input[..s.bytes()]);
    e.memory[0xC100 + s.bytes()] = count;
    let args = if s.width == 32 { 2 } else { 0 };
    if body {
        e.b = value as u8;
        e.c = (value >> 8) as u8;
        e.a = if s.width == 32 {
            (value >> 16) as u8
        } else {
            count
        };
        e.set_sp(stack - 2 - args);
        e.write16(e.sp(), stop);
        if args != 0 {
            e.memory[e.sp() as usize + 2] = (value >> 24) as u8;
            e.memory[e.sp() as usize + 3] = count;
        }
    }
    let return_sp = e.sp();
    let initial: Vec<_> = "abcdefhl".chars().map(|r| (r, e.reg(r))).collect();
    e.reset_metrics();
    e.run_until(stop, 8192)?;
    let actual = e.a as u32
        | (e.b as u32) << 8
        | if s.width == 32 {
            (e.c as u32) << 16 | (e.d as u32) << 24
        } else {
            0
        };
    let mut expected = expected(s, value, count);
    if mode.ends_with("live") || b.native_live {
        expected = expected.wrapping_add(value) & s.mask();
    }
    let peak = if body { 2 + args as i32 } else { 0 } + e.peak_stack_bytes;
    let restored = e.sp() == if body { stack - args } else { stack };
    let writes = e
        .written_addresses
        .iter()
        .all(|&a| stack as i32 - peak <= a as i32 && a < stack);
    let frame = !body || e.read16(return_sp) == stop;
    let argument = !body
        || args == 0
        || (e.memory[(return_sp + 2) as usize] == (value >> 24) as u8
            && e.memory[(return_sp + 3) as usize] == count);
    Ok(Measurement {
        cycles: e.cycles,
        peak,
        correct: actual == expected && restored && writes && frame && argument,
        actual,
        expected,
        changes: initial
            .into_iter()
            .filter(|(r, v)| e.reg(*r) != *v)
            .map(|(r, _)| r.to_string())
            .collect(),
    })
}

fn lane_kernel(s: &Spec) -> Result<String> {
    let n = u32::from(
        s.count
            .ok_or_else(|| anyhow::anyhow!("constant required"))?,
    ) & (s.width - 1);
    let m = s.bytes();
    let regs = &["a", "b", "c", "d"][..m];
    let k = n as usize / 8;
    let mut r = n % 8;
    let mut op = s.op.as_str();
    let mut out = String::new();
    let mut byte_count = k;
    if s.op == "shl" && k == m - 1 && r > 4 {
        instruction(&mut out, "ld a, b");
        for _ in 0..8 - r {
            instruction(&mut out, "rrca");
        }
        instruction(&mut out, format!("and {}", (255u32 << r) & 255));
        instruction(&mut out, format!("ld {}, a", regs[m - 1]));
        instruction(&mut out, "xor a");
        for reg in &regs[1..m - 1] {
            instruction(&mut out, format!("ld {reg}, a"));
        }
        return Ok(out);
    }
    if (op == "rol" || op == "ror") && r > 4 {
        byte_count = (k + 1) % m;
        r = 8 - r;
        op = if op == "rol" { "ror" } else { "rol" };
    }
    if s.op == "sar" && k != 0 {
        instruction(&mut out, format!("ld a, {}", REGISTERS[m - 1]));
        instruction(&mut out, "rlca");
        instruction(&mut out, "sbc a, a");
        instruction(&mut out, "ld h, a");
    }
    let mut pending: Vec<(&str, &str)> = (0..m)
        .map(|i| {
            let source = match s.op.as_str() {
                "shl" => {
                    if i >= k {
                        REGISTERS[i - k]
                    } else {
                        "0"
                    }
                }
                "shr" | "sar" => {
                    if i + k < m {
                        REGISTERS[i + k]
                    } else if s.op == "sar" {
                        "h"
                    } else {
                        "0"
                    }
                }
                "rol" => REGISTERS[(i + m - byte_count) % m],
                _ => REGISTERS[(i + byte_count) % m],
            };
            (regs[i], source)
        })
        .filter(|(d, s)| d != s)
        .collect();
    let mut a_zero = false;
    while !pending.is_empty() {
        if let Some(index) = pending
            .iter()
            .position(|(dst, _)| !pending.iter().any(|(_, src)| dst == src))
        {
            let (dst, src) = pending.remove(index);
            if src == "0" && dst == "a" {
                instruction(&mut out, "xor a");
                a_zero = true;
            } else {
                instruction(
                    &mut out,
                    format!("ld {dst}, {}", if src == "0" && a_zero { "a" } else { src }),
                );
                if dst == "a" {
                    a_zero = false;
                }
            }
        } else {
            let register = pending[0].0;
            instruction(&mut out, format!("ld l, {register}"));
            for (_, src) in &mut pending {
                if *src == register {
                    *src = "l";
                }
            }
        }
    }
    let active = if s.op == "shl" {
        &regs[k..]
    } else if s.op == "shr" || s.op == "sar" {
        &regs[..m - k]
    } else {
        regs
    };
    if op == "shr" && active.len() == 1 && active[0] == "a" && r != 0 {
        for _ in 0..if r > 4 { 8 - r } else { r } {
            instruction(&mut out, if r > 4 { "rlca" } else { "rrca" });
        }
        instruction(&mut out, format!("and {}", 255u32 >> r));
        return Ok(out);
    }
    for _ in 0..r {
        match op {
            "shl" => {
                instruction(
                    &mut out,
                    if active[0] == "a" {
                        "add a, a".into()
                    } else {
                        format!("sla {}", active[0])
                    },
                );
                for reg in &active[1..] {
                    instruction(&mut out, format!("rl {reg}"));
                }
            }
            "shr" | "sar" => {
                instruction(
                    &mut out,
                    format!(
                        "{} {}",
                        if op == "sar" { "sra" } else { "srl" },
                        active.last().unwrap()
                    ),
                );
                for &reg in active[..active.len() - 1].iter().rev() {
                    instruction(
                        &mut out,
                        if reg == "a" {
                            "rra".into()
                        } else {
                            format!("rr {reg}")
                        },
                    );
                }
            }
            "rol" => {
                instruction(&mut out, format!("ld e, {}", regs[m - 1]));
                instruction(&mut out, "rlc e");
                instruction(&mut out, "rla");
                for reg in &regs[1..] {
                    instruction(&mut out, format!("rl {reg}"));
                }
            }
            "ror" => {
                instruction(&mut out, "ld e, a");
                instruction(&mut out, "rrc e");
                for reg in regs[1..].iter().rev() {
                    instruction(&mut out, format!("rr {reg}"));
                }
                instruction(&mut out, "rra");
            }
            _ => unreachable!(),
        }
    }
    Ok(out)
}

#[derive(Default)]
struct Stats {
    n: u64,
    sum: u64,
    hist: BTreeMap<u64, u64>,
    peak: i32,
    changes: BTreeSet<String>,
}
impl Stats {
    fn add(&mut self, m: &Measurement) {
        self.n += 1;
        self.sum += m.cycles;
        *self.hist.entry(m.cycles).or_default() += 1;
        self.peak = self.peak.max(m.peak);
        self.changes.extend(m.changes.iter().cloned());
    }
    fn value(&self) -> Value {
        let mut seen = 0;
        let mut lower = 0;
        let mut upper = 0;
        for (&cycles, &count) in &self.hist {
            if seen <= (self.n - 1) / 2 && (self.n - 1) / 2 < seen + count {
                lower = cycles;
            }
            if seen <= self.n / 2 && self.n / 2 < seen + count {
                upper = cycles;
            }
            seen += count;
        }
        json!({"count":self.n,"min_cycles":self.hist.first_key_value().unwrap().0,"median_cycles":if self.n%2==0{json!((lower+upper)as f64/2.0)}else{json!(upper)},"max_cycles":self.hist.last_key_value().unwrap().0,"mean_cycles":if self.sum.is_multiple_of(self.n){json!(self.sum/self.n)}else{json!(self.sum as f64/self.n as f64)},"peak_stack_bytes":self.peak,"observed_changes":self.changes})
    }
}
fn vectors(width: u32, samples: usize, seed: u64) -> Vec<u32> {
    let mask = if width == 16 { 65535 } else { u32::MAX };
    let sign = 1u32 << (width - 1);
    let mut values: BTreeSet<u32> = [
        0,
        1,
        2,
        sign - 1,
        sign,
        sign + 1,
        mask - 1,
        mask,
        0x55555555 & mask,
        0xAAAAAAAA & mask,
        0x12345678 & mask,
        0x80000001 & mask,
        0xFF00FF00 & mask,
        0x00FF00FF & mask,
    ]
    .into_iter()
    .collect();
    for bit in 0..width {
        values.insert(1 << bit);
        values.insert(mask ^ (1 << bit));
    }
    for byte in 1..width / 8 {
        let edge = 1u32 << (byte * 8);
        values.extend([edge - 1, edge, edge + 1]);
    }
    let mut rng = Random::new(seed);
    for _ in 0..samples {
        let low = rng.range(0, 65536) as u32;
        values.insert(if width == 16 {
            low
        } else {
            low | (rng.range(0, 65536) as u32) << 16
        });
    }
    values.into_iter().collect()
}
fn counts(width: u32) -> Vec<u8> {
    (0..width as u8)
        .chain([width as u8, (width + 1) as u8, 255])
        .collect()
}
fn run_command(
    directory: &Path,
    name: &str,
    args: Vec<String>,
    commands: &mut Vec<Vec<String>>,
) -> Result<()> {
    commands.push(args.clone());
    let output = Command::new(&args[0])
        .args(&args[1..])
        .current_dir(harness::root())
        .output()?;
    harness::write(
        &directory.join(format!("{name}.log")),
        [output.stdout.as_slice(), output.stderr.as_slice()].concat(),
    )?;
    ensure!(
        output.status.success(),
        "{name} failed; see {}/{}.log",
        directory.display(),
        name
    );
    Ok(())
}
fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).into()).collect()
}
fn prepare(
    directory: &Path,
    skip: bool,
    full: bool,
    commands: &mut Vec<Vec<String>>,
) -> Result<()> {
    if !skip {
        let mut args = strings(&[
            "cargo",
            "+gbc-lr35902",
            "run",
            "--locked",
            "--manifest-path",
            "tests/fixtures/shifts/clif/Cargo.toml",
            "--target-dir",
            "third_party/wasmtime/target",
            "--",
        ]);
        args.push(directory.join("clif").display().to_string());
        if full {
            args.push("--full".into());
        }
        run_command(directory, "clif-build", args, commands)?;
        run_command(
            directory,
            "backend-build",
            strings(&[
                "cargo",
                "+gbc-lr35902",
                "build",
                "--release",
                "--locked",
                "--manifest-path",
                "third_party/rustc_codegen_cranelift/Cargo.toml",
            ]),
            commands,
        )?;
        let mut args = strings(&[
            "rustc",
            "+gbc-lr35902",
            "-Zunstable-options",
            "-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so",
            "--target",
            "targets/lr35902.json",
            "--emit=asm",
            "--crate-type=lib",
            "-Cpanic=abort",
            "-Copt-level=2",
            "tests/fixtures/shifts/rust.rs",
            "-o",
        ]);
        args.push(directory.join("rust.asm").display().to_string());
        run_command(directory, "rust", args, commands)?;
    }
    Ok(())
}
fn compiler_rows(
    directory: &Path,
    values: &BTreeMap<u32, Vec<u32>>,
    rows: &mut Vec<Value>,
    errors: &mut Vec<Value>,
    full: bool,
) -> Result<usize> {
    let native = directory.join("clif");
    let metadata: Value = serde_json::from_str(&fs::read_to_string(native.join("results.json"))?)?;
    let mut executions = 0;
    for entry in metadata.as_array().unwrap() {
        if entry["status"] != "compiled" {
            continue;
        }
        let spec = Spec {
            op: entry["operation"].as_str().unwrap().into(),
            width: entry["width"].as_u64().unwrap() as u32,
            count: entry["count"].as_u64().map(|n| n as u8),
        };
        if !full && !selected_counts(spec.width, false).contains(&spec.count.unwrap_or(0)) {
            continue;
        }
        let variant = if entry["live"] == true {
            "native-live"
        } else {
            "native"
        };
        let built = build(
            &spec,
            variant,
            Some(
                &directory
                    .join("artifacts")
                    .join(format!("{}-{variant}", spec.name())),
            ),
            Some(&native),
        )?;
        for &mode in built
            .modes
            .iter()
            .filter(|&&mode| full || mode == "body" || mode == "call")
        {
            let mut stat = Stats::default();
            for &v in &values[&spec.width] {
                let n = spec.count.unwrap();
                let result = run_case(&built, &spec, v, n, mode, 0xD000, 0x55)?;
                executions += 1;
                if !result.correct {
                    errors.push(json!({"variant":variant,"name":spec.name(),"mode":mode,"value":v,"actual":result.actual,"expected":result.expected}));
                }
                stat.add(&result);
            }
            let mut row = stat.value();
            row["variant"] = json!(variant);
            row["operation"] = json!(spec.op);
            row["width"] = json!(spec.width);
            row["count"] = json!(spec.count);
            row["mode"] = json!(mode);
            row["code_bytes"] = json!(built.helper);
            row["caller_bytes"] = json!(built.caller.get(mode));
            row["scratch_ram_bytes"] = json!(0);
            row["table_bytes"] = json!(0);
            rows.push(row);
        }
    }
    Ok(executions)
}
fn rust_checks(directory: &Path) -> Result<Value> {
    let source = fs::read_to_string(directory.join("rust.asm"))?;
    let source = regex::Regex::new(r#"SECTION "[^"]+", ROM0"#)?
        .replacen(
            &source,
            1,
            regex::NoExpand("SECTION \"Rust probes\", ROM0[$1000]"),
        )
        .to_string();
    let p = harness::assemble(
        &[
            ("rust.asm", source),
            (
                "stop.asm",
                "SECTION \"Rust stop\", ROM0[$100]\naudit_stop::\n nop\n".into(),
            ),
        ],
        Some(&directory.join("rust-probes")),
    )?;
    let mut rows = vec![];
    for (symbol, width, count) in [
        ("rust_shl16_8", 16, 8),
        ("rust_shr32_8", 32, 8),
        ("checked16_15", 16, 15),
        ("checked16_16", 16, 16),
        ("checked16_65536", 16, 65536),
        ("checked32_31", 32, 31),
        ("checked32_32", 32, 32),
        ("checked16_valid_15", 16, 15),
        ("checked16_valid_16", 16, 16),
        ("checked16_valid_65536", 16, 65536),
        ("checked32_valid_31", 32, 31),
        ("checked32_valid_32", 32, 32),
    ] {
        for value in representative_values(width, 2, 7) {
            let mut e = p.emulator(&format!("__rust_{symbol}"))?;
            e.b = value as u8;
            e.c = (value >> 8) as u8;
            e.a = (value >> 16) as u8;
            let stack = 0xD000u16;
            let args = if width == 32 { 2 } else { 0 };
            e.set_sp(stack - 2 - args);
            e.write16(e.sp(), p.symbols["audit_stop"]);
            if width == 32 {
                e.memory[e.sp() as usize + 2] = (value >> 24) as u8;
            }
            e.reset_metrics();
            e.run_until(p.symbols["audit_stop"], 8192)?;
            let actual = e.a as u32
                | (e.b as u32) << 8
                | if width == 32 {
                    (e.c as u32) << 16 | (e.d as u32) << 24
                } else {
                    0
                };
            let actual = if symbol.contains("valid") {
                e.a as u32
            } else {
                actual
            };
            let expected = if symbol.contains("valid") {
                u32::from(count < width)
            } else if count >= width {
                0
            } else if width == 32 {
                value >> count
            } else {
                ((value as u16).wrapping_shl(count)) as u32
            };
            ensure!(
                actual == expected,
                "Rust probe {symbol} value={value:X}: {actual:X} != {expected:X}"
            );
            ensure!(e.sp() == stack - args, "Rust probe {symbol} SP");
            ensure!(
                e.written_addresses.iter().all(|&a| stack as i32
                    - 2
                    - args as i32
                    - e.peak_stack_bytes
                    <= a as i32
                    && a < stack - args),
                "Rust probe {symbol} writes"
            );
        }
        rows.push(json!({"symbol":symbol,"width":width,"count":count,"executions":representative_values(width,2,7).len(),"correct":true}));
    }
    Ok(json!({"results":rows,"provenance":p.provenance}))
}
pub fn main(args: &[String]) -> Result<()> {
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        println!(
            "cargo run --example audit-shifts -- [--samples N] [--seed N] [--full] [--exhaustive16] [--skip-build] [--output FILE] [--artifacts DIR] [--csv FILE]"
        );
        return Ok(());
    }
    let a = Args::parse(
        args,
        &["samples", "seed", "output", "artifacts", "csv", "sdcc"],
        &["skip-build", "full", "exhaustive16"],
    )?;
    let full = a.has("full");
    let samples = a.integer("samples", if full { 128 } else { 2 })? as usize;
    ensure!(samples > 0, "--samples must be positive");
    ensure!(
        full || samples <= 8,
        "more than 8 random samples requires --full"
    );
    ensure!(
        !a.has("exhaustive16") || full,
        "--exhaustive16 requires --full"
    );
    let seed = a.integer("seed", 0)?;
    let directory = harness::root().join(a.path("artifacts", "build/shift-audit"));
    fs::create_dir_all(&directory)?;
    let mut commands = vec![];
    prepare(
        &directory,
        a.has("skip-build"),
        a.has("full"),
        &mut commands,
    )?;
    if let Some(sdcc) = a.get("sdcc") {
        regenerate_sdcc(&directory, Path::new(sdcc), &mut commands)?;
    }

    let values: BTreeMap<_, _> = [16, 32]
        .into_iter()
        .map(|w| {
            (
                w,
                if full {
                    vectors(w, samples, seed)
                } else {
                    representative_values(w, samples, seed)
                },
            )
        })
        .collect();
    let mut rows = vec![];
    let mut errors = vec![];
    let mut executions = 0usize;
    let mut exhaustive = 0usize;
    let mut provenance = serde_json::Map::new();
    for width in [16, 32] {
        for &op in OPS {
            for count in selected_counts(width, full)
                .into_iter()
                .map(Some)
                .chain([None])
            {
                let s = Spec {
                    op: op.into(),
                    width,
                    count,
                };
                let variants: &[&str] = if !full {
                    if count.is_none() {
                        &["loop", "byte-loop"]
                    } else if width == 16 && op == "shl" {
                        &["lanes", "pair", "sdcc-speed"]
                    } else {
                        &["lanes", "sdcc-speed"]
                    }
                } else if count.is_none() {
                    &["loop", "byte-loop", "sdcc-speed", "sdcc-size"]
                } else if width == 16 && op == "shl" {
                    &[
                        "bits",
                        "bytes",
                        "lanes",
                        "pair",
                        "loop",
                        "sdcc-speed",
                        "sdcc-size",
                    ]
                } else {
                    &["bits", "bytes", "lanes", "loop", "sdcc-speed", "sdcc-size"]
                };
                for &variant in variants {
                    println!("auditing {} {variant}", s.name());
                    let key = format!("{}-{variant}", s.name());
                    let built = build(
                        &s,
                        variant,
                        Some(&directory.join("artifacts").join(&key)),
                        None,
                    )?;
                    provenance.insert(key, built.program.provenance.clone());
                    for n in if let Some(n) = count {
                        vec![n]
                    } else {
                        selected_counts(width, full)
                    } {
                        for &mode in built
                            .modes
                            .iter()
                            .filter(|&&mode| full || mode == "body" || mode == "call")
                        {
                            let mut stats = Stats::default();
                            for &value in &values[&width] {
                                let result = run_case(&built, &s, value, n, mode, 0xD000, 0x55)?;
                                executions += 1;
                                if !result.correct {
                                    errors.push(json!({"variant":variant,"name":s.name(),"mode":mode,"value":value,"count":n,"actual":result.actual,"expected":result.expected}));
                                }
                                stats.add(&result);
                            }
                            let mut row = stats.value();
                            row["variant"] = json!(variant);
                            row["operation"] = json!(op);
                            row["width"] = json!(width);
                            row["count"] = json!(n);
                            row["variable_count"] = json!(count.is_none());
                            row["mode"] = json!(mode);
                            row["code_bytes"] = json!(built.helper);
                            row["caller_bytes"] = json!(built.caller.get(mode));
                            row["one_use_rom_bytes"] = json!(if mode == "body" {
                                built.helper
                            } else if mode.starts_with("inline") {
                                built.caller[mode]
                            } else {
                                built.helper + built.caller[mode]
                            });
                            row["scratch_ram_bytes"] = json!(0);
                            row["table_bytes"] = json!(0);
                            if variant.starts_with("sdcc-") {
                                let bridge = built.program.symbols["sdcc_native"] as usize - 0x1000;
                                row["bridge_code_bytes"] = json!(bridge);
                                row["sdcc_native_code_bytes"] = json!(built.helper - bridge);
                                if mode == "body" {
                                    let bridge_cycles = if width == 16 {
                                        52
                                    } else if count.is_none() {
                                        132
                                    } else {
                                        84
                                    };
                                    row["derived_native_min_cycles"] =
                                        json!(row["min_cycles"].as_u64().unwrap() - bridge_cycles);
                                    row["derived_native_max_cycles"] =
                                        json!(row["max_cycles"].as_u64().unwrap() - bridge_cycles);
                                }
                            }
                            if mode == "call" || mode == "call-live" {
                                let inline_mode = if mode == "call" {
                                    "inline"
                                } else {
                                    "inline-live"
                                };
                                if let Some(&bytes) = built.caller.get(inline_mode) {
                                    let call = built.caller[mode];
                                    row["uses_for_strict_rom_saving"] = if bytes > call {
                                        json!(built.helper / (bytes - call) + 1)
                                    } else {
                                        Value::Null
                                    };
                                    row["outline_cost_per_use_bytes"] = json!(call);
                                    row["inline_cost_per_use_bytes"] = json!(bytes);
                                }
                            }
                            rows.push(row);
                        }
                    }
                    if a.has("exhaustive16")
                        && width == 16
                        && variant == "lanes"
                        && count.is_some_and(|n| n < 16)
                    {
                        for value in 0..=65535 {
                            let r =
                                run_case(&built, &s, value, count.unwrap(), "body", 0xD000, 0x55)?;
                            ensure!(r.correct, "exhaustive failure {s:?} value={value:X}: {r:?}");
                            exhaustive += 1;
                        }
                    }
                }
            }
        }
    }
    let compiler_executions = compiler_rows(&directory, &values, &mut rows, &mut errors, full)?;
    let rust = rust_checks(&directory)?;
    let root = harness::root();
    let mut hashes = serde_json::Map::new();
    for name in [
        "tests/support/src/shifts.rs",
        "tests/support/src/emu.rs",
        "tests/support/src/harness.rs",
        "tests/support/src/random.rs",
        "tests/fixtures/shifts/rust.rs",
        "tests/fixtures/shifts/provenance.json",
        "tests/fixtures/shifts/clif/Cargo.toml",
        "third_party/rustc_codegen_cranelift/Cargo.lock",
        "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/abi.rs",
        "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/regs.rs",
        "tests/fixtures/shifts/sdcc.c",
        "tests/fixtures/shifts/sdcc-speed.s",
        "tests/fixtures/shifts/sdcc-size.s",
        "tests/fixtures/shifts/clif/src/main.rs",
        "tests/fixtures/shifts/clif/Cargo.lock",
        "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/lower.rs",
        "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/emit.rs",
        "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/mod.rs",
    ] {
        hashes.insert(
            name.into(),
            json!(harness::hash(&fs::read(root.join(name))?)),
        );
    }
    let metadata: Value =
        serde_json::from_str(&fs::read_to_string(directory.join("clif/results.json"))?)?;
    let report = json!({"timing_unit":"CPU T-cycles","full":full,"samples":samples,"seed":seed,"inputs":values,"rows":rows,"errors":errors.iter().take(16).collect::<Vec<_>>(),"error_count":errors.len(),"validated_executions":executions,"exhaustive16_executions":exhaustive,"compiler_executions":compiler_executions,"compiler_capabilities":metadata,"rust_probes":rust,"source_sha256":hashes,"commands":commands,"provenance":provenance,"limitations":["flat-memory instruction subset; no interrupts, banking or peripheral timing","default vectors are zero, sign bit, maximum, patterned and seeded random words; --full adds single-bit and complementary-bit inputs","CALL paths use normal ABI saves; inline-live conservatively saves original input on stack","stack includes return address, incoming argument slot, saves and nested calls","SDCC native timings are derived by subtracting the fixed measured bridge instruction cost","constant Rust guarded payload probes validate the range guard; full core Option-return ABI is not tested"]});
    harness::write_json(&a.path("output", "build/shift-audit/results.json"), &report)?;
    if let Some(path) = a.get("csv") {
        let fields = [
            "operation",
            "width",
            "count",
            "variable_count",
            "variant",
            "mode",
            "code_bytes",
            "caller_bytes",
            "one_use_rom_bytes",
            "min_cycles",
            "median_cycles",
            "max_cycles",
            "mean_cycles",
            "peak_stack_bytes",
            "uses_for_strict_rom_saving",
            "sdcc_native_code_bytes",
            "derived_native_min_cycles",
        ];
        let mut text = fields.join(",") + "\n";
        for row in report["rows"].as_array().unwrap() {
            text += &fields
                .iter()
                .map(|k| match row.get(*k) {
                    None | Some(Value::Null) => String::new(),
                    Some(Value::String(s)) => s.clone(),
                    Some(v) => v.to_string(),
                })
                .collect::<Vec<_>>()
                .join(",");
            text.push('\n');
        }
        harness::write(Path::new(path), text)?;
    }
    ensure!(
        errors.is_empty(),
        "{} incorrect cases; see report",
        errors.len()
    );
    println!(
        "{executions} candidate executions, {exhaustive} exhaustive cases, {compiler_executions} compiler executions"
    );
    Ok(())
}

fn regenerate_sdcc(directory: &Path, sdcc: &Path, commands: &mut Vec<Vec<String>>) -> Result<()> {
    use std::io::Write;
    use std::process::Stdio;
    let args = strings(&["cc", "-E", "-P", "tests/fixtures/shifts/sdcc.c"]);
    commands.push(args.clone());
    let expanded = harness::command(&args)?;
    let input = format!(
        "#line 1 {}\n{expanded}",
        serde_json::to_string(
            &harness::root()
                .join("tests/fixtures/shifts/sdcc.c")
                .display()
                .to_string()
        )?
    );
    harness::write(&directory.join("sdcc.i"), &input)?;
    for (profile, flag) in [("speed", "--opt-code-speed"), ("size", "--opt-code-size")] {
        let output = directory.join(format!("sdcc-{profile}.s"));
        let args = vec![
            harness::root().join(sdcc).display().to_string(),
            "--c1mode".into(),
            "-msm83".into(),
            "--std-c23".into(),
            flag.into(),
            "-o".into(),
            output.display().to_string(),
        ];
        commands.push(args.clone());
        let mut child = Command::new(&args[0])
            .args(&args[1..])
            .current_dir(harness::root())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        child.stdin.take().unwrap().write_all(input.as_bytes())?;
        let result = child.wait_with_output()?;
        harness::write(
            &directory.join(format!("sdcc-{profile}.log")),
            [result.stdout.as_slice(), result.stderr.as_slice()].concat(),
        )?;
        ensure!(result.status.success(), "SDCC {profile} compilation failed");
        let raw = fs::read_to_string(output)?;
        for width in [16, 32] {
            for &op in OPS {
                for count in (0..width as u8).map(Some).chain([None]) {
                    let s = Spec {
                        op: op.into(),
                        width,
                        count,
                    };
                    ensure!(
                        normalized_sdcc_body(&s, &raw)?
                            == sdcc_body(&s, &format!("sdcc-{profile}"))?,
                        "regenerated SDCC {profile} differs for {}",
                        s.name()
                    );
                }
            }
        }
    }
    Ok(())
}

fn selected_counts(width: u32, full: bool) -> Vec<u8> {
    if full {
        counts(width)
    } else {
        vec![0, 1, 8, (width - 1) as u8]
    }
}
fn representative_values(width: u32, samples: usize, seed: u64) -> Vec<u32> {
    let mask = if width == 16 { 65535 } else { u32::MAX };
    let mut values = vec![0, 1u32 << (width - 1), mask, 0x12345678 & mask];
    let mut rng = Random::new(seed);
    for _ in 0..samples {
        let low = rng.range(0, 65536) as u32;
        values.push(if width == 16 {
            low
        } else {
            low | (rng.range(0, 65536) as u32) << 16
        });
    }
    values
}
