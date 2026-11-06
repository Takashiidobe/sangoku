import argparse
import csv
import io
import json
import random
import statistics
import subprocess
from collections import defaultdict
from pathlib import Path

from audit_runtime import execute
from runtime_harness import ROOT, assemble

FIXTURES = ROOT / "tests/fixtures/division"
VARIANTS = (
    "rolled",
    "unroll4",
    "unroll16",
    "early",
    "sdcc-sm83",
    "sdcc-z80",
    "sdcc-z80-early",
)
BOUNDARIES = (
    0,
    1,
    2,
    3,
    126,
    127,
    128,
    129,
    254,
    255,
    256,
    257,
    32767,
    32768,
    32769,
    65534,
    65535,
)
OPERATIONS = ("q", "r", "both", "separate")


def source_for(variant):
    if variant in ("raw", "raw-alt"):
        path = "baseline.asm" if variant == "raw" else "baseline-alt.asm"
        return (
            (FIXTURES / path)
            .read_text()
            .replace("ROM0", "ROM0[$400]", 1)
            .replace("__gbc_udiv16::", "audit_q::\n__gbc_udiv16::")
            .replace("__gbc_urem16::", "audit_r::\n__gbc_urem16::")
        )
    if variant in ("sdcc-sm83", "sdcc-z80", "sdcc-z80-early"):
        source = (FIXTURES / f"{variant.removesuffix('-early')}.asm").read_text()
        if variant.endswith("-early"):
            source = "DEF AUDIT_EARLY EQU 1\n" + source
    else:
        unroll = {"rolled": 1, "unroll4": 4, "unroll16": 16, "early": 1}[variant]
        source = f"DEF AUDIT_UNROLL EQU {unroll}\n"
        if variant == "early":
            source += "DEF AUDIT_EARLY EQU 1\n"
        source += (FIXTURES / "restoring.asm").read_text()
    source = source.replace(
        'INCLUDE "tests/fixtures/division/wrappers.asm"',
        (FIXTURES / "wrappers.asm").read_text(),
    )
    return source.replace(
        'INCLUDE "tests/fixtures/division/early.asm"',
        (FIXTURES / "early.asm").read_text(),
    )


def quotient_remainder_size(variant, *, artifacts=None):
    wrappers = (FIXTURES / "wrappers.asm").read_text()
    combined = wrappers[wrappers.index("audit_both::") :]
    source = source_for(variant).replace(combined, "")
    return assemble({"division.asm": source}, artifacts=artifacts).size(
        "ROM0", {"Audit division"}
    )


def setup_call(entry):
    return f"""    ld hl, audit_inputs
    ld b, [hl]
    inc hl
    ld c, [hl]
    inc hl
    ld a, [hl]
    ld e, a
    inc hl
    ld a, [hl]
    add sp, -2
    ld hl, sp + 0
    ld [hl], a
    ld a, e
    call {entry}
    add sp, 2
"""


def build(variant, *, artifacts=None):
    operations = ("q", "r") if variant in ("raw", "raw-alt") else OPERATIONS
    caller = 'SECTION "Audit callers", ROM0[$100]\n'
    for operation in operations:
        caller += f"audit_{operation}_start::\n"
        if operation == "separate":
            caller += setup_call("audit_q") + "    ld c, a\n    push bc\n"
            caller += setup_call("audit_r")
            caller += "    ld c, a\n    ld d, b\n    pop hl\n    ld a, l\n    ld b, h\n"
        else:
            caller += setup_call(f"audit_{operation}")
        caller += f"audit_{operation}_done::\n"
    caller += 'SECTION "Audit inputs", WRAM0[$c100]\naudit_inputs::\n    ds 4\n'
    return assemble(
        {"division.asm": source_for(variant), "callers.asm": caller},
        artifacts=artifacts,
    )


