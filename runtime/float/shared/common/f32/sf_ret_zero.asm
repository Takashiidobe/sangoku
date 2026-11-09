INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_ret_zero", ROM0


; signed zero with the sign in wS

sf_ret_zero::
    xor a
    ld [wR], a
    ld [wR + 1], a
    ld [wR + 2], a
    ld a, [wS]
    ld [wR + 3], a
    jp sf_return
