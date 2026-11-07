#!/usr/bin/env python3
"""Generate bounded binary128 reference cases using host libquadmath."""
import argparse
import ctypes
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUTPUT = ROOT / "tests/fixtures/emulate/softfloat-f128.json"
REFERENCE = r"""
#include <quadmath.h>
#include <fenv.h>
typedef union { __float128 value; unsigned char bytes[16]; } Quad;
void reference(unsigned char *ctx) {
    Quad a, b, c, r;
    int i, op = ctx[64], relation;
    fesetround(FE_TONEAREST);
    for (i = 0; i < 16; ++i) { a.bytes[i] = ctx[i]; b.bytes[i] = ctx[16+i]; c.bytes[i] = ctx[32+i]; }
    switch (op) {
    case 0: r.value = a.value + b.value; break;
    case 1: r.value = a.value - b.value; break;
    case 2: r.value = a.value * b.value; break;
    case 3: r.value = a.value / b.value; break;
    case 4: r.value = sqrtq(a.value); break;
    case 5: r.value = fmaq(a.value, b.value, c.value); break;
    case 6: r.value = isnanq(a.value) || isnanq(b.value) ? nanq("") : a.value == b.value ? (signbitq(a.value) ? a.value : b.value) : a.value < b.value ? a.value : b.value; break;
    case 7: r.value = isnanq(a.value) || isnanq(b.value) ? nanq("") : a.value == b.value ? (signbitq(a.value) ? b.value : a.value) : a.value > b.value ? a.value : b.value; break;
    case 8: r.value = truncq(a.value); break;
    case 9: r.value = floorq(a.value); break;
    case 10: r.value = ceilq(a.value); break;
    case 11: r.value = rintq(a.value); break;
    default:
        relation = isnanq(a.value) || isnanq(b.value) ? 8 : a.value == b.value ? 1 : a.value < b.value ? 2 : 4;
        ctx[48] = (relation & ctx[65]) != 0;
        return;
    }
    for (i = 0; i < 16; ++i) ctx[48+i] = r.bytes[i];
}
"""


def integer(value):
    if not value:
        return 0
    sign = int(value < 0) << 127
    value = abs(value)
    exponent = value.bit_length() - 1
    return sign | ((exponent + 16383) << 112) | ((value << (112 - exponent)) & ((1 << 112) - 1))


def generate():
    if sys.byteorder != "little":
        raise SystemExit("reference generation requires a little-endian host")
    one, two = integer(1), integer(2)
    sign, infinity = 1 << 127, 0x7fff << 112
    nan = infinity | (1 << 111)
    maximum = infinity - 1
    minimum, tiny = 1 << 112, 1
    half = (16383 - 1) << 112
    half_ulp = (16383 - 113) << 112
    pairs = [(0, sign), (one, two), (one, half_ulp), (one + 1, half_ulp),
             (minimum, sign | tiny), (tiny, tiny), (maximum, two),
             (maximum, tiny), (tiny, maximum), (integer(-17), integer(5)),
             (infinity, sign | infinity), (nan, one), (infinity | 1, 0)]
    unary = [0, sign, half, sign | half, half + 1, integer(3) - (1 << 112),
             integer(5) - (1 << 112), sign | (integer(5) - (1 << 112)),
             tiny, minimum, two, maximum, infinity, integer(-1), nan]
    triples = [(one, two, integer(3)), (one + 1, one - 2, sign | one),
               (maximum, two, sign | maximum), (tiny, half, tiny),
               (infinity, 0, one), (sign, two, sign),
               (maximum, maximum, sign | infinity), (tiny, tiny, minimum),
               (maximum, maximum, tiny), (tiny, tiny, maximum),
               (nan, one, 0), (one, two, nan), (infinity, two, sign | infinity)]
    modes = {"fadd": 0, "fsub": 1, "fmul": 2, "fdiv": 3, "sqrt": 4,
             "fma": 5, "fmin": 6, "fmax": 7, "trunc": 8, "floor": 9,
             "ceil": 10, "nearest": 11}
    masks = {"eq": 1, "ne": 14, "one": 6, "ueq": 9, "lt": 2,
             "le": 3, "gt": 4, "ge": 5, "ult": 10, "ule": 11,
             "ugt": 12, "uge": 13, "ord": 7, "uno": 8}
    cases = []
    with tempfile.TemporaryDirectory() as directory:
        source = Path(directory) / "reference.c"
        library = Path(directory) / "reference.so"
        source.write_text(REFERENCE)
        subprocess.run(["cc", "-std=gnu11", "-O2", "-shared", "-fPIC", str(source),
                        "-lquadmath", "-lm", "-o", str(library)], check=True)
        ref = ctypes.CDLL(str(library)).reference
        ref.argtypes = [ctypes.POINTER(ctypes.c_ubyte)]

        def add(name, a, b=0, c=0):
            compare = name.startswith("cmp_")
            mode = 12 if compare else modes[name]
            mask = masks[name[4:]] if compare else 0
            data = bytearray(a.to_bytes(16, "little") + b.to_bytes(16, "little")
                             + c.to_bytes(16, "little") + bytes(16) + bytes([mode, mask]))
            ref((ctypes.c_ubyte * 66).from_buffer(data))
            result = int.from_bytes(data[48:64], "little")
            if not compare and (result & infinity) == infinity and result & ((1 << 112) - 1):
                result = nan
            cases.append({"op": name, "a": f"{a:032x}", "b": f"{b:032x}",
                          "c": f"{c:032x}", "result": f"{result:032x}"})

        for a, b in pairs:
            for op in ["fadd", "fsub", "fmul", "fdiv", "fmin", "fmax"]:
                add(op, a, b)
            for cc in masks:
                add("cmp_" + cc, a, b)
        for a in unary:
            for op in ["sqrt", "trunc", "floor", "ceil", "nearest"]:
                add(op, a)
        for a, b, c in triples:
            add("fma", a, b, c)
    digest = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    return json.dumps({"generator_sha256": digest, "cases": cases}, indent=2) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    text = generate()
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit("stale binary128 cases; run tools/generate_softfloat128_cases.py")
    else:
        OUTPUT.write_text(text)


if __name__ == "__main__":
    main()
