INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_ret_a", ROM0



sf_ret_a::
    ld hl, wA
    jp sf_ret_hl
