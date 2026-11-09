INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_shr1", ROM0


; hl -> 32-bit value; shift right one bit

sf_shr1::
    inc hl
    inc hl
    inc hl
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    ret
