INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_cmp_mag", ROM0


; compare |wA| with |wB|: C set if smaller, Z if equal

sf_cmp_mag::
    ld hl, wA + 3
    ld de, wB + 3
    ld a, [de]
    and $7f
    ld b, a
    ld a, [hl]
    and $7f
    cp b
    ret nz
    ld c, 3
.next
    dec hl
    dec de
    ld a, [de]
    ld b, a
    ld a, [hl]
    cp b
    ret nz
    dec c
    jr nz, .next
    ret
