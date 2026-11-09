# Register allocation audit against SDCC

_created 2026-11-09_

Run `python3 benchmarks/regalloc_baseline.py` to compile
`benchmarks/regalloc/kernels.rs` with this backend and `kernels.c` with SDCC
4.6 `-msm83 --opt-code-size`, then print one row per kernel and compiler. Add a
kernel by adding a function with the same name to both files. Rust kernels must
not call core helpers or bounds-check, so the comparison measures code
generation and not inlining; opt 2 and z are intentionally identical because
size needs aggressive inlining.

- `bytes`: exact bytes from `rgbasm`/`rgblink` (Rust) and `sdasgb -l` (SDCC).
- `insns`: instructions, not counting raw `db` runs.
- `moves`: 8-bit register-to-register `ld`.
- `stack`: instructions that mention `sp`, plus `push` and `pop`.
- `jp`: absolute jumps that a `jr` could often replace.

`benchmarks/regalloc_baseline.json` stores the last accepted Rust and SDCC rows.
`--check` prints every metric that changed and exits non-zero if a kernel grew;
`--update` rewrites the baseline. Update it in the commit that improves a
kernel, and add a row to the history below.

Each Rust function is a separate section, so a call to a helper would add an
undefined symbol; the script stubs those to link, and the helper's bytes are not
counted. Keep kernels call-free.

## History

| change | rust bytes | sdcc bytes | ratio |
| ------ | ---------: | ---------: | ----: |
| baseline (2026-11-09) | 1190 | 426 | 2.79 |
| sangoku-dq5: offset-0 byte loads/stores via `ld a,[de]`/`ld [de],a` | 1153 | 426 | 2.71 |

## Findings (2026-11-09)

The allocator's own losses are small next to lowering. regalloc2 sees seven
independent byte registers, no scratch register, one-byte SP-relative spill
slots and a call that clobbers everything (`abi.rs`, `get_machine_env`). Pairs
such as HL are enforced by fixed-register constraints in lowering.

Allocator and spill losses:

- Reloading a byte while HL is live costs `push hl; ld hl,sp+N; ld c,[hl];
  pop hl`, and `push af; ld a,h; ... pop af` when A is live too. (sangoku-e6w)
- Loop-carried values are stored and reloaded on the back edge, an A-passed
  argument is stored at entry while D/E/H/L are free, and `ld d, d` is emitted.
  (sangoku-1de)
- Every pointer load copies the pointer into HL, which doubles its pressure.
  (sangoku-dq5)

Lowering losses that dominate the byte gap:

- `p.add(1)` and `n -= 1` go through A instead of `inc`/`dec`. (sangoku-2mv)
- Comparisons build a boolean from a raw `db` run before branching.
  (sangoku-i70)
- `jp` where `jr` fits, and `ret` placed in the fall-through. (sangoku-2ea)
- Moves through A where `ld [hl], r` exists, and no `[hl]` ALU operands.
  (sangoku-98d)
- Rotate by a constant is repeated `sla/rl/set` instead of `swap`.
  (sangoku-21y)
- Arguments past the third byte arrive on the stack, which is an ABI choice.

Work is tracked under epic sangoku-otw.

## sangoku-dq5: byte loads and stores through DE

Byte loads and stores with offset 0 now emit `ld a,[de]` / `ld [de],a` instead
of copying the pointer into HL. The pointer is a fixed D/E use, a load defines
A directly and a store uses A directly, so H, L and a scratch A are no longer
clobbered. Other offsets and 16-bit accesses still use the HL path.

| kernel | before | after |
| ------ | -----: | ----: |
| strlen16 | 83 | 64 |
| max_u8 | 100 | 78 |
| fill | 47 | 43 |
| copy_u8 | 131 | 130 |
| count_eq | 99 | 104 |
| sum_u8 | 100 | 104 |

`count_eq` and `sum_u8` grew. Their loops still build `p.add(1)` and `n -= 1`
through A with constant registers (sangoku-2mv) and `count_eq` compares through
a fixed-register `db` sequence (sangoku-i70), so the allocator now has A and DE
pinned on top of that pressure. Re-measure both after those land.

DE and not BC because the byte ABI passes a 16-bit argument as low=B, high=C,
the reverse of hardware BC, so a BC load would need a swap for every pointer
argument. DE costs one `ld d,c; ld e,b` at entry. A pointer that is accessed
at both offset 0 (DE) and another offset (BC/HL path) is shuffled between them;
see `load-store-i32.clif`.
