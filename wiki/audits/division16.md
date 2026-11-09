# Unsigned 16-bit division audit

**Replace quotient and remainder: YES.** Use the compact `sdcc-z80` SM83
adaptation with independent, correct ABI wrappers and a shared arithmetic core.
The q/r-only object links to 95 bytes; the previous 97-byte object was
incorrect. The runtime replacement is implemented in `sangoku-88j`; its
verification is recorded below.

**Use a shared divmod call when both results are needed: YES.** The audit entry
returns quotient in A/B and remainder in C/D, with low bytes first. Backend
recognition and runtime exposure are tracked in `sangoku-yz4`, after the runtime
replacement. Standalone division/remainder continue to use their own entries.

**Use early exits as the default: NO.** Retain `sdcc-z80-early` as an audited
alternative. It adds 38 bytes and materially improves divide-by-one and
smaller-dividend cases, but increases the narrow-random median by 68 clocks.
There is no application workload distribution establishing that tradeoff as the
default. **Use the SDCC SM83 loop or unrolled restoring variants: NO** for the
default implementation; their measured cycle/size tradeoffs lose to the compact
Z80-derived port on the narrow/wide random classes.

These decisions cover 16-bit unsigned operations only. The backend currently
libcalls `udiv.i16` and `urem.i16`. SDCC's generic `_divulong.c` and
`_modulong.c` were inspected as references; 32-bit code was not benchmarked
without a concrete core reproducer. No verdict is claimed for that width. The
candidate measurements below precede the runtime migration recorded in the
verification section.

## Results

All timings below are CPU T-cycles for complete synthetic callers, including
operand loads, argument packing, call/return, and cleanup. Each random class
contains 1,024 cases with seed 0. Narrow divisors are uniform in 1..127; wide
divisors are uniform in 128..65535. Dividends are independently uniform in
0..65535. The same inputs are used across candidates. Averages are per-class,
not an assumed game workload.

The q/r ROM column comes from an independent link excluding the combined
wrapper. The next column includes q/r/divmod wrappers and the shared core. Stack
is the maximum observed nonzero-divisor caller stack, including arguments and
nested return addresses. All candidates use zero table bytes and zero owned
scratch RAM. The input harness owns a separate four-byte RAM block.

| Candidate        | q/r ROM | With divmod ROM | Narrow median | Wide mean | Peak stack |
| ---------------- | ------: | --------------: | ------------: | --------: | ---------: |
| `rolled`         |      68 |              82 |          2452 |    1973.9 |          8 |
| `unroll4`        |     149 |             163 |          1924 |    1445.9 |          8 |
| `unroll16`       |     466 |             480 |          1744 |    1265.9 |          6 |
| `early`          |     106 |             120 |          2520 |    1181.4 |          8 |
| `sdcc-sm83`      |      83 |              97 |          2672 |    2654.5 |         10 |
| `sdcc-z80`       |      95 |             109 |          1236 |    1133.3 |          6 |
| `sdcc-z80-early` |     133 |             147 |          1304 |     750.0 |          6 |

The repaired rolled control is correct and shares the same wrappers. It is the
performance baseline; timings of the broken production object are not used to
claim an improvement. Against that control, the compact port reduces narrow
caller mean cycles from 2451.7 to 1236 and wide mean cycles from 1973.9 to
1133.3. It trades 27 more ROM bytes for two fewer peak stack bytes.

The compact core performs 16 iterations for divisors below 128 and nine for
wider divisors. Its measured narrow path is 1236 caller clocks throughout the
random class; its wide-random class ranges from 1116 to 1136. Zero-divisor runs
stop at RST vector $0010: compact q/r callers take 228 clocks to that vector and
peak at eight stack bytes. The trap handler itself is not measured.

### Early exits

These rows show quotient caller mean cycles by input class. `less-than` chooses
a uniform nonzero divisor, then a uniform dividend below that divisor; it does
not uniformly sample every possible ordered pair.

| Input class     | Compact port | With early exits |
| --------------- | -----------: | ---------------: |
| `divisor-one`   |       1236.0 |            316.0 |
| `equal`         |       1164.5 |            367.2 |
| `less-than`     |       1136.4 |            328.6 |
| `power-of-two`  |       1177.5 |            772.3 |
| `narrow-random` |       1236.0 |           1295.5 |
| `wide-random`   |       1133.3 |            750.0 |

The early checks handle divisor one, dividend smaller than divisor, and equal
operands before entering the loop. They preserve the zero-divisor policy. Their
benefit is substantial for those input classes; the added ROM and other path
costs are explicit rather than folded into a single average.

### One call versus two

The compact port, wide-random class:

| Requested results | Caller bytes | Median cycles | Mean cycles | Peak stack |
| ----------------- | -----------: | ------------: | ----------: | ---------: |
| `q`               |           22 |          1136 |      1133.3 |          6 |
| `r`               |           22 |          1140 |      1137.3 |          6 |
| `both`            |           22 |          1144 |      1141.3 |          6 |
| `separate`        |           51 |          2324 |      2318.6 |          8 |

`both` is one divmod call. `separate` reloads both inputs for each call and
preserves the quotient on the stack while obtaining the remainder. The combined
wrapper adds 14 helper ROM bytes while reducing the paired caller by 29 bytes,
for a net 15-byte reduction at one such call site in this fixture. The backend
follow-up must measure compiler-emitted callers and preserve trap ordering;
these assembly callers do not establish compiler spill costs.

## Correctness and source adaptations

