INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_ret_b", ROM0

sf_ret_b::
    ld hl, wB
    jp sf_ret_hl
