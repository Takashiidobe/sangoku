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
| sangoku-2mv: `inc`/`dec` for constant +-1 on bytes and words | 942 | 426 | 2.21 |
| sangoku-i70: fused `cp` + `jp cc` for unsigned/eq/ne byte compares feeding `brif` | 868 | 426 | 2.04 |

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
- Signed byte, 16-bit and 32-bit comparisons still build a boolean from a raw
  `db` run before branching; unsigned/eq/ne byte compares are fused
  (sangoku-i70).
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

## sangoku-2mv: inc and dec for constant steps

`iadd`/`isub` by 1 or -1 (either operand order for `iadd`) lowers to
`IncDec8` and `IncDec16` instead of loading a constant register and adding
through A. A byte step is `inc r` / `dec r`. A word step is `inc rr` / `dec rr`
when the allocator placed the value in BC, DE or HL (low, high in C/B, E/D,
L/H); otherwise `inc lo; jr nz,@+3; inc hi`, and for decrement `inc lo; dec lo;
jr nz,@+3; dec hi; dec lo` (the `inc`/`dec` pair sets Z when `lo` was zero
without touching A). Both reuse their source register, so they add no move and
no A clobber. `unit_step` in `lower.rs` does the constant match.

| kernel | before | after |
| ------ | -----: | ----: |
| copy_u8 | 130 | 53 |
| fill | 43 | 27 |
| strlen16 | 64 | 33 |
| count_eq | 104 | 77 |
| sum_u8 | 104 | 75 |
| max_u8 | 78 | 69 |
| sum_u16 | 133 | 117 |
| fib16 | 40 | 34 |

This also removed the `sum_u8` and `count_eq` regressions from sangoku-dq5.
Most of the saving is the constant registers it no longer needs, which was
pressure on the allocator, so loops that spilled now stay in registers.

Left over in these loops: the word increment still lands in a scratch pair and
is moved back (`ld c,d; inc e; jr nz; inc c; ld d,c`) instead of `inc de`, a
no-op `ld a, a` follows each `ld a,[de]`, and the loop test is `ld a,b; or a`
where `dec b` already set Z (sangoku-1de, sangoku-i70).

`tests/core_rom/unit_steps.rs` runs word and byte steps across the 0xFF and
0x00 boundaries on the emulator at opt 0, 2 and z. The backend's own unit tests
in `isa/lr35902` did not compile before this change (missing imports in the
`emit.rs` test module); they now do and pass.

## sangoku-i70: fused byte compare and branch

A `brif` whose condition is a single-use 8-bit `icmp` with an equality or
unsigned condition lowers to `CompareBranch8`: `ld a,lhs; cp rhs` then `jp cc`
on the flags (`lower.rs` `fuse_compare_branch`). Only A is clobbered, so B, C
and H are no longer pinned by the compare. `>` and `<=` use two conditional
jumps plus a final `jp`. Still unfused: signed byte, 16-bit and 32-bit compares,
and compares with more than one use.

Case: `if x > m { m = x }`. Rule: `Brif(Icmp(i8, eq/ne/unsigned))` with a
unique-use icmp was `Compare8` plus `BranchIf`, now `CompareBranch8`.

Before:

```asm
db $79, $BD, ...   ; 0/1 into L
ld b, c
ld a, l
or a
jp nz, .block6
```

After:

```asm
ld a, h
cp c
jp z, .block5
jp nc, .block6
```

The boolean no longer pins L, B and A, and the second test is gone.

| kernel | before | after |
| ------ | -----: | ----: |
| max_u8 | 69 | 40 |
| count_eq | 77 | 45 |
| sat_chain | 143 | 130 |

Remaining in the fused sequences: `jp` where `jr` fits (sangoku-2ea), and
`ld a,a` is skipped but `ld a,r` before `cp` is not folded when the left
operand could already be in A.

## sangoku-e6w: omit dead-A spill saves

Stack loads into H or L preserve the other half through A. When A is dead, the
emitter now omits the AF save and restore. Stores from H or L also omit the AF
save when A is dead, while preserving live HL. Each affected access saves two
bytes and a push/pop pair; stack offsets account for the omitted push. Live A
still receives the original preservation sequence.

Spills remain relative to each invocation's SP, so recursion and interrupt
reentry do not share scratch storage. Static or HRAM slots need a separate
allocation and ownership scheme before they can replace stack slots safely.
Reserving a scratch register would reduce the allocator's seven-byte register
pool; this change uses existing liveness instead.

Run `cargo run --example check-spills` to compare native emission with RGBDS and
execute the accesses for live/dead H, L and A at short and large offsets. The
default unknown-liveness path continues to preserve all scratch registers.

Measured against the stored baseline: `pressure` shrinks from 66 to 64 bytes,
`pressure16` from 182 to 178, and `sum_u16` from 117 to 115. No kernel grows.
