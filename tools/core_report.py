#!/usr/bin/env python3
"""Report how much of Rust `core` compiles to CLIF and lowers for LR35902.

Builds core through the backend in CLIF-only mode, then compiles
tests/core_driver/driver.rs against it so generic core code is instantiated.
Each CLIF function is lowered separately with `sangoku-lower --probe`.

Usage:
    tools/core_report.py [--json] [--keep DIR] [--skip-backend-build]
"""
import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOOLCHAIN = "gbc-lr35902"
BACKEND = ROOT / "third_party/rustc_codegen_cranelift/target/release/librustc_codegen_cranelift.so"
DRIVER = ROOT / "tests/core_driver/driver.rs"
CORE = ROOT / "third_party/rust/library/core/src/lib.rs"
BUILTINS = ROOT / "third_party/rust/library/compiler-builtins/compiler-builtins/src/lib.rs"


def run(cmd, env=None, check=True):
    result = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, env=env)
    if check and result.returncode != 0:
        sys.exit(f"{' '.join(map(str, cmd))} failed:\n{result.stderr[-4000:]}")
    return result


def rustc(extra, source, output, env):
    cmd = [
        "rustc", f"+{TOOLCHAIN}", "--edition=2024", "--crate-type=rlib",
        "--target", str(ROOT / "targets/lr35902.json"), "-Zunstable-options",
        "-Cpanic=abort", "-Copt-level=0", "-Awarnings",
        f"-Zcodegen-backend={BACKEND}", "--emit=asm,llvm-ir,metadata",
        *extra, str(source), "-o", str(output),
    ]
    return run(cmd, env=env, check=False)


def symbols(clif):
    names = {}
    lines = clif.splitlines()
    for index, line in enumerate(lines):
        if line.startswith("function "):
            name = line[len("function "):].split("(")[0]
            following = lines[index + 1] if index + 1 < len(lines) else ""
            names[name] = following.removeprefix("; symbol ").strip() or name
    return names


def probe(clif_path):
    result = run(["cargo", "run", "-q", "--release", "-p", "sangoku-lower", "--", "--probe", str(clif_path)])
    names = symbols(clif_path.read_text())
    rows = []
    for line in result.stdout.splitlines():
        name, status, *message = line.split("\t")
        rows.append({"symbol": names.get(name, name), "status": status, "message": (message or [""])[0]})
    return rows


def normalise(message):
    return re.sub(r"\d+", "N", message)


def summarise(label, rows, backend_errors):
    counts = Counter(row["status"] for row in rows)
    groups = defaultdict(list)
    for row in rows:
        if row["status"] != "ok":
            groups[(row["status"], normalise(row["message"]))].append(row["symbol"])
    return {
        "label": label,
        "functions": len(rows),
        "ok": counts["ok"],
        "failed": len(rows) - counts["ok"],
        "backend_errors": backend_errors,
        "groups": [
            {"status": status, "message": message, "count": len(syms), "examples": syms[:3]}
            for (status, message), syms in sorted(groups.items(), key=lambda item: -len(item[1]))
        ],
    }


def backend_errors(stderr):
    return sorted({line.strip() for line in stderr.splitlines() if line.startswith("error")})


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--keep", type=Path)
    parser.add_argument("--skip-backend-build", action="store_true")
    args = parser.parse_args()

    if not args.skip_backend_build:
        run(["cargo", f"+{TOOLCHAIN}", "build", "--release", "--locked", "--manifest-path",
             str(ROOT / "third_party/rustc_codegen_cranelift/Cargo.toml")])
    env = {**os.environ, "CG_CLIF_CLIF_ONLY": "1"}
    with tempfile.TemporaryDirectory() as scratch:
        out = args.keep or Path(scratch)
        out.mkdir(parents=True, exist_ok=True)
        core_result = rustc(["--crate-name", "core"], CORE, out / "core.s", env)
        if core_result.returncode != 0:
            sys.exit(f"core did not compile:\n{core_result.stderr[-4000:]}")
        reports = [summarise("core", probe(out / "core.ll"), [])]
        deps = ["--extern", f"core={out / 'libcore.rmeta'}", "-L", f"dependency={out}"]
        builtins_result = rustc(
            ["--crate-name", "compiler_builtins", "--cfg", 'feature="compiler-builtins"', *deps],
            BUILTINS, out / "compiler_builtins.s", env)
        builtins_clif = out / "compiler_builtins.ll"
        reports.append(summarise(
            "compiler_builtins",
            probe(builtins_clif) if builtins_clif.exists() else [],
            backend_errors(builtins_result.stderr)))
        deps += ["--extern", f"compiler_builtins={out / 'libcompiler_builtins.rmeta'}"]
        driver_result = rustc(["--crate-name", "core_driver", *deps], DRIVER, out / "driver.s", env)
        errors = backend_errors(driver_result.stderr)
        driver_clif = out / "driver.ll"
        rows = probe(driver_clif) if driver_clif.exists() else []
        reports.append(summarise("core generics (driver)", rows, errors))


    if args.json:
        print(json.dumps(reports, indent=2))
        return
    for report in reports:
        print(f"{report['label']}: {report['ok']}/{report['functions']} lower, {report['failed']} fail")
        for error in report["backend_errors"]:
            print(f"  backend error: {error}")
        for group in report["groups"]:
            print(f"  {group['count']:5}  {group['status']}: {group['message']}")
            for example in group["examples"]:
                print(f"           e.g. {example}")


if __name__ == "__main__":
    main()
