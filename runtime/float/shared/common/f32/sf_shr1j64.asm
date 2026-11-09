INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_shr1j64", ROM0


; hl -> 64-bit value; shift right one bit, jamming the lost bit into bit 0

sf_shr1j64::
    FOR i, 7
    inc hl
    ENDR
    srl [hl]
    FOR i, 7
    dec hl
    rr [hl]
    ENDR
    ret nc
    set 0, [hl]
    ret
