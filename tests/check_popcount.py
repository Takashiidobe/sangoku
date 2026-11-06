import argparse
import hashlib
import json
import os
import random
import re
import subprocess
from pathlib import Path

from audit_popcount import build as build_reference
from check_spills import check_accesses, check_pressure
from runtime_harness import ROOT, assemble

LEVELS = ("0", "1", "2", "3", "s", "z")
PROBES = ("popcount", "popcount_wide", "popcount_live", "popcount_pressure")


def build(directory):
    commands = []
    env = dict(
        os.environ,
        CG_CLIF_ENABLE_VERIFIER="1",
        SANGOKU_SPILL_ARTIFACTS=str(directory / "instructions"),
        SANGOKU_POPCOUNT_ARTIFACTS=str(directory / "kernels"),
    )

    def run(name, command):
        commands.append(command)
        with (directory / f"{name}.log").open("w") as log:
            subprocess.run(
                command, cwd=ROOT, env=env, stdout=log, stderr=log, check=True
            )

    run(
        "codegen-tests",
        [
            "cargo",
            "+gbc-lr35902",
            "test",
            "--locked",
            "--manifest-path",
            "third_party/wasmtime/cranelift/codegen/Cargo.toml",
            "--no-default-features",
            "--features",
            "std,lr35902,x86",
            "--lib",
            "isa::lr35902::",
        ],
    )
    run(
        "backend-build",
        [
            "cargo",
            "+gbc-lr35902",
            "build",
            "--release",
            "--locked",
            "--manifest-path",
            "third_party/rustc_codegen_cranelift/Cargo.toml",
        ],
    )
    for level in LEVELS:
        run(
            f"compile-{level}",
            [
                "rustc",
                "+gbc-lr35902",
                "-Zunstable-options",
                "-Zcodegen-backend=third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so",
                "--target",
                "targets/lr35902.json",
                "--emit=asm",
                "--crate-type=lib",
                "-Cpanic=abort",
                f"-Copt-level={level}",
                "tests/fixtures/popcount/replacement.rs",
                "-o",
                str(directory / f"level-{level}.asm"),
            ],
        )
    return commands


def kernels(directory):
    result = {}
    for name in ("shift", "swar"):
        program = build_reference(name)
        start = program.symbols["audit_popcount"]
        end = program.symbols["audit_popcount_end"]
        result[name] = program.rom[start : end - 1]
    for opt in ("none", "speed", "speed_and_size"):
        for size in (False, True):
            code = (
                directory / "kernels" / f"{opt}-{str(size).lower()}.bin"
            ).read_bytes()
            expected = result["shift" if size or opt == "none" else "swar"] + b"\xc9"
            assert code == expected, (opt, size)
    return result


def execute(program, probe, value, other, stack, poison):
    emu = program.emulator(f"__rust_audit_{probe}")
    for index, reg in enumerate("adehl"):
        setattr(emu, reg, (poison + index * 31) & 255)
    emu.b, emu.c = value & 255, value >> 8
    emu.a = other & 255 if probe == "popcount_pressure" else poison
    emu.f, emu.sp = poison & 0xF0, stack - 2
    emu.write16(emu.sp, program.symbols["audit_stop"])
    emu.memory[stack] = other >> 8
    emu.reset_metrics()
    emu.run_until(program.symbols["audit_stop"], max_instructions=4096)
    actual = emu.a | emu.b << 8
    expected = value.bit_count()
    if probe == "popcount_wide":
        actual |= emu.c << 16 | emu.d << 24
    elif probe == "popcount_live":
        expected = (value + expected) & 65535
    elif probe == "popcount_pressure":
        expected = (value + other + expected + other.bit_count()) & 65535
    assert actual == expected, (probe, value, other, stack, poison, actual, expected)
    assert emu.sp == stack, probe
    assert emu.memory[stack] == other >> 8, probe
    assert emu.read16(stack - 2) == program.symbols["audit_stop"], probe
    peak = 2 + emu.peak_stack_bytes
    assert all(
        stack - peak <= address < stack - 2 for address in emu.written_addresses
    ), probe
    return emu.cycles, peak