def run_case(program, operation, lhs, rhs, *, body=False, poison=0x55):
    if body and operation == "separate":
        raise ValueError("separate calls have no single helper body")

    def initialize(emu):
        address = program.symbols["audit_inputs"]
        emu.memory[address : address + 4] = lhs.to_bytes(2, "little") + rhs.to_bytes(
            2, "little"
        )
        emu.d, emu.e, emu.hl = poison, poison ^ 255, (poison << 8) | (poison ^ 255)

    entry = f"audit_{operation}" if body else f"audit_{operation}_start"
    result = execute(
        program,
        entry,
        lhs=lhs,
        rhs=rhs,
        body=body,
        initialize=initialize,
        stop_symbol=f"audit_{operation}_done",
    )
    regs = result["registers"]
    value = regs["a"] | regs["b"] << 8
    actual = (
        (value, regs["c"] | regs["d"] << 8)
        if operation in ("both", "separate")
        else value
    )
    expected = (
        None
        if rhs == 0
        else (
            lhs // rhs
            if operation == "q"
            else lhs % rhs
            if operation == "r"
            else (lhs // rhs, lhs % rhs)
        )
    )
    base = 0xD000 - result["peak_stack_bytes"]
    writes_in_stack = all(
        base <= address < 0xD000 for address in result["written_addresses"]
    )
    correct = (
        result["status"] == "rst_10"
        if rhs == 0
        else (
            result["status"] == "returned"
            and result["stack_restored"]
            and actual == expected
            and writes_in_stack
        )
    )
    if correct and rhs and operation in ("both", "separate"):
        correct = lhs == rhs * actual[0] + actual[1] and actual[1] < rhs
    result.update(
        {
            "actual": actual,
            "expected": expected,
            "correct": correct,
            "writes_in_stack": writes_in_stack,
        }
    )
    return result


def input_classes(samples, seed):
    rng = random.Random(seed)
    return {
        "boundary": [(lhs, rhs) for lhs in BOUNDARIES for rhs in BOUNDARIES if rhs],
        "divisor-one": [(lhs, 1) for lhs in BOUNDARIES],
        "equal": [(value, value) for value in BOUNDARIES if value],
        "power-of-two": [(lhs, 1 << bit) for lhs in BOUNDARIES for bit in range(16)],
        "less-than": [
            (rng.randrange(rhs), rhs)
            for rhs in [rng.randrange(1, 65536) for _ in range(samples)]
        ],
        "narrow-random": [
            (rng.randrange(65536), rng.randrange(1, 128)) for _ in range(samples)
        ],
        "wide-random": [
            (rng.randrange(65536), rng.randrange(128, 65536)) for _ in range(samples)
        ],
    }


def audit(variants, *, samples, seed, exhaustive8=False, artifacts=None):
    classes = input_classes(samples, seed)
    report = {
        "timing_unit": "CPU T-cycles",
        "seed": seed,
        "samples_per_random_class": samples,
        "input_classes": {name: pairs for name, pairs in classes.items()},
        "upstream": json.loads((FIXTURES / "provenance.json").read_text()),
        "results": [],
        "baselines": {},
        "limitations": [
            "flat-memory SM83 subset, no interrupts/peripherals/bank switching/bus timing",
            "hand-written callers reload operands from a 4-byte harness-owned WRAM input block",
            "all code sizes include quotient, remainder and combined wrappers plus the core",
            "combined result quotient A/B and remainder C/D; separate calls preserve quotient on stack",
            "body measurements include RET and a preexisting 4-byte caller frame",
            "no 32-bit core reproducer: generic SDCC 32-bit C inspected but not benchmarked",
        ],
    }
    for variant in ("raw", "raw-alt"):
        directory = artifacts / variant if artifacts else None
        try:
            program = build(variant, artifacts=directory)
            failures = []
            for operation in ("q", "r"):
                for lhs, rhs in ((1, 1), (0x1234, 16), (65535, 0), (0, 65535)):
                    for poison in (0, 0x55):
                        result = run_case(
                            program, operation, lhs, rhs, body=True, poison=poison
                        )
                        failures.append(
                            {
                                "operation": operation,
                                "lhs": lhs,
                                "rhs": rhs,
                                "poison": poison,
                                "actual": result["actual"],
                                "expected": result["expected"],
                                "correct": result["correct"],
                                "cycles": result["cycles"],
                            }
                        )
            report["baselines"][variant] = {
                "code_bytes": program.size("ROM0", {"Unsigned 16-bit division"}),
                "cases": failures,
                "provenance": program.provenance,
            }
        except subprocess.CalledProcessError as error:
            report["baselines"][variant] = {
                "build_error": str(error),
                "stderr": error.stderr,
                "artifacts": str(directory),
            }
    for variant in variants:
        print(f"auditing {variant}", flush=True)
        program = build(variant, artifacts=artifacts / variant if artifacts else None)
        groups = defaultdict(list)
        errors = []
        cases = 0
        changed = set()
        for label, pairs in classes.items():
            for lhs, rhs in pairs:
                for operation in OPERATIONS:
                    for body in (True, False) if operation != "separate" else (False,):
                        result = run_case(program, operation, lhs, rhs, body=body)
                        cases += 1
                        changed.update(result["observed_register_changes"])
                        if not result["correct"]:
                            errors.append(
                                {
                                    "class": label,
                                    "operation": operation,
                                    "body": body,
                                    "lhs": lhs,
                                    "rhs": rhs,
                                    "result": result,
                                }
                            )
                        groups[(label, operation, "body" if body else "caller")].append(
                            result
                        )
        traps = []
        for lhs in (0, 1, 32768, 65535):
            for operation in OPERATIONS:
                result = run_case(program, operation, lhs, 0)
                traps.append(
                    {
                        "lhs": lhs,
                        "operation": operation,
                        "correct": result["correct"],
                        "cycles": result["cycles"],
                        "peak_stack_bytes": result["peak_stack_bytes"],
                    }
                )
        exhaustive = {"executed": 0, "correct": True}
        if exhaustive8 and variant in ("sdcc-z80", "sdcc-z80-early"):
            print(
                "validating all 8-bit operand pairs through combined ABI entry",
                flush=True,
            )
            for lhs in range(256):
                for rhs in range(256):
                    result = run_case(program, "both", lhs, rhs, body=True)
                    exhaustive["executed"] += 1
                    exhaustive["correct"] &= result["correct"]
        summary = []
        for (label, operation, mode), values in groups.items():
            cycles = [value["cycles"] for value in values]
            summary.append(
                {
                    "class": label,
                    "operation": operation,
                    "mode": mode,
                    "count": len(values),
                    "min_cycles": min(cycles),
                    "median_cycles": statistics.median(cycles),
                    "max_cycles": max(cycles),
                    "mean_cycles": statistics.mean(cycles),
                    "peak_stack_bytes": max(
                        value["peak_stack_bytes"] for value in values
                    ),
                }
            )
        report["results"].append(
            {
                "variant": variant,
                "code_bytes": program.size("ROM0", {"Audit division"}),
                "quotient_remainder_code_bytes": quotient_remainder_size(
                    variant,
                    artifacts=artifacts / variant / "q-r-only" if artifacts else None,
                ),
                "core_bytes": program.sections["Audit division"].address
                + program.sections["Audit division"].size
                - program.symbols["audit_core"],
                "table_bytes": 0,
                "scratch_ram_bytes": 0,
                "harness_input_bytes": 4,
                "caller_bytes": {
                    operation: program.symbols[f"audit_{operation}_done"]
                    - program.symbols[f"audit_{operation}_start"]
                    for operation in OPERATIONS
                },
                "allowed_clobbers": list("abcdefhl"),
                "observed_changes": sorted(changed),
                "provenance": program.provenance,
                "validated_executions": cases,
                "errors": errors[:8],
                "error_count": len(errors),
                "zero_divisor": traps,
                "exhaustive8": exhaustive,
                "summary": summary,
            }
        )
        print(f"{variant}: {cases} executions, {len(errors)} incorrect", flush=True)
    return report


def write_csv(report, path):
    output = io.StringIO()
    fields = [
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
    writer = csv.DictWriter(output, fieldnames=fields, lineterminator="\n")
    writer.writeheader()
    for variant in report["results"]:
        for summary in variant["summary"]:
            writer.writerow(
                {
                    "variant": variant["variant"],
                    "code_bytes": variant["code_bytes"],
                    "quotient_remainder_code_bytes": variant[
                        "quotient_remainder_code_bytes"
                    ],
                    **summary,
                }
            )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(output.getvalue())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--variant", action="append", choices=VARIANTS)
    parser.add_argument("--samples", type=int, default=256)
    parser.add_argument("--seed", type=int, default=0)
    parser.add_argument("--exhaustive8", action="store_true")
    parser.add_argument("--artifacts", type=Path, default=ROOT / "build/division-audit")
    parser.add_argument(
        "--output", type=Path, default=ROOT / "build/division-audit.json"
    )
    parser.add_argument("--csv", type=Path)
    args = parser.parse_args()
    if args.samples < 1:
        parser.error("--samples must be positive")
    report = audit(
        args.variant or VARIANTS,
        samples=args.samples,
        seed=args.seed,
        exhaustive8=args.exhaustive8,
        artifacts=args.artifacts,
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    if args.csv:
        write_csv(report, args.csv)
    return (
        0
        if all(
            not result["error_count"]
            and all(t["correct"] for t in result["zero_divisor"])
            and result["exhaustive8"]["correct"]
            for result in report["results"]
        )
        else 1
    )


if __name__ == "__main__":
    raise SystemExit(main())
