# SM83 shifts and rotates audit

`sangoku-mt4.4` recommends constant specialization and optimization-dependent
variable helpers. This audit changes no production lowering or runtime code.
Implementation follows in `sangoku-9tk` (constants) and `sangoku-clp` (variables).

Run the bounded comparison with:

```bash
cargo run --example audit-shifts -- --artifacts build/shift-audit-fast \
  --output build/shift-audit-fast/results.json
```

Defaults cover counts 0, 1, 8 and width-1, four explicit values and two seeded
random words, body/caller modes and the shortlisted candidates. More than eight
random samples requires `--full`; exhaustive checks require both `--full` and
`--exhaustive16`. Ordinary `cargo test` uses small representative matrices.

## Decisions

| Operation | 16-bit constant | 32-bit constant | Variable, both widths |
| --- | --- | --- | --- |
| Left shift | Replace: YES | Adopt: YES; currently absent | Adopt: YES |
| Logical right shift | Replace: YES | Replace: YES | Adopt: YES |
| Arithmetic right shift | Adopt: YES; currently absent | Adopt: YES; currently absent | Adopt: YES |
| Rotate left | Adopt: YES; currently absent | Adopt: YES; currently absent | Adopt: YES |
| Rotate right | Adopt: YES; currently absent | Adopt: YES; currently absent | Adopt: YES |

For constants, select byte moves plus residual bit operations (`lanes`). For
16-bit left shifts by 2–7, prefer the `pair` candidate using `ADD HL,HL`.
Zero-count identity: Replace: NO. Keep the identity operation.

For variable counts, select `loop` for unoptimized/size intent and `byte-loop`
for speed intent. The byte loop adds 28 clocks for counts below eight; a known
small count range can favor the compact loop even in speed builds. Mask CLIF
counts by width-1. Rust checked shifts need their full-width range guard before
count truncation; masked counts alone do not implement checked shifts.

Blanket outlining: Replace: NO. Inline tiny constant sequences. For helper
bytes H, per-use call bytes C and inline bytes I, outline for strict ROM savings
only when I>C and uses exceed H/(I-C). The CSV records the first saving use
count, including argument packing and live-value preservation. CALL/RET alone
adds 40 T-cycles.

## Measured costs

These body figures include RET in CPU T-cycles. Bytes are linked instruction
bytes. Candidates use the audit ABI; their costs are targets for implementation,
not guarantees about future allocated compiler output.

| Constant case | Current bytes / clocks | Selected bytes / clocks |
| --- | --- | --- |
| 16-bit left by 1 | 7 / 40 | 6 / 36 |
| 16-bit left by 7 | 31 / 136 | 12 / 88 (`pair`) |
| 16-bit left by 8 | 35 / 152 | 2 / 20 |
| 16-bit right by 8 | 35 / 152 | 4 / 28 |
| 16-bit right by 15 | 63 / 264 | 7 / 40 |
| 32-bit right by 1 | 25 / 168 | 16 / 84 |
| 32-bit right by 8 | 81 / 392 | 10 / 60 |
| 32-bit right by 31 | 265 / 1128 | 14 / 76 |

Variable means below give equal weight to every legal normalized count, from
the retained full run. They are not averages over the sparse default cases.

| Width / operation | Bit loop bytes / mean clocks | Byte loop bytes / mean clocks | Byte loop min–max clocks |
| --- | --- | --- | --- |
| 16 / logical shifts | 16 / 284.5 | 30 / 213 | 80–352 |
| 16 / arithmetic right | 16 / 284.5 | 32 / 217 | 80–360 |
| 16 / rotations | 18 / 344.5 | 32 / 241 | 80–408 |
| 32 / logical shifts | 28 / 836.25 | 44 / 385 | 128–648 |
| 32 / arithmetic right | 28 / 836.25 | 46 / 397 | 128–672 |
| 32 / rotations | 30 / 960.25 | 46 / 413 | 128–704 |

Manual helper body stack is 2 bytes for 16-bit and 4 for 32-bit, including the
return address and incoming arguments. Table ROM and owned scratch RAM are
zero. Complete caller, live-input caller, inline and inline-live measurements,
stack peaks and sharing thresholds are in [shifts.csv](shifts.csv). JSON adds
observed register changes; callers must treat AF/BC/DE/HL as clobbered unless
the final compiler ABI establishes narrower preservation.

The 16-bit input is B/C, output A/B, variable count A. The 32-bit input is
B/C/A/[entry SP+2], output A/B/C/D, variable count [entry SP+3]. SDCC uses a
bridge to its native ABI: 52 clocks for 16-bit, 84 for constant 32-bit and 132
for variable 32-bit. CSV native costs subtract that fixed bridge; compare
complete bridged callers for this backend rather than claiming native SDCC
is uniformly slower.

## Evidence and reproduction

Fixtures and compiler probes live in `tests/fixtures/shifts`; the executable
comparison is `tests/support/src/shifts.rs`. SDCC speed/size snapshots come from
the clean `/home/takashi/sdcc` revision 17147. Exact commands, source hash and
tool versions are in `tests/fixtures/shifts/provenance.json`. Optional `--sdcc
PATH` regenerates both profiles and checks normalized instruction parity.

The historical full report is `build/shift-audit/results.json`; retained
assembly/ROMs and compiler output are alongside it. Its CSV is committed here.
That run passed 2,204,465 candidate cases, 5,242,880 exhaustive constant-16
cases and 47,424 compiler executions. Reproduction, explicitly opt-in:

```bash
cargo run --example audit-shifts -- --full --samples 128 --seed 0 \
  --exhaustive16 --output build/shift-audit/results.json \
  --csv wiki/audits/shifts.csv
```

The final bounded run passed 1,968 candidate and 252 compiler executions,
plus 72 actual Rust shift/guard probes, with zero errors. Coverage includes
sign bits, byte boundaries, width-1, oversized masked counts, live inputs,
restored SP, return frames and stack-only writes. Rust guarded payload/validity
probes include count 65536; full core Option-return ABI remains untested.

Supported native CLIF outputs are assembled and byte-compared to emitted
machine code. Missing compiler operations use the explicit `bits` reference.
SRA flags/timing and auto-indexed HL loads/stores have independent emulator
regressions. The emulator models flat-memory instruction timing, without
interrupts, peripherals or banking. See the [testing log](../log/2026-11-06-16-40.md).