The frozen production baseline interprets the dividend bytes backwards:
`udiv16(1, 1)` returns 256. The remainder entry does not unpack arguments or
initialize its remainder and produces different results with different scratch
register state. `baseline-alt.asm` fails to link because its quotient entry
references `__gbc_udiv16.divide`, but that scoped label is defined under the
remainder entry. These failures are retained as audit fixtures, not accepted
performance candidates.

All correct candidates independently accept B/C = dividend low/high and A/[SP+2]
= divisor low/high. Quotient/remainder single results return in A/B. The core
contract is BC = quotient and HL = remainder; the combined wrapper maps these to
A/B and C/D. All arithmetic registers/flags may be clobbered. Each
nonzero-divisor execution checks results, stack restoration, and that writes
remain in the measured stack frame. Combined results additionally satisfy
`dividend == divisor * quotient + remainder` and `remainder < divisor`.

- `rolled`, `unroll4`, and `unroll16` repair initialization and byte order,
  share wrappers/core, and execute one, four, or sixteen restoring steps per
  group. Full unrolling removes the counter save/restore entirely.
- `early` adds the same early checks to the rolled core.
- `sdcc-sm83` adapts `.divu16` from SDCC's SM83 `divint.s`: RGBDS syntax,
  register-result adaptation, trap policy, and redundant tail-jump removal.
- `sdcc-z80` adapts the split algorithm from `z80-common/divunsigned.s`.
  `adc hl,hl` becomes `rl l; rl h`; `sbc hl,de` becomes bytewise subtraction
  with A held temporarily in the otherwise free C register. `djnz` becomes
  `dec b; jr nz`. Register moves implement the output mapping instead of EX.
- `sdcc-z80-early` adds the early checks to that port.

The shared Z80 `modunsigned.s` is a wrapper around division and exchanges the
result registers, supporting a shared core rather than an independent modulo
algorithm. SDCC source paths, SVN revisions and SHA-256 hashes are recorded in
[provenance.json](../../tests/fixtures/division/provenance.json). Original
notices and the license text accompany the audit-only adapted sources in
[NOTICE](../../tests/fixtures/division/NOTICE) and
[COPYING](../../tests/fixtures/division/COPYING).

## Reproduce the tests and measurements

```bash
cargo test
cargo run --example audit-division -- --samples 1024 --seed 0 --exhaustive8 \
  --output build/division-audit/results.json \
  --artifacts build/division-audit/artifacts --csv wiki/audits/division16.csv
```

The audit completed 309,985 candidate executions without a correctness failure:
25,543 nonzero-divisor executions for each of seven variants, 16 zero-divisor
checks per variant, and all 65,536 eight-bit operand pairs (including zero) for
each of the two Z80-derived variants. Unit tests add a separate seed-1 random
corpus, scratch-register poison values, baseline failure checks, and independent
emulator arithmetic/flag/timing regressions.

Input classes include all pairs of 17 boundary values with nonzero divisors,
divisor one, equal operands, powers of two, smaller dividends, narrow random
divisors, and wide random divisors. The JSON stores exact inputs and per-class
min/median/max/mean costs; [division16.csv](division16.csv) retains those
measurements for every operation and mode. Body costs include RET and a
preexisting four-byte argument/return frame; caller costs include the full
sequence. Source snapshots, hashes, objects, symbols, maps, ROMs, commands, and
tools versions are retained under the artifact directory. `raw-alt` also retains
`build-error.txt` with its linker diagnostic.

The emulator remains a flat-memory SM83 instruction subset. Results exclude
interrupts, peripherals, memory banking and bus-cycle behavior. Unsupported
instructions fail. No new emulator instructions were required for this audit.
The reporting helper now accepts an initialization callback and a named stop
symbol so multiple caller fixtures can share one linked candidate image.

## Runtime verification (sangoku-88j)

`runtime/integer/udiv/common/i16.asm` now uses the compact port with independent quotient and
remainder ABI wrappers. The linked object is 95 bytes, including both wrappers
and the 70-byte shared core. At the same fixed link address, its instruction
bytes match the audited q/r-only object exactly. The unused combined wrapper is
omitted; paired-operation lowering remains in `sangoku-yz4`.

The broken alternate was removed from `runtime/`; its frozen copy remains in the
audit fixtures. Source provenance, the original SDCC notice and additional
exception, and the GPL text accompany the production source in `runtime/NOTICE`
and `runtime/COPYING`.

```bash
cargo run --example audit-division -- --variant runtime --samples 1024 --seed 0 \
  --exhaustive8 --output build/runtime-division/results.json \
  --artifacts build/runtime-division/artifacts \
  --csv wiki/audits/division16-runtime.csv
cargo run -- build examples/blank.rs build/blank.gb
```

All 149,329 production-entry executions passed: 18,245 ordinary corpus cases, 12
zero-divisor caller cases, and 131,072 executions covering every eight-bit
operand pair through each public entry. Unit tests also cover both entries and
their callers with seed 2, poisoned scratch registers, stack restoration, write
bounds, and zero-divisor behavior. The ROM smoke build passes.

The runtime measurements match the audit's compact candidate:

| Entry     | Narrow body clocks | Narrow caller clocks | Wide caller mean | Normal peak stack |
| --------- | -----------------: | -------------------: | ---------------: | ----------------: |
| quotient  |               1084 |                 1236 |           1133.3 |                 6 |
| remainder |               1088 |                 1240 |           1137.3 |                 6 |

Both entries use zero scratch RAM and zero table bytes. Zero-divisor callers
reach the trap vector with eight peak stack bytes. The caller timing here uses
the same memory-backed operand setup as the candidate audit; a caller packing
constant operands has different setup costs. Full per-class statistics are in
[division16-runtime.csv](division16-runtime.csv).
