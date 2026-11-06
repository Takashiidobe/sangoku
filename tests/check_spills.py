import argparse
import hashlib
import json
import os
import random
import re
import subprocess
from pathlib import Path

from audit_popcount import run_case
from emu import Emulator
from runtime_harness import ROOT, assemble


def build(directory):
    commands = []

    def run(name, command, *, env=None):
        commands.append(command)
        with (directory / f"{name}.log").open("w") as log:
            subprocess.run(
                command, cwd=ROOT, env=env, stdout=log, stderr=log, check=True
            )

    env = dict(os.environ, SANGOKU_SPILL_ARTIFACTS=str(directory / "instructions"))
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
        env=env,
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
    for name, source in (
        ("popcount", "tests/fixtures/popcount/baseline.rs"),
        ("live", "tests/fixtures/spills/live.rs"),
    ):
        run(
            name,
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
                "-Copt-level=2",
                source,
                "-o",
                str(directory / f"{name}.asm"),
            ],
        )
    return commands


def check_accesses(directory):
    executions = 0
    registers = "bcdehl_a"
    for path in sorted((directory / "instructions").glob("*_*.bin")):
        operation, register, offset = path.stem.split("_")
        register, offset = registers[int(register)], int(offset)
        code = path.read_bytes()
        source = path.with_suffix(".asm").read_text()
        program = assemble(
            {"access.asm": 'SECTION "Access", ROM0[0]\n' + source},
            artifacts=directory / "assembled" / path.stem,
        )
        assert program.rom[: len(code)] == code, path
        assert program.size("ROM0") == len(code), path
        for stack in (0xC800, 0xD400):
            for seed in (0, 0x55, 0xFF):
                for flags in range(0, 256, 16):
                    emu = Emulator(code)
                    for index, reg in enumerate("abcdehl"):
                        setattr(emu, reg, (seed + index * 31) & 255)
                    emu.f, emu.sp = flags, stack
                    address = stack + offset
                    value = (seed * 73 + flags + offset) & 255
                    emu.memory[address] = value
                    expected = {r: getattr(emu, r) for r in "abcdefhl"}
                    if operation == "load":
                        expected[register] = value
                    else:
                        value = expected[register]
                    emu.reset_metrics()
                    emu.run_until(len(code), max_instructions=1024)
                    actual = {r: getattr(emu, r) for r in expected}
                    assert actual == expected, (path, stack, seed, flags, actual)
                    assert emu.sp == stack and emu.peak_stack_bytes == 4, path
                    assert emu.memory[address] == value, path
                    allowed = set(range(stack - 4, stack))
                    if operation == "store":
                        allowed.add(address)
                    assert emu.written_addresses <= allowed, path
                    executions += 1
    assert executions == 12096, executions
    return executions


def check_pressure(directory):
    executions = 0
    for name in ("pressure", "incoming"):
        code = (directory / "instructions" / f"{name}.bin").read_bytes()
        for stack in (0xC800, 0xD400):
            for value in range(256):
                emu = Emulator(code)
                for index, reg in enumerate("abcdehl"):
                    setattr(emu, reg, (value + index * 31) & 255)
                emu.f, emu.sp = value & 0xF0, stack - 2
                emu.write16(emu.sp, 0x800)
                emu.memory[stack : stack + 5] = bytes([value ^ 255] * 4 + [value])
                emu.reset_metrics()
                emu.run_until(0x800, max_instructions=2048)
                assert emu.a == (78 if name == "pressure" else value), (
                    name,
                    value,
                    emu.a,
                )
                assert emu.sp == stack, name
                assert emu.read16(stack - 2) == 0x800, name
                assert emu.memory[stack + 4] == value, name
                assert all(
                    stack - 2 - emu.peak_stack_bytes <= address < stack - 2
                    for address in emu.written_addresses
                ), name
                executions += 1
    return executions


def check_rust(directory, exhaustive):
    rng = random.Random(7)
    values = sorted(
        set(range(256))
        | {value << 8 for value in range(256)}
        | {0xFFFF, 0x5555, 0xAAAA, 0x8001, 0x1234}
        | {rng.randrange(65536) for _ in range(1024)}
    )
    summary = {}
    for name, symbol in (
        ("popcount", "__rust_audit_popcount"),
        ("live", "__rust_audit_popcount_live"),
    ):
        source = (directory / f"{name}.asm").read_text()
        source = re.sub(
            r'SECTION "[^"]+", ROM0',
            'SECTION "Audit helper", ROM0[$400]',
            source,
            count=1,
        ).replace(symbol, "audit_popcount")
        program = assemble(
            {
                "function.asm": source,
                "stop.asm": 'SECTION "Stop", ROM0[$100]\naudit_call_done::\n nop\n',
            },
            artifacts=directory / name,
        )
        executions = 0
        max_cycles = max_stack = 0
        for stack in (0xC800, 0xD000, 0xD002):
            for poison in (0, 0xA5, 0xFF):
                for value in values:
                    result = run_case(
                        program, name, value, poison=poison, stack_base=stack
                    )
                    expected = value.bit_count()
                    if name == "live":
                        expected = (value + expected) & 65535
                    assert result["actual"] == expected, (name, value, stack, result)
                    assert result["writes_in_stack"] and result["stack_restored"]
                    max_cycles = max(max_cycles, result["cycles"])
                    max_stack = max(max_stack, result["peak_stack_bytes"])
                    executions += 1
        if exhaustive:
            for value in range(65536):
                result = run_case(program, name, value)
                expected = value.bit_count()
                if name == "live":
                    expected = (value + expected) & 65535
                assert result["actual"] == expected, (name, value, result)
                assert result["writes_in_stack"] and result["stack_restored"]
                executions += 1
        summary[name] = {
            "executions": executions,
            "code_bytes": program.size("ROM0", {"Audit helper"}),
            "max_cycles": max_cycles,
            "peak_stack_bytes": max_stack,
            "provenance": program.provenance,
        }
    return summary


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--exhaustive16", action="store_true")
    parser.add_argument("--output", type=Path, default=ROOT / "build/spill-repair")
    args = parser.parse_args()
    directory = args.output.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    commands = [] if args.skip_build else build(directory)
    result = {
        "commands": commands,
        "access_executions": check_accesses(directory),
        "pressure_executions": check_pressure(directory),
        "rust": check_rust(directory, args.exhaustive16),
        "sha256": {
            str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in (
                ROOT / "tests/check_spills.py",
                ROOT / "tests/fixtures/spills/live.rs",
                ROOT
                / "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/emit.rs",
                ROOT
                / "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/mod.rs",
                ROOT
                / "third_party/wasmtime/cranelift/codegen/src/isa/lr35902/inst/regs.rs",
            )
        },
    }
    (directory / "results.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
