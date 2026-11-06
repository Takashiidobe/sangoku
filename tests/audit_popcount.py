import argparse
import csv
import io
import json
import random
import re
import statistics
from collections import defaultdict
from pathlib import Path

from runtime_harness import ROOT, assemble

FIXTURES = ROOT / "tests/fixtures/popcount"
VARIANTS = (
    "repaired",
    "shift",
    "byte-loop",
    "unrolled",
    "clear",
    "swar",
    "swar-shared",
    "nibble",
    "nibble-page",
    "byte",
    "sdcc",
)
INLINE = {
    "repaired",
    "shift",
    "byte-loop",
    "unrolled",
    "clear",
    "swar",
    "nibble",
    "nibble-page",
    "byte",
}
PRESERVES_HL = {"shift", "byte-loop", "unrolled", "clear", "swar", "swar-shared"}
LOAD_INPUT = """    ld hl, audit_input
    ld b, [hl]
    inc hl
    ld c, [hl]
"""
ADD_STACK_INPUT = """    pop de
    add d
    ld c, a
    ld a, b
    adc e
    ld b, a
    ld a, c
"""
ADD_REGISTER_INPUT = """    add l
    ld c, a
    ld a, b
    adc h
    ld b, a
    ld a, c
"""


def sdcc_source():
    lines = []
    for line in (FIXTURES / "sdcc-native.s").read_text().splitlines():
        line = line.split(";", 1)[0].strip()
        if not line or line.startswith("."):
            continue
        line = re.sub(r"\b(\d+)\$", r"sdcc_\1", line).replace("#", "")
        lines.append(line.replace("_audit_popcount::", "sdcc_count:"))
    return (
        """audit_popcount::
    ld e, b
    ld d, c
    call sdcc_count
    ld a, c
    ret
audit_popcount_end::
"""
        + "\n".join(lines)
        + "\n"
    )


def source_for(variant):
    if variant in ("raw", "repaired"):
        source = (FIXTURES / "baseline.asm").read_text()
        source = re.sub(
            r'SECTION "[^"]+", ROM0',
            'SECTION "Audit helper", ROM0[$400]',
            source,
            count=1,
        )
        source = source.replace("__rust_audit_popcount", "audit_popcount")
        if variant == "repaired":
            store = "ld hl, sp + 0\nld [hl], a"
            load = "ld hl, sp + 0\nld a, [hl]"
            if source.count(store) != 1 or source.count(load) != 1:
                raise ValueError("unexpected frozen spill/reload sequences")
            source = source.replace(store, "push hl\nld hl, sp + 2\nld [hl], a\npop hl")
            source = source.replace(load, "push hl\nld hl, sp + 2\nld a, [hl]\npop hl")
        return source + "\naudit_popcount_end::\n"
    source = (
        sdcc_source()
        if variant == "sdcc"
        else (FIXTURES / f"{variant.removesuffix('-page')}.asm").read_text()
    )
    if variant == "nibble-page":
        source = source.replace("    add LOW(audit_table)\n", "").replace(
            "ROM0[$810], ALIGN[4]", "ROM0[$800], ALIGN[8]"
        )
    return 'SECTION "Audit helper", ROM0[$400]\n' + source


def build(variant, *, artifacts=None):
    source = source_for(variant)
    caller = 'SECTION "Audit callers", ROM0[$100]\n'
    modes = ["call", "call-live"]
    if variant in INLINE:
        body = (
            source.split("audit_popcount::\n", 1)[1]
            .split("audit_popcount_end::", 1)[0]
            .strip()
        )
        if not body.endswith("ret"):
            raise ValueError("inline fixture must end in RET")
        caller += "MACRO POPCOUNT_INLINE\n" + body[:-3] + "\nENDM\n"
        modes += ["inline", "inline-live"]
    for mode in modes:
        key = mode.replace("-", "_")
        caller += f"audit_{key}_start::\n" + LOAD_INPUT
        live = mode.endswith("live")
        registers = live and mode.startswith("inline") and variant in PRESERVES_HL
        if live:
            caller += "    ld l, b\n    ld h, c\n" if registers else "    push bc\n"
        caller += (
            "    POPCOUNT_INLINE\n"
            if mode.startswith("inline")
            else "    call audit_popcount\n"
        )
        if live:
            caller += ADD_REGISTER_INPUT if registers else ADD_STACK_INPUT
        caller += f"audit_{key}_done::\n"
    caller += 'SECTION "Audit input", WRAM0[$c100]\naudit_input::\n    ds 2\n'
    return assemble({"helper.asm": source, "callers.asm": caller}, artifacts=artifacts)


