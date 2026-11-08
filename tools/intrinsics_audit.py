#!/usr/bin/env python3
"""Probe integer and float intrinsics through Rust, CLIF, and LR35902 lowering.

Run with --check to compare tests/coverage/intrinsics.tsv, or --keep DIR to
retain generated source and CLIF. This checks compilation, not execution.
"""

import argparse
import os
import re
import subprocess
import tempfile
from pathlib import Path

from core_report import ROOT, rustc, symbols

OUT = ROOT / "tests/coverage/intrinsics.tsv"
INTS = [f"{sign}{bits}" for sign in ("u", "i") for bits in (8, 16, 32, 64, 128)] + [
    "usize",
    "isize",
]


def cases():
    groups = {"integer": {}}
    for ty in INTS:
        for name in ("ctpop", "ctlz", "cttz", "ctlz_nonzero", "cttz_nonzero"):
            groups["integer"][f"{name}_{ty}"] = (f"x: {ty}", "u32", f"{name}(x)")
        for name in ("bswap", "bitreverse"):
            groups["integer"][f"{name}_{ty}"] = (f"x: {ty}", ty, f"{name}(x)")
        for name in ("rotate_left", "rotate_right"):
            groups["integer"][f"{name}_{ty}"] = (f"x: {ty}, n: u32", ty, f"x.{name}(n)")
        for name in ("saturating_add", "saturating_sub", "integer_min", "integer_max"):
            groups["integer"][f"{name}_{ty}"] = (
                f"x: {ty}, y: {ty}",
                ty,
                f"{name}(x, y)",
            )
        for name in ("add_with_overflow", "sub_with_overflow", "mul_with_overflow"):
            groups["integer"][f"{name}_{ty}"] = (
                f"x: {ty}, y: {ty}",
                f"({ty}, bool)",
                f"{name}(x, y)",
            )
        if ty.startswith("u"):
            groups["integer"][f"carryless_mul_{ty}"] = (
                f"x: {ty}, y: {ty}",
                ty,
                "carryless_mul(x, y)",
            )
        for name in (
            ("unchecked_funnel_shl", "unchecked_funnel_shr")
            if ty.startswith("u")
            else ()
        ):
            groups["integer"][f"{name}_{ty}"] = (
                f"x: {ty}, y: {ty}, n: u32",
                ty,
                f"{name}(x, y, n)",
            )
        unsigned = ty.replace("i", "u", 1) if ty.startswith("i") else ty
        groups["integer"][f"carrying_mul_add_{ty}"] = (
            f"x: {ty}, y: {ty}, a: {ty}, c: {ty}",
            f"({unsigned}, {ty})",
            "carrying_mul_add(x, y, a, c)",
        )
    for ty in ("f16", "f32", "f64", "f128"):
        group = groups[ty] = {}
        for name in (
            "fabs",
            "sqrt",
            "floor",
            "ceil",
            "trunc",
            "round",
            "round_ties_even",
            "exp",
            "exp2",
            "log",
            "log2",
            "log10",
            "sin",
            "cos",
        ):
            group[f"{name}_{ty}"] = (f"x: {ty}", ty, f"{name}(x)")
        group[f"powi_{ty}"] = (f"x: {ty}, n: i32", ty, "powi(x, n)")
        for name in (
            "minimum",
            "maximum",
            "minimum_number_nsz_",
            "maximum_number_nsz_",
            "copysign",
        ):
            intrinsic = f"{name}{ty}"
            group[f"{intrinsic}"] = (f"x: {ty}, y: {ty}", ty, f"{intrinsic}(x, y)")
        for name in (
            "fadd_algebraic",
            "fsub_algebraic",
            "fmul_algebraic",
            "fdiv_algebraic",
            "frem_algebraic",
        ):
            group[f"{name}_{ty}"] = (f"x: {ty}, y: {ty}", ty, f"{name}(x, y)")
        group[f"powf_{ty}"] = (f"x: {ty}, y: {ty}", ty, "powf(x, y)")
        for name in ("fma", "fmuladd"):
            group[f"{name}_{ty}"] = (
                f"x: {ty}, y: {ty}, z: {ty}",
                ty,
                f"{name}(x, y, z)",
            )
    groups["memory"] = {
        "copy": ("src: *const u8, dst: *mut u8, n: usize", "()", "copy(src, dst, n)"),
        "copy_nonoverlapping": (
            "src: *const u8, dst: *mut u8, n: usize",
            "()",
            "copy_nonoverlapping(src, dst, n)",
        ),
        "write_bytes": (
            "dst: *mut u8, value: u8, n: usize",
            "()",
            "write_bytes(dst, value, n)",
        ),
        "compare_bytes": (
            "a: *const u8, b: *const u8, n: usize",
            "i32",
            "compare_bytes(a, b, n)",
        ),
        "raw_eq": ("a: &u32, b: &u32", "bool", "raw_eq(a, b)"),
    }
    return groups


