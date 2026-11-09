INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_ret_c", ROM0

sf_ret_c::
    ld hl, wC
    jp sf_ret_hl
