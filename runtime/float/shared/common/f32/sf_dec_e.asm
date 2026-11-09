INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_dec_e", ROM0



sf_dec_e::
    ld hl, wE
    jp sf_dec16
