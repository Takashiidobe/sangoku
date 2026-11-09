INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_swap_x", ROM0



sf_swap_x::
    ld hl, wXa
    ld de, wXb
    ld c, 7
.next
    ld a, [de]
    ld b, [hl]
    ld [hl+], a
    ld a, b
    ld [de], a
    inc de
    dec c
    jr nz, .next
    ret
