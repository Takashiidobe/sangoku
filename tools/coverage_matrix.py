#!/usr/bin/env python3
"""Lower one tiny CLIF function per (op, type) and report what LR35902 supports.

Usage: coverage_matrix.py [--check]

Writes tests/coverage/lower.md and lower.tsv. With --check, fails when the
checked-in files differ from the current backend, or when any generated
function is invalid CLIF (a generator bug, not a missing feature).
"""
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "tests" / "coverage"
INTS = ["i8", "i16", "i32", "i64", "i128"]
FLOATS = ["f16", "f32", "f64", "f128"]
ADDR = "i16"
CCS = ["eq", "ne", "ult", "ule", "ugt", "uge", "slt", "sle", "sgt", "sge"]
FCCS = ["eq", "ne", "lt", "le", "gt", "ge", "ord", "uno"]
BITS = {"i8": 8, "i16": 16, "i32": 32, "i64": 64, "i128": 128}

cases = {}


def add(op, ty, params, ret, body):
    sig = ", ".join(f"v{i}: {t}" for i, t in enumerate(params))
    ret_sig = f" -> {ret}" if ret else ""
    lines = body.split("\n")
    if not re.match(r"\s*(return|trap |br_table)", lines[-1]) and not lines[-1].startswith("block"):
        lines.append("    return")
    decls = [l for l in lines if re.match(r"\s*(sig|fn|ss|gv)\d+ =", l)]
    rest = [l for l in lines if l not in decls]
    cases[(op, ty)] = (
        f"function %f({', '.join(params)}){ret_sig} system_v {{\n"
        + "".join(d + "\n" for d in decls)
        + f"block0({sig}):\n" + "\n".join(rest) + "\n}\n"
    )


def result(body, ret, var="v9"):
    return f"    {body}\n    return {var}" if ret else f"    {body}\n    return"


def binary(op, ty, ret=None, imm=None):
    ret = ret or ty
    add(op, ty, [ty, ty], ret, result(f"v9 = {op} v0, v1", ret))


def unary(op, ty, ret=None):
    ret = ret or ty
    add(op, ty, [ty], ret, result(f"v9 = {op} v0", ret))


for ty in INTS:
    for op in [
        "iadd", "isub", "imul", "udiv", "urem", "sdiv", "srem", "band", "bor", "bxor",
        "smin", "smax", "umin", "umax",
        "umulhi", "smulhi",
    ]:
        binary(op, ty)
    for op in ["ineg", "bnot", "iabs", "popcnt", "clz", "cls", "ctz", "bswap", "bitrev"]:
        unary(op, ty)
    for op in ["ishl", "ushr", "sshr", "rotl", "rotr"]:
        add(f"{op}.const", ty, [ty], ty,
            f"    v8 = iconst.i8 3\n    v9 = {op} v0, v8\n    return v9")
        add(f"{op}.var", ty, [ty, "i8"], ty, f"    v9 = {op} v0, v1\n    return v9")
    for cc in CCS:
        add(f"icmp.{cc}", ty, [ty, ty], "i8", f"    v9 = icmp {cc} v0, v1\n    return v9")
    add("select", ty, [ty, ty, "i8"], ty, "    v9 = select v2, v0, v1\n    return v9")
    add("iconst", ty, [], ty,
        f"    v9 = iconst.{ty} 90\n    return v9" if ty != "i128"
        else "    v8 = iconst.i64 90\n    v9 = uextend.i128 v8\n    return v9")
    add("brif", ty, [ty, ty], ty,
        "    v8 = icmp ult v0, v1\n    brif v8, block1, block2\nblock1:\n    return v0\nblock2:\n    return v1")
    for op in ["uadd_overflow", "sadd_overflow", "usub_overflow", "ssub_overflow",
               "umul_overflow", "smul_overflow"]:
        add(op, ty, [ty, ty], ty, f"    v9, v10 = {op} v0, v1\n    return v9")
    add("load", ty, [ADDR], ty, f"    v9 = load.{ty} v0\n    return v9")
    add("store", ty, [ADDR, ty], None, f"    store v1, v0")
    add("load.offset", ty, [ADDR], ty, f"    v9 = load.{ty} v0+300\n    return v9")
    add("call", ty, [ty], ty,
        f"    sig0 = (i8) -> {ty} system_v\n    fn0 = %callee sig0\n"
        f"    v8 = iconst.i8 1\n    v9 = call fn0(v8)\n    return v9")
    add("call.arg", ty, [ty], None, f"    sig0 = ({ty}) system_v\n    fn0 = %callee sig0\n    call fn0(v0)")

