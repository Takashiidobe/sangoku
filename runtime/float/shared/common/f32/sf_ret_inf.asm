INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_ret_inf", ROM0

sf_ret_inf::
    xor a
    ld [wR], a
    ld [wR + 1], a
    ld a, $80
    ld [wR + 2], a
    ld a, [wS]
    or $7f
    ld [wR + 3], a
    jp sf_return
