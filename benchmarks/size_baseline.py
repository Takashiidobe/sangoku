#!/usr/bin/env python3
"""Compare linked ROM bytes of the Rust examples with SDCC sm83 C equivalents."""

import collections
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PROGRAMS = {
    "empty": ROOT / "benchmarks/empty.rs",
    "hello": ROOT / "examples/hello.rs",
    "primes": ROOT / "examples/primes.rs",
    "sort": ROOT / "examples/sort.rs",
    "heap": ROOT / "examples/heap.rs",
}
CRATES = {
    "core": "core",
    "compiler_builtins": "builtins",
    "alloc": "alloc",
    "gbc_heap": "heap",
}
COLUMNS = ["asm", "user", "core", "builtins", "alloc", "heap"]
SECTION = re.compile(r'\(\$([0-9a-fA-F]+) bytes?\) \["([^"]*)"\]')
SDCC_AREA = re.compile(r"^(_\w+)\s+[0-9A-F]{8}\s+[0-9A-F]{8}\s*=\s*(\d+)\.\s*bytes", re.M)
SDCC_ROM_AREAS = {"_HOME", "_CODE", "_INITIALIZER", "_GSINIT", "_GSFINAL"}


def defining_crate(name):
    if not name.startswith("__rust"):
        return "asm"
    if name.startswith("__rust_core_"):
        return "core"
    match = re.search(r"C(?:s[0-9A-Za-z]*_)?(\d+)([A-Za-z_]\w*)", name[7:])
    if not match:
        return "user"
    return CRATES.get(match.group(2)[: int(match.group(1))], "user")


def rust_sizes(source, opt_level, out):
    flags = ["--alloc"] if source.stem == "heap" else []
    build = subprocess.run(
        ["cargo", "run", "-q", "--", "build", *flags, source, out, "--opt-level", opt_level],
        cwd=ROOT,
        capture_output=True,
    )
    build.check_returncode()
    maps = sorted(
        (ROOT / "build/artifacts").glob(f"{source.stem}-*/probe.map"),
        key=lambda path: path.stat().st_mtime,
    )
    sizes = collections.Counter()
    for size, name in SECTION.findall(maps[-1].read_text()):
        sizes[defining_crate(name)] += int(size, 16)
    return sizes


def sdcc_size(name, work):
    source = ROOT / "benchmarks/sdcc"
    subprocess.run(["sdasgb", "-o", work / "crt0.rel", source / "crt0.s"], check=True)
    subprocess.run(
        ["sdcc", "-msm83", "-c", source / f"{name}.c", "-o", work / f"{name}.rel"],
        check=True,
        capture_output=True,
    )
    subprocess.run(
        [
            "sdcc", "-msm83", "--no-std-crt0", "--code-loc", "0x0200",
            "--data-loc", "0xC000", work / "crt0.rel", work / f"{name}.rel",
            "-o", work / f"{name}.ihx",
        ],
        check=True,
        capture_output=True,
    )
    areas = SDCC_AREA.findall((work / f"{name}.map").read_text())
    return sum(int(size) for area, size in areas if area in SDCC_ROM_AREAS)


def main():
    levels = sys.argv[1:] or ["z", "2"]
    print("program  opt " + " ".join(f"{c:>8}" for c in COLUMNS) + "    total  sdcc")
    with tempfile.TemporaryDirectory() as directory:
        work = Path(directory)
        for name, source in PROGRAMS.items():
            sdcc = sdcc_size(name, work)
            for level in levels:
                sizes = rust_sizes(source, level, work / f"{name}-{level}.gb")
                row = " ".join(f"{sizes[c]:>8}" for c in COLUMNS)
                print(f"{name:<8} {level:>3} {row} {sum(sizes.values()):>8} {sdcc:>5}")


if __name__ == "__main__":
    main()
