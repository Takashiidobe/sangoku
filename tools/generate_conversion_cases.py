#!/usr/bin/env python3
"""Generate bounded, integer-only IEEE conversion reference cases."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FLOATS = {16: (10, 5), 32: (23, 8), 64: (52, 11), 128: (112, 15)}
INTS = (8, 16, 32, 64, 128)


def rounded(n, shift):
    if shift <= 0:
        return n << -shift
    q, r = divmod(n, 1 << shift)
    half = 1 << (shift - 1)
    return q + (r > half or r == half and q & 1)


def encode(sign, kind, n, power, width):
    fraction, exponent = FLOATS[width]
    bias = (1 << (exponent - 1)) - 1
    maximum = (1 << exponent) - 1
    sign <<= width - 1
    if kind == 3:
        return sign | maximum << fraction | 1 << (fraction - 1)
    if kind == 2:
        return sign | maximum << fraction
    if not n:
        return sign
    high = n.bit_length() - 1 + power
    unit = max(high, 1 - bias) - fraction
    significand = rounded(n, unit - power)
    if significand >= 1 << (fraction + 1):
        significand >>= 1
        high += 1
    if high > bias:
        return sign | maximum << fraction
    field = max(high + bias, 1) if significand >= 1 << fraction else 0
    return sign | field << fraction | (significand & ((1 << fraction) - 1))


def decode(raw, width):
    fraction, exponent = FLOATS[width]
    field = raw >> fraction & ((1 << exponent) - 1)
    n = raw & ((1 << fraction) - 1)
    sign = raw >> (width - 1)
    if field == (1 << exponent) - 1:
        return sign, 3 if n else 2, 0, 0
    return sign, 1, n | ((1 << fraction) if field else 0), max(field, 1) - ((1 << (exponent - 1)) - 1) - fraction


def float_samples(width):
    fraction, exponent = FLOATS[width]
    maximum = (1 << exponent) - 1
    sign = 1 << (width - 1)
    values = [0, sign, 1, sign | 1, (1 << fraction) - 1,
              1 << fraction, (maximum << fraction) - 1,
              maximum << fraction, sign | maximum << fraction,
              maximum << fraction | 1, sign | maximum << fraction | 1]
    for n, p in [(1, 0), (3, -1), (1, -1), (3, -2)]:
        for s in (0, 1):
            values.append(encode(s, 1, n, p, width))
    return values


def main():
    cases = []
    functions = {}

    def add(op, source, destination, raw, expected, trap=False):
        name = f'{op}_{source}_{destination}'
        functions[name] = (op, source, destination)
        cases.append(dict(name=name, source=f'{raw:x}', result=f'{expected:x}', trap=trap))

    for width in FLOATS:
        for bits in INTS:
            if width <= 32 and bits <= 32:
                continue
            for signed in (False, True):
                op = 'fcvt_from_sint' if signed else 'fcvt_from_uint'
                limit = 1 << (bits - signed)
                values = [0, 1, limit - 1, limit // 2 - 1]
                if signed:
                    values += [-1, -limit, -limit + 1]
                precision = FLOATS[width][0] + 1
                if bits > precision + 2:
                    values += [(1 << (precision + 1)) + delta for delta in (1, 2, 3)]
                for value in values:
                    raw = value & ((1 << bits) - 1)
                    expected = encode(int(value < 0), 1, abs(value), 0, width)
                    add(op, f'i{bits}', f'f{width}', raw, expected)
                op = 'fcvt_to_sint' if signed else 'fcvt_to_uint'
                low, high = (-limit, limit - 1) if signed else (0, limit - 1)
                samples = float_samples(width)
                fraction, _ = FLOATS[width]
                for boundary in (limit, -limit if signed else 0):
                    raw = encode(int(boundary < 0), 1, abs(boundary), 0, width)
                    samples += [raw, raw + 1, max(raw - 1, 0)]
                samples.append(encode(1, 1, (limit << 1) + 1 if signed else 1, -1, width))
                for raw in dict.fromkeys(samples):
                    sign, kind, n, power = decode(raw, width)
                    magnitude = n << power if power >= 0 else n >> -power
                    value = -magnitude if sign else magnitude
                    trap = kind >= 2 or not low <= value <= high
                    if kind == 3:
                        result = 0
                    elif kind == 2:
                        result = low if sign else high
                    else:
                        result = min(max(value, low), high)
                    result &= (1 << bits) - 1
                    add(op, f'f{width}', f'i{bits}', raw, result, trap)
                    add(op + '_sat', f'f{width}', f'i{bits}', raw, result)
        for other in FLOATS:
            if width == other or max(width, other) <= 32:
                continue
            op = 'fpromote' if width < other else 'fdemote'
            samples = float_samples(width)
            fraction = FLOATS[other][0]
            if width > other:
                for delta in (-1, 0, 1, 3):
                    samples.append(encode(0, 1, (1 << (fraction + 2)) + 2 + delta, -fraction - 2, width))
                bias = (1 << (FLOATS[other][1] - 1)) - 1
                samples += [encode(0, 1, n, 1 - bias - fraction - 1, width) for n in (1, 3, 5)]
            for raw in dict.fromkeys(samples):
                expected = encode(*decode(raw, width), other)
                add(op, f'f{width}', f'f{other}', raw, expected)
    fixture = 'set enable_multi_ret_implicit_sret=true\ntarget z80\n'
    for name, (op, source, destination) in functions.items():
        fixture += f'''
function %{name}(i16) system_v {{
block0(v0: i16):
    v1 = load.{source} v0
    v2 = {op}.{destination} v1
    store v2, v0+16
    return
}}
'''
    output = ROOT / 'tests/fixtures/emulate'
    (output / 'wide-convert.clif').write_text(fixture)
    digest = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    rows = ',\n'.join('    ' + json.dumps(case) for case in cases)
    (output / 'wide-convert.json').write_text(
        '{\n  "generator_sha256": "' + digest + '",\n  "cases": [\n' + rows + '\n  ]\n}\n')



if __name__ == '__main__':
    main()
