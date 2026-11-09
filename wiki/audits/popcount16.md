# SM83 16-bit population count audit

The spill defect below was subsequently fixed in `sangoku-7em`; see the
[repair log](../log/2026-11-06-14-51.md). The retained baseline and measurements
describe the compiler at audit time. The replacement is implemented in
`sangoku-pg3`; see the current results below.

## Current selection

Native i8/i16/i32 size/speed helpers and shared byte counting supersede the
selection described below. Optimized non-size i16 remains inline. See
[runtime libraries](../concepts/runtime-libraries.md) for the current ABI,
selection policy, measurements and bounded test command.

## Earlier inline implementation

Rust optimization level `0` uses the compact loop, `1`/`2`/`3` use parallel
counting, and `s`/`z` use the compact loop. The compiler bridge preserves the
size preference in the LR35902 `optimize_for_size` ISA flag; the shared
`opt_level=none` setting identifies unoptimized code. Other targets retain their
existing settings. Both algorithms are inline native instructions; neither adds
a table or a runtime helper call.

The target instruction consumes fixed B/C inputs and produces A/B outputs. It
declares C/D scratch clobbers for the loop, plus E for parallel counting. HL
remains available to the allocator. Inputs used after the instruction are
preserved through ordinary allocation, including spills when necessary.

The actual [Rust probes](../../tests/fixtures/popcount/replacement.rs) include
compiler control flow, moves, zero extension and spills. Costs include RET;
stack includes its two-byte return address and any frame/temporary saves. Each
row has constant cycles across the tested inputs.

| Level | Probe                  | Code bytes | T-cycles | Peak stack |
| ----- | ---------------------- | ---------: | -------: | ---------: |
| `0`   | Count returned as u16  |         27 |      728 |          2 |
| `0`   | Count returned as u32  |         25 |      720 |          2 |
| `0`   | Original input + count |         36 |      764 |          2 |
| `0`   | Two live inputs/counts |        166 |     2488 |         14 |
| `1–3` | Count returned as u16  |         57 |      248 |          2 |
| `1–3` | Count returned as u32  |         63 |      272 |          2 |
| `1–3` | Original input + count |         67 |      288 |          2 |
| `1–3` | Two live inputs/counts |        245 |     1708 |         14 |
| `s/z` | Count returned as u16  |         19 |      696 |          2 |
| `s/z` | Count returned as u32  |         25 |      720 |          2 |
| `s/z` | Original input + count |         29 |      736 |          2 |
| `s/z` | Two live inputs/counts |        153 |     2436 |         14 |

The simple direct CLIF functions still match the audited 13-byte/664-cycle and
51-byte/216-cycle kernels including RET. The Rust fixture adds control-flow
instructions; unoptimized Rust adds further moves/blocks. Consequently, kernel
counts alone do not predict complete caller costs. The pressure probe reads its
fourth input byte from the incoming stack and computes the wrapping sum of both
inputs and both counts. It forces preservation across two counting instructions.

Reproduce with `cargo run --example check-popcount -- --exhaustive16`. All four
probes pass every u16 input at all six levels; the second pressure input uses
`(40503 * input + 12345) & 65535`, plus independent seed-11 pairs. Alternate
stacks, poisoned registers, native-byte/RGBDS parity, general stack-access
regressions and the compiler/allocation verifiers are included. The run passes
1,918,408 executions. Commands, sources, objects, ROMs, maps, symbols, hashes
and results are retained in `build/popcount-replacement`. See the
[implementation log](../log/2026-11-06-15-10.md) for validation.

## Historical audit decision

**Replace the current expansion: YES.** It is incorrect when its temporaries
spill, and a minimally repaired version is still much larger and slower than the
audited alternatives. The general spill/reload defect is tracked in
`sangoku-7em`; popcount replacement is tracked separately in `sangoku-pg3`.