for ty in INTS:
    for dst in INTS:
        if BITS[dst] > BITS[ty]:
            for op in ("uextend", "sextend"):
                add(f"{op}", f"{ty}->{dst}", [ty], dst, f"    v9 = {op}.{dst} v0\n    return v9")
        elif BITS[dst] < BITS[ty]:
            add("ireduce", f"{ty}->{dst}", [ty], dst, f"    v9 = ireduce.{dst} v0\n    return v9")
for ty in ("i8", "i16", "i32"):
    for dst in ("i16", "i32"):
        if BITS[dst] > BITS[ty]:
            for op in ("uload", "sload"):
                add(f"{op}{BITS[ty]}", f"{ty}->{dst}", [ADDR], dst,
                    f"    v9 = {op}{BITS[ty]}.{dst} v0\n    return v9")
    if ty != "i8":
        add(f"istore{BITS[ty] // 2 if ty != 'i16' else 8}", ty, [ADDR, ty], None,
            f"    istore{BITS[ty] // 2 if ty != 'i16' else 8} v1, v0")

for ty in FLOATS:
    for op in ["fadd", "fsub", "fmul", "fdiv", "fmin", "fmax", "fcopysign"]:
        binary(op, ty)
    for op in ["fneg", "fabs", "sqrt", "ceil", "floor", "trunc", "nearest"]:
        unary(op, ty)
    add("fma", ty, [ty, ty, ty], ty, "    v9 = fma v0, v1, v2\n    return v9")
    for cc in FCCS:
        add(f"fcmp.{cc}", ty, [ty, ty], "i8", f"    v9 = fcmp {cc} v0, v1\n    return v9")
    add("select", ty, [ty, ty, "i8"], ty, "    v9 = select v2, v0, v1\n    return v9")
    add("f.const", ty, [], ty, f"    v9 = f{ty[1:]}const 0.0\n    return v9")
    add("load", ty, [ADDR], ty, f"    v9 = load.{ty} v0\n    return v9")
    add("store", ty, [ADDR, ty], None, "    store v1, v0")
    add("call", ty, [ty], ty,
        f"    sig0 = ({ty}) -> {ty} system_v\n    fn0 = %callee sig0\n    v9 = call fn0(v0)\n    return v9")
    for other in FLOATS:
        if ty != other and FLOATS.index(ty) < FLOATS.index(other):
            add("fpromote", f"{ty}->{other}", [ty], other, f"    v9 = fpromote.{other} v0\n    return v9")
            add("fdemote", f"{other}->{ty}", [other], ty, f"    v9 = fdemote.{ty} v0\n    return v9")
    for it in ("i8", "i16", "i32", "i64"):
        for op in ("fcvt_to_sint", "fcvt_to_uint", "fcvt_to_sint_sat", "fcvt_to_uint_sat"):
            add(op, f"{ty}->{it}", [ty], it, f"    v9 = {op}.{it} v0\n    return v9")
        for op in ("fcvt_from_sint", "fcvt_from_uint"):
            add(op, f"{it}->{ty}", [it], ty, f"    v9 = {op}.{ty} v0\n    return v9")
        if BITS[it] == int(ty[1:]):
            add("bitcast", f"{it}->{ty}", [it], ty, f"    v9 = bitcast.{ty} little v0\n    return v9")
            add("bitcast", f"{ty}->{it}", [ty], it, f"    v9 = bitcast.{it} little v0\n    return v9")

