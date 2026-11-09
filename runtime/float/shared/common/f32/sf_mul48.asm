INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_mul48", ROM0


; wXa/wXb normalized: wP (48 bits) = significand product

sf_mul48::
    ld hl, wXa + X_M
    call sf_shl1
    ld a, [wXa + X_M + 1]
    ld [wMc], a
    ld a, [wXa + X_M + 2]
    ld [wMc + 1], a
    ld a, [wXa + X_M + 3]
    ld [wMc + 2], a
    ld hl, wXb + X_M
    call sf_shl1
    ld a, [wXb + X_M + 1]
    ld [wP], a
    ld a, [wXb + X_M + 2]
    ld [wP + 1], a
    ld a, [wXb + X_M + 3]
    ld [wP + 2], a
    xor a
    ld [wP + 3], a
    ld [wP + 4], a
    ld [wP + 5], a
    ld b, 24
.multiply
    ld hl, wP
    bit 0, [hl]
    jr z, .no_add
    ld hl, wP + 3
    ld de, wMc
    ld a, [de]
    add [hl]
    ld [hl+], a
    inc de
    ld a, [de]
    adc [hl]
    ld [hl+], a
    inc de
    ld a, [de]
    adc [hl]
    ld [hl], a
    jr .shift
.no_add
    and a
.shift
    ld hl, wP + 5
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec b
    jr nz, .multiply
    ret