**Use table-free parallel counting as the default: YES.** The `swar` prototype
is 51 code bytes and 216 T-cycles as a function including RET, with no lookup
data, alignment constraint, or private stack frame. It preserves HL, allowing
the synthetic inline caller to keep the original input there. Its arithmetic
kernel without RET is 50 bytes and 200 cycles. The backend follow-up must
measure actual generated Rust code, including moves, CFG and register-allocation
costs; these prototype counts are not guarantees for every compiled caller.

**Make lookup tables mandatory: NO.** The page-aligned nibble lookup is faster
and has slightly fewer code-plus-data bytes: 34 + 16 bytes and 164 cycles. It is
a strong alternative when page placement is acceptable. It requires a
256-byte-aligned table and uses HL for addressing, so the live-input fixture
must save the input on the stack. Alignment padding is not included in the
component byte counts. The byte table is fastest but requires 256 data bytes.

**Unconditionally outline every popcount: NO.** A call adds 40 cycles relative
to the same inline sequence, and the normal call ABI can force register saves.
Outlining can still save ROM across multiple uses; the per-mode component sizes
below make that tradeoff explicit. A 13-byte constant-time shift loop is the
smallest tested general implementation and is the audited size alternative. At
audit time, the cg_clif bridge mapped all optimized Rust levels to
`speed_and_size`, so that flag alone could not distinguish a size request from
ordinary level 2.

This audit covers `popcnt.i16` and its 16-bit bit patterns, including all
zero-extended eight-bit inputs. It does not establish support or decisions for
other IR widths. Production compiler/runtime code is unchanged by the audit. The
[testing log](../log/2026-11-06-14-36.md) records completion and validation.

## The current compiler output fails

The actual Rust probe is
[baseline.rs](../../tests/fixtures/popcount/baseline.rs). It invokes the pinned
compiler's `ctpop` intrinsic on `u16`, then returns the count as `u16`. The
emitted function is 207 bytes. Input zero returns 11 with harness stack base
$D000, or 12 with stack base $D002. It should return zero in both cases. The
result depends on the stack address because a spill destroys the input:

```asm
    ld l, b
    ld h, c
```

L/H initially hold the value. Later, while that value is live:

```asm
    ld a, e
    ld hl, sp + 0
    ld [hl], a
    ld a, l
```

The address setup replaces both input bytes. A reload does the same later.
Explicit stack instructions declare A/H/L clobbers, but allocation-generated
spill/reload edits are emitted after register allocation. Their scratch-register
requirements need a general fix rather than a popcount-only workaround.

The `repaired` audit control pushes/restores HL around the two address setups,
using SP+2 while the temporary save is present. It passes the correctness corpus
at 211 bytes and 936 cycles. That patch is confined to the audit fixture
materialization; the backend is not repaired here. Broken output timings are not
used to claim a correct implementation is faster.

## Algorithm results

All costs in this table are function bodies including RET. Stack includes the
supplied two-byte return address and any internal saves/calls, but excludes an
outer caller's own frame. Every implementation owns zero scratch RAM. Table
storage is ROM. The variable-time bit-clearing row uses the uniform-random class
(1,024 values, seed 0); its complete-domain formula is given below.

| Candidate     | Code bytes | Table bytes | Cycles min/median/max | Peak stack |
| ------------- | ---------: | ----------: | --------------------: | ---------: |
| `repaired`    |        211 |           0 |           936/936/936 |          6 |
| `shift`       |         13 |           0 |           664/664/664 |          2 |
| `byte-loop`   |         20 |           0 |           540/540/540 |          2 |
| `unrolled`    |         66 |           0 |           276/276/276 |          2 |
| `clear`       |         23 |           0 |          208/688/1168 |          2 |
| `swar`        |         51 |           0 |           216/216/216 |          2 |
| `swar-shared` |         36 |           0 |           300/300/300 |          4 |
| `nibble`      |         42 |          16 |           196/196/196 |          2 |
| `nibble-page` |         34 |          16 |           164/164/164 |          2 |
| `byte`        |         10 |         256 |              60/60/60 |          2 |
| `sdcc`        |         42 |           0 |        5232/5232/5232 |          6 |

