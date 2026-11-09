INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_inc_e", ROM0



sf_inc_e::
    ld hl, wE
    inc [hl]
    ret nz
    inc hl
    inc [hl]
    ret