def modes_for(variant):
    return (
        ("body", "call", "call-live", "inline", "inline-live")
        if variant in INLINE
        else ("body", "call", "call-live")
    )


def run_case(program, variant, value, mode="body", *, poison=0x55, stack_base=0xD000):
    key = mode.replace("-", "_")
    body = mode == "body"
    stop = program.symbols["audit_call_done" if body else f"audit_{key}_done"]
    emu = program.emulator("audit_popcount" if body else f"audit_{key}_start")
    emu.d, emu.e, emu.hl, emu.f = (
        poison,
        poison ^ 255,
        (poison << 8) | (poison ^ 255),
        0xF0,
    )
    emu.sp = stack_base
    emu.memory[0xC100:0xC102] = value.to_bytes(2, "little")
    if body:
        emu.b, emu.c = value & 255, value >> 8
        emu.sp -= 2
        emu.write16(emu.sp, stop)
    initial = {register: getattr(emu, register) for register in "abcdefhl"}
    original_hl = emu.hl
    emu.reset_metrics()
    emu.run_until(stop, max_instructions=2048)
    actual = emu.a | emu.b << 8
    expected = (
        (value + value.bit_count()) & 65535
        if mode.endswith("live")
        else value.bit_count()
    )
    entry_stack = 2 if body else 0
    peak = entry_stack + emu.peak_stack_bytes
    writes = all(
        stack_base - peak <= address < stack_base for address in emu.written_addresses
    )
    preserved = not body or variant not in PRESERVES_HL or emu.hl == original_hl
    restored = emu.sp == stack_base
    return {
        "actual": actual,
        "expected": expected,
        "correct": actual == expected and writes and restored and preserved,
        "cycles": emu.cycles,
        "entry_stack_bytes": entry_stack,
        "peak_stack_bytes": peak,
        "writes_in_stack": writes,
        "stack_restored": restored,
        "observed_changes": [
            register
            for register in initial
            if getattr(emu, register) != initial[register]
        ],
    }


def input_classes(samples, seed):
    rng = random.Random(seed)
    return {
        "all-8-bit": list(range(256)),
        "zero": [0],
        "single-bit": [1 << bit for bit in range(16)],
        "sparse": sorted(
            {
                (1 << left) | (1 << right)
                for left in range(16)
                for right in range(left, 16)
            }
        ),
        "dense": sorted(
            {
                65535 ^ ((1 << left) | (1 << right))
                for left in range(16)
                for right in range(left, 16)
            }
        ),
        "boundary": [
            0,
            1,
            127,
            128,
            255,
            256,
            257,
            32767,
            32768,
            65534,
            65535,
            0x5555,
            0xAAAA,
            0x8001,
            0xFF00,
        ],
        "uniform-random": [rng.randrange(65536) for _ in range(samples)],
    }


