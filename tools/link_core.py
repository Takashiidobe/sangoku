#!/usr/bin/env python3
"""Link a Rust program against real `core` into a Game Boy ROM.

Compiles core, compiler_builtins and the program to RGBDS assembly, keeps only
the sections reachable from `__rust_gbc_main`, then assembles and links them
with the runtime routines the kept code references.

Usage:
    tools/link_core.py PROGRAM.rs OUTPUT.gb [--keep DIR] [--stats]
"""
import argparse
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOOLCHAIN = "gbc-lr35902"
BACKEND = ROOT / "third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so"
CORE = ROOT / "third_party/rust/library/core/src/lib.rs"
BUILTINS = ROOT / "third_party/rust/library/compiler-builtins/compiler-builtins/src/lib.rs"
ENTRY = "__rust_gbc_main"
RUNTIME = ["udiv16", "mul16", "wide32", "softfloat32", "softfloat64", "softfloat128", "convert", "memory", "panic"]

SECTION = re.compile(r'^SECTION\s+"([^"]+)"', re.M)
LABEL = re.compile(r"^([A-Za-z_][\w$#@.]*)::", re.M)
TOKEN = re.compile(r"[A-Za-z_][\w$#@]*")


def run(cmd, **kwargs):
    result = subprocess.run([str(c) for c in cmd], cwd=ROOT, capture_output=True, text=True, **kwargs)
    if result.returncode != 0:
        sys.exit(f"{' '.join(map(str, cmd))} failed:\n{result.stderr[-4000:]}")
    return result


def compile_asm(source, name, out, deps, extra):
    asm = out / f"{name}.s"
    run(["rustc", f"+{TOOLCHAIN}", "--edition=2024", "--crate-type=rlib",
         "--target", ROOT / "targets/lr35902.json", "-Zunstable-options",
         "-Cpanic=abort", "-Copt-level=0", "-Awarnings", f"-Zcodegen-backend={BACKEND}",
         "--emit=asm,metadata", "--crate-name", name, *extra, *deps, source, "-o", asm])
    return asm.read_text()


def split_sections(text):
    starts = [m.start() for m in SECTION.finditer(text)]
    for begin, end in zip(starts, starts[1:] + [len(text)]):
        yield text[begin:end]


def prune(chunks, roots):
    defined = {}
    for index, chunk in enumerate(chunks):
        for label in LABEL.findall(chunk):
            defined[label] = index
    kept, pending = set(), [defined[r] for r in roots if r in defined]
    while pending:
        index = pending.pop()
        if index in kept:
            continue
        kept.add(index)
        for token in TOKEN.findall(chunks[index]):
            target = defined.get(token)
            if target is not None and target not in kept:
                pending.append(target)
    return [chunks[i] for i in sorted(kept)]


def section_sizes(map_text):
    return {m.group(2): int(m.group(1), 16)
            for m in re.finditer(r'SECTION: \$[0-9a-f]+-\$[0-9a-f]+ \(\$([0-9a-f]+) bytes\) \["([^"]+)"\]', map_text)}


def place(chunks, sizes, budget):
    """Fill ROM0 first-come, spill the rest to ROMX; a 32 KiB cartridge maps both at once."""
    placed, used = [], 0
    for chunk in chunks:
        name = SECTION.match(chunk).group(1)
        size = sizes[name]
        if used + size <= budget:
            used += size
            placed.append(chunk)
        else:
            placed.append(re.sub(r'^(SECTION "[^"]+", )ROM0', r"\1ROMX", chunk, count=1))
    return placed


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("program", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--keep", type=Path)
    parser.add_argument("--stats", action="store_true")
    args = parser.parse_args()

    with tempfile.TemporaryDirectory() as scratch:
        out = args.keep or Path(scratch)
        out.mkdir(parents=True, exist_ok=True)
        texts = [compile_asm(CORE, "core", out, [], [])]
        deps = ["--extern", f"core={out / 'libcore.rmeta'}", "-L", f"dependency={out}"]
        texts.append(compile_asm(
            BUILTINS, "compiler_builtins", out, deps,
            ["--cfg", 'feature="compiler-builtins"', "--cfg", 'feature="unmangled-names"',
             "--cfg", 'feature="mem"']))
        deps += ["--extern", f"compiler_builtins={out / 'libcompiler_builtins.rmeta'}"]
        texts.append(compile_asm(args.program.resolve(), "program", out, deps, []))

        chunks = [chunk for text in texts for chunk in split_sections(text)]
        kept = prune(chunks, [ENTRY])
        if args.stats:
            print(f"sections: {len(kept)}/{len(chunks)} kept")
        program_asm = "\n".join(kept)

        run(["rgbasm", "-o", out / "header.o", ROOT / "rom/header.asm"])
        runtime = []
        needs = {
            "softfloat32": "__gbc_f", "softfloat64": "__gbc_f64", "softfloat128": "__gbc_f128",
            "convert": "__gbc_convert", "memory": ("__gbc_mem", "memcpy", "memmove"),
            "panic": "rust_begin_unwind",
        }
        for name in RUNTIME:
            wanted = needs.get(name)
            if wanted:
                wanted = (wanted,) if isinstance(wanted, str) else wanted
                text = program_asm
                if name == "softfloat32":
                    text = text.replace("__gbc_f64", "").replace("__gbc_f128", "")
                if not any(w in text for w in wanted):
                    continue
            run(["rgbasm", "-o", out / f"{name}.o", ROOT / f"runtime/{name}.asm"])
            runtime.append(out / f"{name}.o")
        fixed = [out / "header.o", *runtime]

        everything_banked = program_asm.replace(", ROM0", ", ROMX")
        (out / "probe.s").write_text(everything_banked)
        run(["rgbasm", "-o", out / "probe.o", out / "probe.s"])
        run(["rgblink", "-m", out / "probe.map", "-o", out / "probe.gb", *fixed, out / "probe.o"])
        map_text = (out / "probe.map").read_text()
        free = int(re.search(r"ROM0: \d+ bytes used / (\d+) free", map_text).group(1))
        sizes = section_sizes(map_text)
        if args.stats:
            print(f"code and data: {sum(sizes[SECTION.match(c).group(1)] for c in kept)} bytes, ROM0 free: {free}")
        (out / "linked.s").write_text("\n".join(place(kept, sizes, free)))
        run(["rgbasm", "-o", out / "program.o", out / "linked.s"])
        args.output.parent.mkdir(parents=True, exist_ok=True)
        run(["rgblink", "-t", "-o", args.output, *fixed, out / "program.o"])
        run(["rgbfix", "-v", "-p", "0", "-t", "SANGOKU", args.output])
        print(f"Wrote {args.output}")


if __name__ == "__main__":
    main()
