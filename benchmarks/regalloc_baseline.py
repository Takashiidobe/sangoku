#!/usr/bin/env python3
"""Compare Rust (this backend) and SDCC sm83 code for the kernels in benchmarks/regalloc."""

import argparse
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
KERNELS = ROOT / "benchmarks/regalloc"
BASELINE = ROOT / "benchmarks/regalloc_baseline.json"
REG8 = "abcdehl"
METRICS = ["bytes", "insns", "moves", "stack", "jp"]
MOVE = re.compile(rf"^ld\s+([{REG8}]),\s*([{REG8}])$")
STACK = re.compile(r"\bsp\b|^(push|pop)\b")
JP = re.compile(r"^jp\s+(?!\(hl\))")
SDCC_LABEL = re.compile(r"^\s+[0-9A-F]{8}\s+\d+\s+_(\w+)::")
SDCC_BYTES = re.compile(r"^\s+[0-9A-F]{8}\s+((?:[0-9A-F]{2} ?)+?)\s+\[\s*\d+\]")
SECTION = re.compile(r'\(\$([0-9a-fA-F]+) bytes?\) \["__rust_(\w+)"\]')


def run(command, **kwargs):
    return subprocess.run(command, check=True, capture_output=True, text=True, **kwargs)


def count(lines):
    totals = dict.fromkeys(METRICS, 0)
    for line in lines:
        text = line.split(";")[0].strip()
        if not text or text.endswith(":") or text.startswith("db "):
            continue
        text = re.sub(r"\s+", " ", text)
        totals["insns"] += 1
        totals["moves"] += bool(MOVE.match(text))
        totals["stack"] += bool(STACK.search(text))
        totals["jp"] += bool(JP.match(text))
    return totals


def rust_functions(names, opt_level, work):
    source = KERNELS / "kernels.rs"
    asm = work / "rust.s"
    run(["cargo", "run", "-q", "--", "asm", source, asm, "--opt-level", opt_level], cwd=ROOT)
    text = asm.read_text()
    bodies = {}
    for name in names:
        match = re.search(rf'^SECTION "__rust_{name}".*?(?=^SECTION|\Z)', text, re.M | re.S)
        if match:
            bodies[name] = match.group(0).splitlines()[2:]
    defined = set(re.findall(r"^(\S+?)::?$", text, re.M))
    called = set(re.findall(r"^\s+(?:call|jp)\s+([A-Za-z_]\w*)\s*$", text, re.M))
    stubs = "".join(
        f'SECTION "stub_{s}", ROM0\n{s}::\n' for s in sorted(called - defined)
    )
    (work / "rust_stubs.s").write_text(text + stubs)
    run(["rgbasm", "-o", work / "rust.o", work / "rust_stubs.s"])
    run(["rgblink", "-m", work / "rust.map", "-o", work / "rust.gb", work / "rust.o"])
    sizes = {n: int(b, 16) for b, n in SECTION.findall((work / "rust.map").read_text())}
    return {n: {**count(body), "bytes": sizes[n]} for n, body in bodies.items()}


def sdcc_functions(work):
    source = KERNELS / "kernels.c"
    asm = work / "sdcc.s"
    run(["sdcc", "-msm83", "--opt-code-size", "-S", source, "-o", asm])
    run(["sdasgb", "-l", "-o", work / "sdcc.rel", asm])
    names = re.findall(r"^\s+\.globl\s+_(\w+)", asm.read_text(), re.M)
    bodies = {n: [] for n in names}
    current = None
    for line in asm.read_text().splitlines():
        label = re.match(r"^_(\w+)::", line)
        if label:
            current = label.group(1)
        elif current and re.match(r"^\t\.area", line):
            current = None
        elif current:
            bodies[current].append(line.lstrip("\t"))
    sizes, current = {}, None
    for line in (work / "sdcc.lst").read_text(errors="ignore").splitlines():
        label = SDCC_LABEL.match(line)
        if label:
            current = label.group(1)
            sizes[current] = 0
            continue
        data = SDCC_BYTES.match(line)
        if current and data:
            sizes[current] += len(data.group(1).split())
    return names, {n: {**count(b), "bytes": sizes[n]} for n, b in bodies.items()}


def collect(opt_level):
    with tempfile.TemporaryDirectory() as directory:
        work = Path(directory)
        names, sdcc = sdcc_functions(work)
        return {"rust": rust_functions(names, opt_level, work), "sdcc": sdcc}


def table(result):
    print(f"{'kernel':<10} {'':<5}" + "".join(f"{m:>7}" for m in METRICS) + "   bytes ratio")
    for name in sorted(result["sdcc"]):
        rust, sdcc = result["rust"].get(name), result["sdcc"][name]
        for label, row in (("rust", rust), ("sdcc", sdcc)):
            if row is None:
                print(f"{name:<10} {label:<5} missing")
                continue
            ratio = f"{rust['bytes'] / sdcc['bytes']:>10.2f}" if label == "rust" and rust else ""
            print(f"{name:<10} {label:<5}" + "".join(f"{row[m]:>7}" for m in METRICS) + ratio)
    rust_total = sum(r["bytes"] for r in result["rust"].values())
    sdcc_total = sum(result["sdcc"][n]["bytes"] for n in result["rust"])
    print(f"\ntotal bytes: rust {rust_total}, sdcc {sdcc_total}, ratio {rust_total / sdcc_total:.2f}")


def compare(result, baseline):
    worse = False
    for name, row in result["rust"].items():
        old = baseline["rust"].get(name)
        if old is None:
            print(f"{name}: new kernel")
            continue
        for metric in METRICS:
            if row[metric] != old[metric]:
                worse |= metric == "bytes" and row[metric] > old[metric]
                print(f"{name}: {metric} {old[metric]} -> {row[metric]}")
    return worse


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--opt-level", default="2")
    parser.add_argument("--update", action="store_true", help="rewrite the stored baseline")
    parser.add_argument("--check", action="store_true", help="diff against the baseline; fail if bytes grow")
    args = parser.parse_args()
    result = collect(args.opt_level)
    table(result)
    if args.update:
        BASELINE.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
        print(f"wrote {BASELINE.relative_to(ROOT)}")
    elif args.check:
        print("\nchanges against baseline:")
        sys.exit(compare(result, json.loads(BASELINE.read_text())))


if __name__ == "__main__":
    main()