The table-free parallel version beats the straightforward unrolled count on both
code size and cycles. Its byte operation groups two-bit fields, then four-bit
fields, then sums the nibbles:

```c
x -= (x >> 1) & 0x55;
x = (x & 0x33) + ((x >> 2) & 0x33);
x = (x + (x >> 4)) & 0x0f;
```

Apply that to both bytes and add the two counts. The SM83 sequence uses RRCA
with masks to discard the rotated-in bits, and SWAP to exchange nibbles. See
[swar.asm](../../tests/fixtures/popcount/swar.asm). The shared-byte variant
reuses one byte-count subroutine, reducing code to 36 bytes but increasing cost
to 300 cycles and adding a nested return address.

The compact shift loop is:

```asm
    xor a
    ld d, 16
.loop
    srl c
    rr b
    adc 0
    dec d
    jr nz, .loop
    ret
```

Input low/high bytes arrive in B/C, and output low/high is A/B. After sixteen
logical shifts, B is already zero, so an explicit zero-extension instruction is
unnecessary. This loop is 13 bytes and 664 cycles including RET.

The bit-clearing variant implements `x &= x - 1` once per set bit. Its cost is
`48 + 80 * count_ones(x)` cycles: 48 for zero, 208 for two set bits, and 1328
for all ones. It is useful for a known sparse domain, but its general-input
latency is worse than parallel counting.

## Lookup tables and alignment

- `nibble` performs four lookups into 16 bytes aligned to 16. An immediate low
  address adjustment keeps indexing correct without requiring a page boundary.
- `nibble-page` removes those four adjustments by placing the same table at a
  page boundary, saving eight code bytes and 32 cycles.
- `byte` uses two direct lookups into a page-aligned 256-byte table.

The aligned nibble version totals 50 instruction/data bytes versus the parallel
version's 51. It is faster in these fixtures, so its layout tradeoff must not be
hidden. Page alignment can require 0..255 padding bytes in a particular layout;
16-byte alignment can require 0..15. Other sections may fill those gaps. The
fixed-address audit isolates code/data costs and does not prove that an eventual
ROM layout incurs no padding or banking cost. Both lookup versions occupy ROM0
and clobber HL for addressing.

## Calls, inlining and live inputs

Callers load the input from a two-byte harness-owned WRAM block. `call` invokes
the helper; `inline` expands its main body without RET. The live modes return
`input + count_ones(input)` modulo 65536. Generic calls save the input on the
stack because the current ABI treats the general registers as clobbered. An
inline body that preserves HL can keep the input in that pair instead.

Each row uses the uniform-random class median. ROM is the code plus shared table
needed for one use, including the input-loading/continuation code; it excludes
alignment gaps. For calls this includes the helper once. Inline totals exclude
the separately assembled helper that would be unused in that mode. The retained
images contain all measurement modes for reproducibility.

| Candidate     | Mode          | One-use ROM bytes | Median cycles | Peak stack |
| ------------- | ------------- | ----------------: | ------------: | ---------: |
| `repaired`    | `call`        |               220 |           996 |          6 |
| `repaired`    | `inline`      |               216 |           956 |          4 |
| `repaired`    | `call-live`   |               228 |          1048 |          8 |
| `repaired`    | `inline-live` |               224 |          1008 |          6 |
| `shift`       | `call`        |                22 |           724 |          2 |
| `shift`       | `inline`      |                18 |           684 |          0 |
| `shift`       | `call-live`   |                30 |           776 |          4 |
| `shift`       | `inline-live` |                26 |           716 |          0 |
| `swar`        | `call`        |                60 |           276 |          2 |
| `swar`        | `inline`      |                56 |           236 |          0 |
| `swar`        | `call-live`   |                68 |           328 |          4 |
| `swar`        | `inline-live` |                64 |           268 |          0 |
| `swar-shared` | `call`        |                45 |           360 |          4 |
| `swar-shared` | `call-live`   |                53 |           412 |          6 |
| `nibble-page` | `call`        |                59 |           224 |          2 |
| `nibble-page` | `inline`      |                55 |           184 |          0 |
| `nibble-page` | `call-live`   |                67 |           276 |          4 |
| `nibble-page` | `inline-live` |                63 |           236 |          2 |
| `byte`        | `call`        |               275 |           120 |          2 |
| `byte`        | `inline`      |               271 |            80 |          0 |
| `byte`        | `call-live`   |               283 |           172 |          4 |
| `byte`        | `inline-live` |               279 |           132 |          2 |

