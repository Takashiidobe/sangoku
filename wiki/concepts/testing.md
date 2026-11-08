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

See [frontend fixtures](frontend-fixtures.md) for snapshot and ROM coverage,
[core and alloc](core-alloc.md) for library surveys, and the
[intrinsic audit](../intrinsics.md) for source-level call routes.