add("bitcast", "i128->f128", ["i128"], "f128", "    v9 = bitcast.f128 little v0\n    return v9")
add("bitcast", "f128->i128", ["f128"], "i128", "    v9 = bitcast.i128 little v0\n    return v9")

add("trap", "-", [], None, "    trap user1")
add("trapnz", "i8", ["i8"], None, "    trapnz v0, user1")
add("trapz", "i8", ["i8"], None, "    trapz v0, user1")
add("br_table", "i32", ["i32"], None,
    "    br_table v0, block1, [block1, block2]\nblock1:\n    return\nblock2:\n    return")
add("call_indirect", "i8", [ADDR, "i8"], "i8",
    "    sig0 = (i8) -> i8 system_v\n    v9 = call_indirect sig0, v0(v1)\n    return v9")
add("func_addr", "-", [], ADDR, "    sig0 = () system_v\n    fn0 = %callee sig0\n    v9 = func_addr.i16 fn0\n    return v9")
add("symbol_value", "-", [], ADDR,
    "    gv0 = symbol %data\n    v9 = symbol_value.i16 gv0\n    return v9")
add("stack_addr", "-", [], ADDR,
    "    ss0 = explicit_slot 4\n    v9 = stack_addr.i16 ss0\n    return v9")


def header(verify):
    verifier = "true" if verify else "false"
    return f"set enable_verifier={verifier}\nset enable_multi_ret_implicit_sret=true\ntarget z80\n\n"


def probe(binary, verify):
    names = {}
    text = header(verify)
    for index, ((op, ty), source) in enumerate(cases.items()):
        name = f"c{index}"
        names[name] = (op, ty)
        text += source.replace(source.split("(")[0], f"function %{name}", 1) + "\n"
    with tempfile.NamedTemporaryFile("w", suffix=".clif") as handle:
        handle.write(text)
        handle.flush()
        run = subprocess.run([binary, "--probe", handle.name], capture_output=True, text=True)
    if run.returncode != 0:
        sys.exit(run.stderr or "sangoku-lower --probe failed")
    results = {}
    for line in run.stdout.splitlines():
        name, status, message = (line.split("\t") + ["", ""])[:3]
        results[names[name]] = (status, message)
    return results


def lowering_binary():
    subprocess.run(["cargo", "build", "-q", "-p", "sangoku-lower"], cwd=ROOT, check=True,
                   stderr=subprocess.DEVNULL)
    return str(ROOT / "target" / "debug" / "sangoku-lower")