For the parallel sequence, two ordinary call sites need 51 shared helper bytes
plus 18 caller bytes, versus 112 bytes for two independent inline fixtures. That
saves 43 ROM bytes but adds call latency and may add stack saves. Actual Rust
register pressure, spill costs, source-value liveness and branch layout must be
measured after implementing the backend lowering. The hand-written fixtures
demonstrate feasible sequences, not the behavior of an unimplemented allocator
strategy.

## SDCC comparison

No dedicated Z80/SM83 population-count assembly or target intrinsic was found in
the inspected `src/z80` and `device/lib` inventory. A generic
`device/lib/__stdc_count_ones.c` does exist. Its full implementation accepts 64
bits and checks a variable mask for each bit.

The audit specializes that source pattern to a 16-bit input, sixteen iterations,
an eight-bit local count and a 16-bit return. SDCC 4.6.4 #17147 was built from
`~/sdcc` out of tree with only the SM83 port enabled. Its actual output
recomputes `1 << i` each iteration using an inner shift loop. The 42-byte
adapted helper includes the bridge from B/C input to SDCC's DE input and from
its BC result to A/B. It costs 5232 cycles. This measures that particular
width-specialized source and compiler output, not a claim about every SDCC
implementation.

[provenance.json](../../tests/fixtures/popcount/provenance.json) records
compiler versions, configure/build commands, input/output/source hashes, backend
revisions and the upstream C source revision/hash. The generated native SDCC
assembly and C specialization are frozen beside the Rust probe. Original SDCC
notices and the shared GPL text reference are in
[NOTICE](../../tests/fixtures/popcount/NOTICE). No upstream sources were
changed.

## Reproduce

```bash
cargo test
cargo run --example audit-popcount -- --samples 1024 --seed 0 --exhaustive16 \
  --output build/popcount-audit/results.json \
  --artifacts build/popcount-audit/artifacts --csv wiki/audits/popcount16.csv
```

All eleven correct candidates passed 342,928 executions: all eight-bit inputs,
boundary patterns, single bits, sparse/dense values and seed-0 random values
across body/call/live/inline modes, plus all 65,536 word values for parallel
counting and each lookup-table version. Separate unit tests use seed 1,
high-byte-only values, alternate stacks and poisoned scratch registers. The SWAP
emulator addition has independent exhaustive byte-result/flag/timing tests. The
CSV stores per-class min/median/max/mean costs; JSON also retains exact inputs,
code/data/caller sizes, stack metrics, clobbers and source/tool records.

To regenerate compiler outputs without replacing the frozen historical files:

```bash
cargo run --example generate-popcount-baselines -- \
  --sdcc build/popcount-audit/sdcc-build/src/sdcc \
  --output build/popcount-audit/regenerated
```

This rebuilds the pinned Rust backend and invokes its toolchain, then compiles
the C specialization with the supplied SDCC binary. Building SDCC itself needs a
C/C++ toolchain, parser generators and Boost headers; the exact configuration
used here is recorded in provenance. Audit tests use frozen compiler output and
do not require rebuilding SDCC. The regenerated Rust function's bytes were
checked against the frozen 207-byte failing output.

Measurements use the flat-memory SM83 instruction subset. Interrupts, peripheral
timing, memory banking and bus effects are excluded. Unsupported instructions
fail explicitly. No production compiler or runtime code was replaced here.