def audit(variants, *, samples, seed, exhaustive16, artifacts):
    classes = input_classes(samples, seed)
    raw = build("raw", artifacts=artifacts / "raw")
    report = {
        "timing_unit": "CPU T-cycles",
        "seed": seed,
        "samples": samples,
        "input_classes": classes,
        "baseline": [],
        "results": [],
        "limitations": [
            "flat-memory instruction subset, no interrupts/banking/peripheral timing",
            "synthetic callers load input from two harness-owned RAM bytes",
            "call-live conservatively saves input on stack across the normal call ABI",
            "inline-live preserves input in free HL where the audited body preserves that pair",
            "table alignment padding and actual future allocator spills are separate costs",
        ],
    }
    for value in (0, 1, 255, 256, 0x1234, 65535):
        for stack_base in (0xD000, 0xD002):
            report["baseline"].append(
                {
                    "value": value,
                    "stack_base": stack_base,
                    "result": run_case(raw, "raw", value, stack_base=stack_base),
                }
            )
    report["baseline_code_bytes"] = raw.size("ROM0", {"Audit helper"})
    report["baseline_provenance"] = raw.provenance
    for variant in variants:
        print(f"auditing {variant}", flush=True)
        program = build(variant, artifacts=artifacts / variant)
        groups = defaultdict(list)
        errors = []
        executions = 0
        changes = defaultdict(set)
        for label, values in classes.items():
            for value in values:
                for mode in modes_for(variant):
                    result = run_case(program, variant, value, mode)
                    executions += 1
                    changes[mode].update(result["observed_changes"])
                    if not result["correct"]:
                        errors.append(
                            {
                                "class": label,
                                "value": value,
                                "mode": mode,
                                "result": result,
                            }
                        )
                    groups[(label, mode)].append(result)
        exhaustive = {"executed": 0, "correct": True}
        if exhaustive16 and variant in ("swar", "nibble", "nibble-page", "byte"):
            print(f"{variant}: exhaustive u16 domain", flush=True)
            for value in range(65536):
                result = run_case(program, variant, value)
                exhaustive["executed"] += 1
                exhaustive["correct"] &= result["correct"]
        summary = []
        for (label, mode), results in groups.items():
            cycles = [result["cycles"] for result in results]
            summary.append(
                {
                    "class": label,
                    "mode": mode,
                    "count": len(results),
                    "min_cycles": min(cycles),
                    "median_cycles": statistics.median(cycles),
                    "max_cycles": max(cycles),
                    "mean_cycles": statistics.mean(cycles),
                    "peak_stack_bytes": max(
                        result["peak_stack_bytes"] for result in results
                    ),
                }
            )
        helper_size = program.size("ROM0", {"Audit helper"})
        caller_sizes = {
            mode: program.symbols[f"audit_{mode.replace('-', '_')}_done"]
            - program.symbols[f"audit_{mode.replace('-', '_')}_start"]
            for mode in modes_for(variant)
            if mode != "body"
        }
        report["results"].append(
            {
                "variant": variant,
                "code_bytes": helper_size,
                "table_bytes": program.size("ROM0", {"Audit table"}),
                "table_alignment": 16
                if variant == "nibble"
                else 256
                if variant in ("byte", "nibble-page")
                else 1,
                "scratch_ram_bytes": 0,
                "harness_input_bytes": 2,
                "caller_bytes": caller_sizes,
                "caller_total_rom_bytes": {
                    mode: size
                    + (helper_size if mode.startswith("call") else 0)
                    + program.size("ROM0", {"Audit table"})
                    for mode, size in caller_sizes.items()
                },
                "body_observed_changes": sorted(changes["body"]),
                "observed_changes_by_mode": {
                    mode: sorted(regs) for mode, regs in changes.items()
                },
                "provenance": program.provenance,
                "validated_executions": executions,
                "errors": errors[:8],
                "error_count": len(errors),
                "exhaustive16": exhaustive,
                "summary": summary,
            }
        )
        print(
            f"{variant}: {executions} executions, {len(errors)} incorrect", flush=True
        )
    return report


def write_csv(report, path):
    text = io.StringIO()
    fields = [
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
    writer = csv.DictWriter(text, fieldnames=fields, lineterminator="\n")
    writer.writeheader()
    for candidate in report["results"]:
        for row in candidate["summary"]:
            writer.writerow(
                {
                    "variant": candidate["variant"],
                    "code_bytes": candidate["code_bytes"],
                    "table_bytes": candidate["table_bytes"],
                    **row,
                }
            )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.getvalue())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--variant", choices=VARIANTS, action="append")
    parser.add_argument("--samples", type=int, default=256)
    parser.add_argument("--seed", type=int, default=0)
    parser.add_argument("--exhaustive16", action="store_true")
    parser.add_argument(
        "--artifacts", type=Path, default=ROOT / "build/popcount-audit/artifacts"
    )
    parser.add_argument(
        "--output", type=Path, default=ROOT / "build/popcount-audit/results.json"
    )
    parser.add_argument("--csv", type=Path)
    args = parser.parse_args()
    if args.samples < 1:
        parser.error("--samples must be positive")
    report = audit(
        args.variant or VARIANTS,
        samples=args.samples,
        seed=args.seed,
        exhaustive16=args.exhaustive16,
        artifacts=args.artifacts,
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    if args.csv:
        write_csv(report, args.csv)
    return (
        0
        if all(
            not candidate["error_count"] and candidate["exhaustive16"]["correct"]
            for candidate in report["results"]
        )
        else 1
    )


if __name__ == "__main__":
    raise SystemExit(main())
