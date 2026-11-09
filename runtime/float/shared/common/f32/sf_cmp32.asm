INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_cmp32", ROM0


; unsigned compare of the 32-bit values at hl (x) and de (y): C set if x < y, Z if equal

sf_cmp32::
    inc hl
    inc hl
    inc hl
    inc de
    inc de
    inc de
    ld c, 4
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
