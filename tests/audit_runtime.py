import argparse
import json
from pathlib import Path

from runtime_harness import ROOT, assemble

STACK_FIXTURE = """SECTION "Audit helper", ROM0[$200]
audit_fixture::
    push bc
    ld hl, audit_table
    ld a, [hl]
    ld [audit_scratch], a
    call audit_inner
    pop bc
    ret
audit_inner:
    push af
    ld a, 9
    sub 4
    sbc 0
    cp 5
    pop af
    ret
SECTION "Audit table", ROM0[$400]
audit_table:
    db 42, 99
SECTION "Audit scratch", WRAM0[$c000]
audit_scratch:
    ds 4
"""


def caller(entry, lhs, rhs):
    return f"""SECTION "Audit caller", ROM0[$100]
audit_start::
    ld b, {lhs & 255}
    ld c, {lhs >> 8}
    ld a, {rhs >> 8}
    add sp, -2
    ld hl, sp + 0
    ld [hl], a
    ld a, {rhs & 255}
    call {entry}
    add sp, 2
audit_done::
"""


def execute(program, entry, *, lhs, rhs, body):
    emu = program.emulator(entry)
    emu.d, emu.e, emu.hl, emu.f = 0x55, 0xAA, 0x1234, 0xF0
    emu.sp = 0xD000
    entry_stack = 0
    if body:
        emu.b, emu.c, emu.a = lhs & 255, lhs >> 8, rhs & 255
        emu.sp -= 4
        emu.write16(emu.sp, program.symbols["audit_done"])
        emu.memory[emu.sp + 2] = rhs >> 8
        entry_stack = 4
    initial = {name: getattr(emu, name) for name in "abcdefhl"}
    emu.reset_metrics()
    stop = program.symbols["audit_done"]
    for _ in range(1000):
        if emu.pc in (stop, 0x10):
            break
        emu.step()
    if emu.pc == stop:
        status = "returned"
    elif emu.pc == 0x10:
        status = "rst_10"
    else:
        raise TimeoutError(f"{entry} exceeded 1000 instructions at {emu.pc:04X}")
    scratch = [
        section for section in program.sections.values() if section.area == "WRAM0"
    ]
    scratch_written = {
        address
        for address in emu.written_addresses
        if any(s.address <= address < s.address + s.size for s in scratch)
    }
    return {
        "cycles": emu.cycles,
        "entry_stack_bytes": entry_stack,
        "peak_additional_stack_bytes": emu.peak_stack_bytes,
        "peak_stack_bytes": entry_stack + emu.peak_stack_bytes,
        "scratch_written_bytes": len(scratch_written),
        "status": status,
        "registers": {name: getattr(emu, name) for name in (*"abcdefhl", "sp", "pc")},
        "observed_register_changes": [
            name for name, value in initial.items() if getattr(emu, name) != value
        ],
        "stack_restored": status == "returned"
        and emu.sp == (0xCFFE if body else 0xD000),
    }


def measure(helper, lhs, rhs, *, artifacts=None):
    if not 0 <= lhs <= 65535 or not 0 <= rhs <= 65535:
        raise ValueError("operands must be unsigned 16-bit values")
    if helper == "stack-fixture":
        entry = "audit_fixture"
        source = STACK_FIXTURE
        expected = 42
    else:
        path = "mul16.asm" if helper == "mul16" else "udiv16.asm"
        source = (ROOT / "runtime" / path).read_text().replace("ROM0", "ROM0[$200]", 1)
        entry = "__gbc_" + helper
        expected = (
            (lhs * rhs) & 65535
            if helper == "mul16"
            else (None if rhs == 0 else lhs // rhs if helper == "udiv16" else lhs % rhs)
        )
    program = assemble(
        {"helper.asm": source, "caller.asm": caller(entry, lhs, rhs)},
        artifacts=artifacts,
    )
    sizes = {
        "helper_code_bytes": program.size("ROM0")
        - program.size("ROM0", {"Audit caller", "Audit table"}),
        "table_rom_bytes": program.size("ROM0", {"Audit table"}),
        "caller_code_bytes": program.size("ROM0", {"Audit caller"}),
        "scratch_ram_bytes": program.size("WRAM0"),
    }
    measurements = {}
    for mode, start in (("body", entry), ("caller", "audit_start")):
        result = execute(program, start, lhs=lhs, rhs=rhs, body=mode == "body")
        regs = result["registers"]
        actual = regs["a"] if helper == "stack-fixture" else regs["a"] | regs["b"] << 8
        correct = (
            (result["status"] == "rst_10")
            if expected is None
            else (
                result["status"] == "returned"
                and result["stack_restored"]
                and actual == expected
            )
        )
        result.update({"actual": actual, "correct": correct})
        measurements[mode] = result
    return {
        "helper": helper,
        "lhs": lhs,
        "rhs": rhs,
        "expected": expected,
        "sizes": sizes,
        "measurements": measurements,
        "provenance": program.provenance,
        "abi": (
            "stack fixture returns A and preserves BC; uses the same argument frame"
            if helper == "stack-fixture"
            else "lhs low/high B/C; rhs low/high A/[SP+2]; result low/high A/B"
        ),
        "input_class": (
            "zero-rhs"
            if rhs == 0
            else "zero-dividend"
            if lhs == 0
            else "unit-rhs"
            if rhs == 1
            else "narrow-rhs"
            if rhs < 256
            else "full-width-rhs"
        ),
        "allowed_clobbers": ["a", "f", "b", "c", "d", "e", "h", "l"],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--helper",
        action="append",
        choices=["mul16", "udiv16", "urem16", "stack-fixture"],
    )
    parser.add_argument("--lhs", type=lambda value: int(value, 0))
    parser.add_argument("--rhs", type=lambda value: int(value, 0))
    parser.add_argument("--output", type=Path)
    parser.add_argument("--artifacts", type=Path)
    args = parser.parse_args()
    if (args.lhs is None) != (args.rhs is None):
        parser.error("--lhs and --rhs must be supplied together")
    pairs = (
        [(args.lhs, args.rhs)]
        if args.lhs is not None
        else [(0, 1), (1, 1), (0x1234, 0x10), (0xFFFF, 0xFF), (0xFFFF, 0x8000)]
    )
    reports = []
    for helper in args.helper or ["mul16", "stack-fixture"]:
        for index, (lhs, rhs) in enumerate(pairs):
            artifacts = args.artifacts / helper / str(index) if args.artifacts else None
            reports.append(measure(helper, lhs, rhs, artifacts=artifacts))
    report = {
        "timing_unit": "CPU T-cycles",
        "limitations": [
            "flat ROM0/WRAM0 memory; instruction subset; no peripherals or interrupts",
            "synthetic ABI callers, not compiler-emitted callers",
            "body entry includes a 2-byte return address and a 2-byte argument slot",
            "scratch RAM is linked WRAM0 allocation; writes are counted separately",
            "observed changes do not establish a callee-saved register contract",
        ],
        "reports": reports,
    }
    text = json.dumps(report, indent=2) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(text)
    else:
        print(text, end="")
    return (
        0
        if all(m["correct"] for r in reports for m in r["measurements"].values())
        else 1
    )


if __name__ == "__main__":
    raise SystemExit(main())
