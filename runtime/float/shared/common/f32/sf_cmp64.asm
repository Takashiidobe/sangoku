INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_cmp64", ROM0


; unsigned compare of the 64-bit values at hl (x) and de (y): C if x < y, Z if equal

sf_cmp64::
    FOR i, 7
    inc hl
    inc de
    ENDR
    ld c, 8
.next
    ld a, [de]
    ld b, a
    ld a, [hl]
    cp b
    ret nz
    dec hl
    dec de
    dec c
    jr nz, .next
    ret
