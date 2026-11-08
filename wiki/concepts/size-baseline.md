# ROM size baseline against SDCC

_created 2026-11-07_

Run `python3 benchmarks/size_baseline.py z 2`. It builds `benchmarks/empty.rs`
and the examples, attributes each retained ROM section to its defining crate,
and builds the C equivalents in `benchmarks/sdcc` with SDCC 4.6 `-msm83` and a
minimal `crt0.s`. SDCC bytes are `_HOME`+`_CODE`+`_INITIALIZER`+`_GSINIT`,
including library helpers such as `malloc`; the header is excluded.

| program | opt | asm | user | core | alloc | heap | total | SDCC |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| empty | z | 1283 | 3 | 0 | 0 | 0 | 1286 | 2 |
| hello | z | 1283 | 1319 | 4264 | 0 | 0 | 6866 | 67 |
| hello | 2 | 1283 | 976 | 0 | 0 | 0 | 2259 | 67 |
| primes | z | 1388 | 5164 | 16342 | 0 | 0 | 22894 | 355 |
| primes | 2 | 1388 | 3676 | 4425 | 0 | 0 | 9489 | 355 |
| sort | z | 1283 | 5248 | 11785 | 0 | 0 | 18316 | 613 |
| sort | 2 | 1283 | 4701 | 1759 | 0 | 0 | 7743 | 613 |
| heap | z | 1401 | 5540 | 28644 | 24996 | 3101 | 63682 | 2000 |
| heap | 2 | 1401 | 3384 | 3287 | 21236 | 2723 | 32031 | 2000 |

## Findings

- Core has no fixed cost; an empty program retains no core code. The 1283
  asm bytes are startup and header (~220) plus the 32-bit runtime (1066), which
  is linked unconditionally (`sangoku-30k.9.1.9.6`).
- At opt s/z, rustc does not MIR-inline and Cranelift does not inline, so
  trivial core wrappers stay calls (s and z are identical). Opt 2 removes most
  core bytes (`sangoku-30k.9.1.9.5`) but currently miscompiles the serial loop
  in hello/primes/sort (`sangoku-30k.9.1.9.4`); its sizes are a target, not
  working ROMs.
- Even fully inlined, user code is 10-15x SDCC: hello's `write_str` alone is
  855 bytes against 67 for the whole SDCC program. Spill and stack-slot code
  dominate (`sangoku-30k.9.1.9`).
- `alloc` adds 21-25 KB plus a 2.7-3 KB heap for one `Vec`/`Box` demo, against
  about 1.6 KB for SDCC `malloc`/`realloc`/`free`.
