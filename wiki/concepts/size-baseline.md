# ROM size baseline against SDCC

_created 2026-11-07_

Run `python3 benchmarks/size_baseline.py z 2`. It builds `benchmarks/empty.rs`
and the examples, attributes each retained ROM section to its defining crate,
and builds the C equivalents in `benchmarks/sdcc` with SDCC 4.6 `-msm83` and a
minimal `crt0.s`. SDCC bytes are `_HOME`+`_CODE`+`_INITIALIZER`+`_GSINIT`,
including library helpers such as `malloc`; the header is excluded.

Before MIR inlining was forced on (2026-11-07, opt z): hello 6866, primes
22894, sort 18316, heap 63682.

With `-Zinline-mir=yes` at every level (`-Zinline-mir-hint-threshold=50` at
opt 0):

| program | opt | asm | user | core | alloc | heap | total | SDCC |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| empty | z | 1283 | 3 | 0 | 0 | 0 | 1286 | 2 |
| hello | 0 | 1283 | 1124 | 1209 | 0 | 0 | 3616 | 67 |
| hello | z | 1283 | 982 | 0 | 0 | 0 | 2265 | 67 |
| primes | 0 | 1388 | 4206 | 8709 | 0 | 0 | 14303 | 355 |
| primes | z | 1388 | 3743 | 4425 | 0 | 0 | 9556 | 355 |
| sort | 0 | 1283 | 4811 | 5312 | 0 | 0 | 11406 | 613 |
| sort | z | 1283 | 5735 | 1759 | 0 | 0 | 8777 | 613 |
| heap | 0 | 1401 | 4468 | 15370 | 29461 | 4682 | 55382 | 2000 |
| heap | z | 1401 | 3384 | 3287 | 21236 | 2723 | 32031 | 2000 |

opt 1/2/3/s produce the same bytes as z.

## Findings

- Core has no fixed cost; an empty program retains no core code. The 1283
  asm bytes are startup and header (~220) plus the 32-bit runtime (1066), which
  is linked unconditionally (`sangoku-30k.9.1.9.6`).
- rustc only MIR-inlines by default at opt 2/3 and Cranelift never inlines, so
  `sangoku` passes `-Zinline-mir=yes` everywhere. At opt 0 Cranelift does not
  clean up inlined bodies, and the default hint threshold made one fixture's
  `gbc_main` exceed a 16 KiB bank, so opt 0 uses hint threshold 50. That costs
  size at opt 0 (core `sort_unstable` needs 57 KB there, but fits at z).
- Inlining exposed missing volatile flags: cg_clif emitted volatile loads and
  stores as plain memory accesses, so Cranelift forwarded a stored value into a
  serial polling loop. Scalar volatile accesses now set the Cranelift
  `volatile` flag (`tests/core_rom/volatile_poll.rs`).
- Even fully inlined, user code is 10-15x SDCC: hello's `write_str` alone is
  855 bytes against 67 for the whole SDCC program. Spill and stack-slot code
  dominate (`sangoku-30k.9.1.9`).
- `alloc` adds 21-25 KB plus a 2.7-3 KB heap for one `Vec`/`Box` demo, against
  about 1.6 KB for SDCC `malloc`/`realloc`/`free`.
