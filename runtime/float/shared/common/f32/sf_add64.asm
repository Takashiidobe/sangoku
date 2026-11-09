INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_add64", ROM0


; hl -> 64-bit x, de -> 64-bit y: x += y

sf_add64::
    ld a, [de]
    add [hl]
    ld [hl+], a
    inc de
    FOR i, 7
    ld a, [de]
    adc [hl]
    ld [hl+], a
    inc de
    ENDR
    ret
