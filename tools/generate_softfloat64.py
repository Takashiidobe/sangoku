#!/usr/bin/env python3
"""Regenerate the RGBDS f64 runtime with SDCC's SM83 backend."""
import argparse
import hashlib
import re
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "runtime/softfloat64.c"
OUTPUT = ROOT / "runtime/softfloat64.asm"


def generate():
    with tempfile.TemporaryDirectory() as directory:
        assembly = Path(directory) / "softfloat64.asm"
        try:
            subprocess.run(
                ["sdcc", "-msm83", "--opt-code-size", "--std-c11", "-S",
                 str(SOURCE), "-o", str(assembly)], check=True, cwd=ROOT,
            )
        except FileNotFoundError:
            raise SystemExit("sdcc is required to regenerate softfloat64.asm")
        raw = assembly.read_text()
    digest = hashlib.sha256(SOURCE.read_bytes() + Path(__file__).read_bytes()).hexdigest()
    lines = [f"; generated from softfloat64.c, sha256 {digest}",
             'SECTION "softfloat64", ROM0']
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
    lines += ["", "__gbc_f64::", "ld e, b", "ld d, c", "jp _sangoku_sf64", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    text = generate()
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit("stale softfloat64.asm; run tools/generate_softfloat64.py")
    else:
        OUTPUT.write_text(text)


if __name__ == "__main__":
    main()
