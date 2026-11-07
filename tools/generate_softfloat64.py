#!/usr/bin/env python3
"""Regenerate the RGBDS f64 or f128 runtime with SDCC's SM83 backend."""
import argparse
import hashlib
import re
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def generate(width=64):
    source = ROOT / f"runtime/softfloat{width}.c"
    with tempfile.TemporaryDirectory() as directory:
        assembly = Path(directory) / f"softfloat{width}.asm"
        try:
            subprocess.run(
                ["sdcc", "-msm83", "--opt-code-size", "--std-c11", "-S",
                 str(source), "-o", str(assembly)], check=True, cwd=ROOT,
            )
        except FileNotFoundError:
            raise SystemExit(f"sdcc is required to regenerate softfloat{width}.asm")
        raw = assembly.read_text()
    digest = hashlib.sha256(source.read_bytes() + Path(__file__).read_bytes()).hexdigest()
    lines = [f"; generated from softfloat{width}.c, sha256 {digest}",
             f'SECTION "softfloat{width}", ROM0']
    code = False
    for line in raw.splitlines():
        line = line.split(";", 1)[0].strip()
        if not line:
            continue
        if line.startswith(".area "):
            code = line.split()[1] in ("_CODE", "_HOME")
            continue
        if line.startswith((".module", ".optsdcc", ".globl")):
            continue
        if not code:
            raise SystemExit(f"unexpected non-code SDCC output: {line}")
        if line.startswith((".dw", ".db")):
            line = line[1:]
        elif line.startswith("."):
            continue
        line = re.sub(r"\b(\d+)\$", r".L\1", line).replace("#", "")
        line = re.sub(r"\((hl[+-]?|bc|de)\)", lambda m: "[" + {"hl+": "hli", "hl-": "hld"}.get(m[1], m[1]) + "]", line)
        line = " ".join(line.split())
        if line.startswith("ldhl "):
            line = "ld hl, sp + " + line.split(",", 1)[1].strip()
        if line == "jp [hl]":
            line = "jp hl"
        lines.append(line)
    lines += ["", f"__gbc_f{width}::", "ld e, b", "ld d, c", f"jp _sangoku_sf{width}", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--width", type=int, choices=[64, 128], default=64)
    args = parser.parse_args()
    output = ROOT / f"runtime/softfloat{args.width}.asm"
    text = generate(args.width)
    if args.check:
        if not output.exists() or output.read_text() != text:
            raise SystemExit(f"stale {output.name}; run tools/generate_softfloat64.py --width {args.width}")
    else:
        output.write_text(text)


if __name__ == "__main__":
    main()