def render(plain, verified):
    mark = {"ok": "ok", "unsupported": "-", "panic": "PANIC", "invalid": "INVALID",
            "verifier": "VERIFIER", "inexpressible": "n/e"}
    lines = [
        "# LR35902 CLIF lowering coverage",
        "",
        "Generated by `tools/coverage_matrix.py`; do not edit. `ok` lowers, `-` is unsupported,",
        "`PANIC` is a backend crash, `n/e` cannot be written as textual CLIF for that type",
        "(the opcode's type set rejects it). Memory operations use `i16` addresses.",
        "",
    ]
    totals = {}
    for status, _ in plain.values():
        totals[status] = totals.get(status, 0) + 1
    lines += ["Totals: " + ", ".join(f"{k} {v}" for k, v in sorted(totals.items())), ""]
    for title, types in (("Integer operations", INTS), ("Float operations", FLOATS)):
        ops = sorted({op for (op, t) in plain if t in types})
        lines += [f"## {title}", "", "| op | " + " | ".join(types) + " |",
                  "|---|" + "---|" * len(types)]
        for op in ops:
            row = [mark[plain[(op, t)][0]] if (op, t) in plain else "" for t in types]
            lines.append(f"| {op} | " + " | ".join(row) + " |")
        lines.append("")
    lines += ["## Conversions", "",
              "Rows are source types; columns are destination types. `ok` lowers,",
              "`-` is unsupported, `n/a` is outside the opcode's conversion domain,",
              "and `?` is applicable but has no coverage fixture. `n/e` means the",
              "fixture cannot be expressed in textual CLIF.", ""]
    conversion_ops = sorted({op for op, ty in plain if "->" in ty})
    for op in conversion_ops:
        if op == "bitcast":
            domains = [(INTS, FLOATS), (FLOATS, INTS)]
        elif op.startswith("fcvt_from_"):
            domains = [(INTS, FLOATS)]
        elif op.startswith("fcvt_to_"):
            domains = [(FLOATS, INTS)]
        elif op in ("fpromote", "fdemote"):
            domains = [(FLOATS, FLOATS)]
        elif op.startswith(("sload", "uload")):
            domains = [([f"i{op[5:]}"], INTS)]
        else:
            domains = [(INTS, INTS)]
        lines += [f"### `{op}`", ""]
        for sources, destinations in domains:
            lines += ["| source \\ destination | " + " | ".join(destinations) + " |",
                      "|---|" + "---|" * len(destinations)]
            for source in sources:
                row = []
                for destination in destinations:
                    src_bits, dst_bits = int(source[1:]), int(destination[1:])
                    if op == "bitcast":
                        applicable = src_bits == dst_bits
                    elif op in ("ireduce", "fdemote"):
                        applicable = src_bits > dst_bits
                    elif op in ("uextend", "sextend", "fpromote") or op.startswith(("sload", "uload")):
                        applicable = src_bits < dst_bits
                    else:
                        applicable = True
                    entry = plain.get((op, f"{source}->{destination}"))
                    row.append(mark[entry[0]] if entry else "?" if applicable else "n/a")
                lines.append(f"| {source} | " + " | ".join(row) + " |")
            lines.append("")
    lines += ["## Other operations", "", "| op | status |", "|---|---|"]
    for (op, ty), (status, _) in sorted(plain.items()):
        if ty not in INTS and ty not in FLOATS and "->" not in ty:
            lines.append(f"| {op} | {mark[status]} |")
    lines += ["", "## Backend crashes", ""]
    crashes = [f"- `{op}` {ty}: {m}" for (op, ty), (st, m) in sorted(plain.items()) if st == "panic"]
    lines += crashes or ["none"]
    lines += ["", "## Rejected by the CLIF verifier", "",
              "These lower only because the backend runs with the verifier off (release)."
              " Anything listed is invalid CLIF for this target and needs fixing before the verifier can be enabled.", ""]
    rejected = sorted(f"{op} {ty}" for (op, ty), (st, _) in verified.items()
                      if plain[(op, ty)][0] == "ok" and st == "verifier")
    lines.append(", ".join(rejected) if rejected else "none")
    lines.append("")
    return "\n".join(lines)


def main():
    check = "--check" in sys.argv[1:]
    binary = lowering_binary()
    plain = probe(binary, False)
    verified = probe(binary, True)
    bad = [f"{op}.{ty}: {m}" for (op, ty), (s, m) in plain.items() if s in ("invalid",)]
    if bad:
        sys.exit("generated invalid CLIF:\n" + "\n".join(bad))
    markdown = render(plain, verified)
    tsv = "".join(f"{op}\t{ty}\t{s}\t{m}\n" for (op, ty), (s, m) in sorted(plain.items()))
    OUT.mkdir(parents=True, exist_ok=True)
    targets = {OUT / "lower.md": markdown, OUT / "lower.tsv": tsv}
    if check:
        stale = [p.name for p, text in targets.items() if not p.exists() or p.read_text() != text]
        if stale:
            sys.exit(f"stale coverage files: {stale}; run tools/coverage_matrix.py")
        return
    for path, text in targets.items():
        path.write_text(text)
    print(markdown.split("## ")[0])


if __name__ == "__main__":
    main()
