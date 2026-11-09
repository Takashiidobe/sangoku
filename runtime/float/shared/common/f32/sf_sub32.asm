INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_sub32", ROM0


; hl -> 32-bit x, de -> 32-bit y: x -= y

sf_sub32::
    ld a, [de]
    ld b, a
    ld a, [hl]
    sub b
    ld [hl+], a
    inc de
    ld a, [de]
    ld b, a
    ld a, [hl]
    sbc b
    ld [hl+], a
    inc de
    ld a, [de]
    ld b, a
    ld a, [hl]
    sbc b
    ld [hl+], a
    inc de
    ld a, [de]
    ld b, a
    ld a, [hl]
    sbc b
    ld [hl], a
    ret
