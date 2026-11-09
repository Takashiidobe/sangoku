# Testing

```bash
cargo nextest run --no-fail-fast
```

The default profile keeps FileCheck and fast regression tests. Tests measured
above 10 seconds, except FileCheck, belong in the extensive profile configured
in [`.config/nextest.toml`](../../.config/nextest.toml). Run the full suite,
including the core/alloc reports, coverage matrix, panic-profile matrix, and
broader numeric checks, with:

```bash
cargo nextest run --profile extensive --no-fail-fast
```

## Selected optimization comparisons

Capture a baseline before changing the compiler, then compare a separate output
directory after rebuilding the affected backend:

```bash
cargo run --example audit-optimizations -- --output build/optimizations/before
cargo run --example audit-optimizations -- --output build/optimizations/after --baseline build/optimizations/before
```

Use repeated `--case` arguments to select `plane`, `upper`, `lower`, `iterator`,
`range_search`, or `expansion_search`. The cases use volatile inputs and check
bounded samples against host results, including NUL, casing expansions and
search boundaries. Snapshots run cases serially; finish backend builds before
starting them.

Each directory keeps `report.json`, build logs, ROMs, MIR, CLIF, assembly and
maps. Reports include retained section/code/data bytes, demangled symbol sizes,
per-input cycles and peak stack bytes, source hashes and compiler provenance.
Cycles and stack are measured from entry to return, excluding startup and the
caller's return address. Anonymous data is grouped by crate. Comparisons write
`comparison.json`; negative deltas mean smaller or faster. Changed sources or
input sets require a fresh baseline. Size changes are reported, not pass/fail
budgets; incorrect execution fails the run.

Run the same cases through the opt-in test binary with:

```bash
SANGOKU_OPT_OUTPUT=build/optimizations/after SANGOKU_OPT_BASELINE=build/optimizations/before cargo nextest run --profile extensive --no-fail-fast --test optimization
```

`SANGOKU_OPT_CASES=upper,iterator` selects cases in the test path. Omit the
baseline to capture a snapshot. Keep output separate from the baseline.

See [frontend fixtures](frontend-fixtures.md) for snapshot and ROM coverage,
[core and alloc](core-alloc.md) for library surveys, and the
[intrinsic audit](../intrinsics.md) for source-level call routes.
