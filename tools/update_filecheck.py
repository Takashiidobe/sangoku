#!/usr/bin/env python3
"""Regenerate the FileCheck expectations embedded in sangoku's fixtures.

Each directory under tests/fixtures named in STAGES is one fixture type: the
phase of the compiler it exercises. Every fixture there carries its expected
output as comment lines at the end of the file:

    // CHECK: fn gbc_main() -> u8 {
    // CHECK-NEXT:     let mut _0: u8;
    // CHECK-EMPTY:

Lines after the first use -NEXT (or -EMPTY for blank lines), so the block reads
as a full-line match of the phase's output. Expectations come from the compiler
under test and are reviewed in the diff; they are never hand-written.

Usage:
    tools/update_filecheck.py                       # every fixture
    tools/update_filecheck.py 'tests/fixtures/mir/*.rs'
    tools/update_filecheck.py tests/fixtures/mir/add.rs
    tools/update_filecheck.py --check               # run FileCheck, change nothing
"""
import argparse
import glob
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FIXTURES = ROOT / "tests" / "fixtures"
SANGOKU = Path(os.environ.get("SANGOKU", ROOT / "target" / "debug" / "sangoku"))

VOLATILE_CLIF = re.compile(r"^(?:set |; abi |; instance )")
CRATE_HASH = re.compile(r"\[[0-9a-f]{4}\]")


def clif_lines(lines):
    return [CRATE_HASH.sub("[..]", l) for l in lines if not VOLATILE_CLIF.match(l)]


def mir_lines(lines):
    while lines and lines[0].startswith("//"):
        lines.pop(0)
    return lines


# stage: (command, output filter, fixture suffix, comment leader)
STAGES = {
    "mir": (lambda f: [str(SANGOKU), "frontend", str(f), "-"], mir_lines, ".rs", "//"),
    "clif": (lambda f: [str(SANGOKU), "clif", str(f), "-"], clif_lines, ".rs", "//"),
    "lower": (lambda f: [str(SANGOKU), "lower", str(f), "-"], lambda lines: lines, ".clif", ";"),
}

GENERATED = re.compile(r"^(?://|;) CHECK(?:-[A-Z]+)?:.*$")


class ToolError(Exception):
    pass


def escape(line):
    return line.replace("{{", "{{[{]}}{").replace("[[", "{{[[]}}[").rstrip()


def stage_of(fixture):
    parts = fixture.resolve().relative_to(FIXTURES).parts
    if len(parts) < 2 or parts[0] not in STAGES:
        return None
    return parts[0]


def run_stage(fixture):
    command, normalize, _, _ = STAGES[stage_of(fixture)]
    result = subprocess.run(
        command(fixture),
        capture_output=True,
        text=True,
        cwd=ROOT,
    )
    if result.returncode != 0:
        raise ToolError(f"{fixture.name} failed:\n{result.stderr}")
    lines = result.stdout.rstrip("\n").split("\n")
    lines = normalize(lines)
    while lines and lines[0] == "":
        lines.pop(0)
    return lines


def checks_for(lines, leader):
    out = []
    for index, line in enumerate(lines):
        if line.strip() == "":
            out.append(f"{leader} CHECK-EMPTY:")
        else:
            directive = "CHECK" if index == 0 else "CHECK-NEXT"
            out.append(f"{leader} {directive}: {escape(line)}")
    return out


def source_without_checks(fixture):
    kept = [l for l in fixture.read_text().split("\n") if not GENERATED.match(l)]
    return "\n".join(kept).rstrip("\n")


def update(fixture):
    text = source_without_checks(fixture) + "\n\n"
    text += "\n".join(checks_for(run_stage(fixture), STAGES[stage_of(fixture)][3])) + "\n"
    if text != fixture.read_text():
        fixture.write_text(text)
        return True
    return False


def check(fixture):
    result = subprocess.run(
        [os.environ.get("FILECHECK", "FileCheck"), str(fixture), "--match-full-lines"],
        input="\n".join(run_stage(fixture)) + "\n",
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise ToolError(f"{fixture.relative_to(ROOT)}: FileCheck failed\n{result.stderr}")


def discover(patterns):
    if not patterns:
        patterns = [str(FIXTURES / stage / f"*{STAGES[stage][2]}") for stage in STAGES]
    paths = sorted({Path(p).resolve() for pattern in patterns for p in glob.glob(pattern)})
    fixtures = [p for p in paths if stage_of(p)]
    if not fixtures:
        raise ToolError("no fixtures matched")
    return fixtures


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("patterns", nargs="*")
    parser.add_argument("--check", action="store_true", help="run FileCheck without rewriting")
    args = parser.parse_args()
    if not SANGOKU.exists():
        print("build the compiler first: cargo build", file=sys.stderr)
        return 1
    failed = False
    for fixture in discover(args.patterns):
        name = fixture.relative_to(ROOT)
        try:
            if args.check:
                check(fixture)
            elif update(fixture):
                print(f"updated {name}")
        except ToolError as error:
            failed = True
            print(f"{name}: {error}", file=sys.stderr)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
