INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_sub64", ROM0


; hl -> 64-bit x, de -> 64-bit y: x -= y

sf_sub64::
    ld a, [de]
    ld b, a
    ld a, [hl]
    sub b
    ld [hl+], a
    inc de
    FOR i, 7
    ld a, [de]
    ld b, a
    ld a, [hl]
    sbc b
    ld [hl+], a
    inc de
    ENDR
    ret
