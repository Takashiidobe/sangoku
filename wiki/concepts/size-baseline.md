# ROM size baseline against SDCC

_created 2026-11-07_

Run `python3 benchmarks/size_baseline.py z 2`. It builds `benchmarks/empty.rs`
and the examples, attributes each retained ROM section to its defining crate,
and builds the C equivalents in `benchmarks/sdcc` with SDCC 4.6 `-msm83` and a
minimal `crt0.s`. SDCC bytes are `_HOME`+`_CODE`+`_INITIALIZER`+`_GSINIT`,
including library helpers such as `malloc`; the header is excluded.

Before MIR inlining was forced on (2026-11-07, opt z): hello 6866, primes 22894,
sort 18316, heap 63682.

With `-Zinline-mir=yes` at every level (`-Zinline-mir-hint-threshold=50` at opt
0), re-measured 2026-11-08 (the script now passes `--alloc` for heap and fails
on a build error; before, the heap row silently read a stale map):

| program | opt | asm | user | core | builtins | alloc | heap | total | SDCC |
| ------- | --- | --: | ---: | ---: | -------: | ----: | ---: | ----: | ---: |
| empty   | z   |  92 |    3 |    0 |        0 |     0 |    0 |    95 |    2 |
| empty   | 0   |  92 |    3 |    0 |        0 |     0 |    0 |    95 |    2 |
| hello   | z   | 315 | 3578 |    0 |        0 |     0 |    0 |  3893 |   67 |
| hello   | 0   | 358 | 1409 |  403 |        0 |     0 |    0 |  2170 |   67 |
| primes  | z   | 381 | 5615 | 3537 |        0 |     0 |    0 |  9533 |  355 |
| primes  | 0   | 401 | 3521 | 5855 |        0 |     0 |    0 |  9777 |  355 |
| sort    | z   | 403 | 8706 | 5740 |        0 |     0 |    0 | 14849 |  613 |
| sort    | 0   | 529 | 8315 | 8550 |        0 |     0 |    0 | 17394 |  613 |
| heap    | z   | 223 | 2746 | 2571 |        0 | 17224 | 2211 | 24975 | 2000 |
| heap    | 0   | 710 | 1848 | 6051 |        0 | 18912 | 2795 | 30316 | 2000 |

Only opt 0 and z were re-measured; opt 1/2/3/s are not.

## Findings

- Core has no fixed cost; an empty program retains no core code, and its 92 asm
  bytes are startup and header only.
- rustc only MIR-inlines by default at opt 2/3 and Cranelift never inlines, so
  `sangoku` passes `-Zinline-mir=yes` everywhere. At opt 0 Cranelift does not
  clean up inlined bodies, and the default hint threshold made one fixture's
  `gbc_main` exceed a 16 KiB bank, so opt 0 uses hint threshold 50. That costs
  size at opt 0 (core `sort_unstable` needs 57 KB there, but fits at z).
- Inlining exposed missing volatile flags: cg_clif emitted volatile loads and
  stores as plain memory accesses, so Cranelift forwarded a stored value into a
  serial polling loop. Scalar volatile accesses now set the Cranelift `volatile`
  flag (`tests/core_rom/volatile_poll.rs`).
- Even fully inlined, user code is 10-15x SDCC: hello's `write_str` alone is 855
  bytes against 67 for the whole SDCC program. Spill and stack-slot code
  dominate (`sangoku-30k.9.1.9`).
- `alloc` adds 17-19 KB plus a 2.2-2.8 KB heap for the `heap.rs` demo, against
  about 2 KB for the SDCC equivalent. `heap.rs` uses both `Global` and
  `&BumpAllocator`, so `RawVecInner::{finish_grow, grow_amortized, deallocate}`
  (2981 + 2273 + 737 bytes) are instantiated twice: 12 KB of the 17 KB. With a
  single allocator (`Vec` push twice, opt z) the program is 13.3 KB with 10.6 KB
  of `alloc`; a `Vec` alone is 10 KB of that, `Box` is nearly free.
- The large `alloc` functions are not missing inlining. 77% of `finish_grow`'s
  2243 instructions are `ld`, the same spill and stack-slot traffic as above
  (`sangoku-30k.9.1.9`); register allocation, not MIR inlining, is the lever.