def check_level(directory, level, reference, exhaustive):
    source = (directory / f"level-{level}.asm").read_text()
    source = re.sub(
        r'SECTION "[^"]+", ROM0',
        'SECTION "Probes", ROM0[$400]',
        source,
        count=1,
    )
    source += "\naudit_program_end::\n"
    program = assemble(
        {
            "probes.asm": source,
            "stop.asm": 'SECTION "Stop", ROM0[$100]\naudit_stop::\n nop\n',
        },
        artifacts=directory / f"level-{level}",
    )
    entries = sorted(program.symbols[f"__rust_audit_{probe}"] for probe in PROBES)
    entries += [program.symbols["audit_program_end"]]
    rng = random.Random(11)
    cases = sorted(
        {(value, value ^ 65535) for value in range(256)}
        | {(value << 8, value) for value in range(256)}
        | {(0xFFFF, 0xFFFF), (0x5555, 0xAAAA), (0x8001, 0x1234)}
        | {(rng.randrange(65536), rng.randrange(65536)) for _ in range(1024)}
    )
    expected_kernel = reference["swar" if level in ("1", "2", "3") else "shift"]
    result = {}
    for probe in PROBES:
        print(f"checking opt={level} {probe}", flush=True)
        start = program.symbols[f"__rust_audit_{probe}"]
        end = entries[entries.index(start) + 1]
        body = program.rom[start:end]
        assert body.count(expected_kernel) == (2 if probe.endswith("pressure") else 1)
        cycles, stack = [], []
        executions = 0
        for base in (0xC800, 0xD000, 0xD002):
            for poison in (0, 0xA5, 0xFF):
                for value, other in cases:
                    cost, peak = execute(program, probe, value, other, base, poison)
                    cycles.append(cost)
                    stack.append(peak)
                    executions += 1
        if exhaustive:
            for value in range(65536):
                other = (40503 * value + 12345) & 65535
                cost, peak = execute(program, probe, value, other, 0xD000, 0x55)
                cycles.append(cost)
                stack.append(peak)
                executions += 1
        result[probe] = {
            "executions": executions,
            "code_bytes": end - start,
            "min_cycles": min(cycles),
            "max_cycles": max(cycles),
            "peak_stack_bytes": max(stack),
        }
    return {"probes": result, "provenance": program.provenance}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--exhaustive16", action="store_true")
    parser.add_argument(
        "--output", type=Path, default=ROOT / "build/popcount-replacement"
    )
    args = parser.parse_args()
    directory = args.output.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    commands = [] if args.skip_build else build(directory)
    reference = kernels(directory)
    result = {
        "commands": commands,
        "verifier_enabled": True,
        "seed": 11,
        "exhaustive16": args.exhaustive16,
        "access_executions": check_accesses(directory),
        "pressure_executions": check_pressure(directory),
        "levels": {
            level: check_level(directory, level, reference, args.exhaustive16)
            for level in LEVELS
        },
        "sha256": {
            str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in (
                ROOT / "tests/check_popcount.py",
                ROOT / "tests/fixtures/popcount/replacement.rs",
                ROOT / "third_party/rustc_codegen_cranelift/src/lib.rs",
                ROOT / "third_party/wasmtime/cranelift/codegen/meta/src/isa/lr35902.rs",
                *sorted(
                    (
                        ROOT / "third_party/wasmtime/cranelift/codegen/src/isa/lr35902"
                    ).rglob("*.rs")
                ),
            )
        },
    }
    (directory / "results.json").write_text(json.dumps(result, indent=2) + "\n")
    print("all optimization levels passed", flush=True)


if __name__ == "__main__":
    main()