def run(command):
    return subprocess.run(command, cwd=ROOT, capture_output=True, text=True)


def call_graph(assembly):
    graph = {}
    for match in re.finditer(
        r"^([A-Za-z_][A-Za-z0-9_]*)::(.*?)(?=^[A-Za-z_][A-Za-z0-9_]*::|\Z)",
        assembly,
        re.M | re.S,
    ):
        graph[match[1]] = set(
            re.findall(
                r"^\s*(?:call|jp)\s+(?:(?:nz|z|nc|c),\s*)?([A-Za-z_][A-Za-z0-9_]*)(?=\s*(?:;.*)?$)",
                match[2],
                re.M,
            )
        )
    return graph


def callees(graph, root):
    pending, visited, leaves = [root], set(), set()
    while pending:
        name = pending.pop()
        if name in visited:
            continue
        visited.add(name)
        if name in graph:
            pending.extend(graph[name])
        else:
            leaves.add(name)
    return sorted(leaves), visited & graph.keys()


def audit(directory):
    for command in (
        ["cargo", "build", "-q", "--bin", "sangoku"],
        ["cargo", "build", "-q", "--release", "-p", "sangoku-lower"],
    ):
        result = run(command)
        if result.returncode:
            raise RuntimeError(result.stderr)
    rows = []
    library_graph = None
    for group, functions in cases().items():
        source = directory / f"{group}.rs"
        clif = source.with_suffix(".clif")
        source.write_text(
            "#![no_std]\n#![feature(core_intrinsics, f16, f128, uint_carryless_mul)]\n#![allow(internal_features, unused_unsafe)]\nuse core::intrinsics::*;\n"
            + "".join(
                f"#[unsafe(no_mangle)]\npub fn audit_{name}({args}) -> {ret} {{ unsafe {{ {body} }} }}\n"
                for name, (args, ret, body) in functions.items()
            )
        )
        result = run(
            [str(ROOT / "target/debug/sangoku"), "clif", str(source), str(clif)]
        )
        (directory / f"{group}.stderr").write_text(result.stderr)
        if result.returncode:
            raise RuntimeError(f"{group} frontend failed:\n{result.stderr[-6000:]}")
        text = clif.read_text()
        libraries = ROOT / "build/core/immediate-abort/opt-0"
        if library_graph is None:
            library_graph = call_graph(
                "\n".join(
                    (libraries / name).read_text()
                    for name in ("core.s", "compiler_builtins.s")
                )
            )
        names = symbols(text)
        result = run([str(ROOT / "target/release/sangoku-lower"), "--probe", str(clif)])
        if result.returncode:
            raise RuntimeError(result.stderr)
        (directory / f"{group}.probe").write_text(result.stdout)
        exported = {}
        for line in result.stdout.splitlines():
            function, status, *message = line.split("\t")
            symbol = names[function]
            if symbol.startswith("audit_"):
                exported[symbol.removeprefix("audit_")] = (status, " ".join(message))
            elif status != "ok":
                rows.append((group, "fallback", status, " ".join(message)))
        for name in functions:
            status, message = exported[name]
            rows.append((name, "lowering", status, message))
        assembly = source.with_suffix(".s")
        result = rustc(
            [
                "-Cdebug-assertions=off",
                "-Zinline-mir=yes",
                "-Zmir-opt-level=2",
                "-Zmir-strip-debuginfo=all-locals",
                "-Zinline-mir-hint-threshold=50",
                "-L",
                f"dependency={libraries}",
                "--extern",
                f"core={libraries / 'libcore.rmeta'}",
                "--extern",
                f"compiler_builtins={libraries / 'libcompiler_builtins.rmeta'}",
            ],
            source,
            assembly,
            {
                key: value
                for key, value in os.environ.items()
                if key != "CG_CLIF_CLIF_ONLY"
            },
        )
        if result.returncode:
            raise RuntimeError(result.stderr)
        graph = library_graph | call_graph(assembly.read_text())
        for name in functions:
            leaves, reachable = callees(graph, f"__rust_audit_{name}")
            rows.append(
                (
                    name,
                    "callees",
                    "external" if leaves else "no-external-call",
                    ", ".join(leaves),
                )
            )
            rows.append(
                (name, "rust-functions", str(len(reachable)), "before-runtime-objects")
            )
    return "case\tphase\tstatus\tdetail\n" + "".join(
        "\t".join(row) + "\n" for row in sorted(rows)
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--keep", type=Path)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as scratch:
        directory = (args.keep or Path(scratch)).resolve()
        directory.mkdir(parents=True, exist_ok=True)
        output = audit(directory)
    if args.check:
        if not OUT.exists() or OUT.read_text() != output:
            raise SystemExit(
                "intrinsic audit changed; run tools/intrinsics_audit.py and review the diff"
            )
    else:
        OUT.write_text(output)
    print(f"{len(output.splitlines()) - 1} audit rows; {OUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
