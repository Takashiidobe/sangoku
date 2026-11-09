INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fround32", ROM0



; mode byte after the float argument: 0 trunc, 1 floor, 2 ceil, 3 nearest even

__gbc_fround32::
    LOAD_A
    ld hl, sp + 3
    ld a, [hl]
    ld [wMask], a
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    or a
    jp nz, sf_ret_a
    ld a, [wA + 3]
    and $80
    ld [wS], a
    ld a, [wA + 3]
    add a
    ld b, a
    ld a, [wA + 2]
    rlca
    ld a, b
    adc 0
    ld d, a
    cp 150
    jp nc, sf_ret_a
    cp 127
    jp c, .small
    ld a, 150
    sub d
    ld c, a
    xor a
    FOR i, 4
    ld [wMk + i], a
    ENDR
.build_mask
    ld hl, wMk
    call sf_shl1
    ld hl, wMk
    set 0, [hl]
    dec c
    jr nz, .build_mask
    ld a, [wA]
    ld [wM], a
    ld a, [wA + 1]
    ld [wM + 1], a
    ld a, [wA + 2]
    ld [wM + 2], a
    ld a, [wA + 3]
    and $7f
    ld [wM + 3], a
    FOR i, 4
    ld a, [wM + i]
    ld b, a
    ld a, [wMk + i]
    and b
    ld [wFr + i], a
    ENDR
    ld hl, wFr
    call sf_is_zero32
    jp z, sf_ret_a
    ld a, [wMk]
    add 1
    ld [wHf], a
    FOR i, 1, 4
    ld a, [wMk + i]
    adc 0
    ld [wHf + i], a
    ENDR
    ld a, [wMask]
    or a
    jp z, .down
    cp 1
    jp z, .floor
    cp 2
    jp z, .ceil
    ld hl, wFr
    call sf_shl1
    ld hl, wFr
    ld de, wHf
    call sf_cmp32
    jp c, .down
    jp nz, .up
    TIE_BYTE 0
    ld c, a
    TIE_BYTE 1
    or c
    ld c, a
    TIE_BYTE 2
    or c
    ld c, a
    TIE_BYTE 3
    or c
    jp nz, .up
    jr .down
.floor
    ld a, [wS]
    or a
    jp nz, .up
    jr .down
.ceil
    ld a, [wS]
    or a
    jr z, .up
.down
    call sf_clear_frac
    jr .emit
.up
    call sf_clear_frac
    FOR i, 4
    ld a, [wMk + i]
    ld b, a
    ld a, [wM + i]
    or b
    ld [wM + i], a
    ENDR
    ld hl, wM
    inc [hl]
    jp nz, .emit
    inc hl
    inc [hl]
    jp nz, .emit
    inc hl
    inc [hl]
    jp nz, .emit
    inc hl
    inc [hl]
.emit
    ld a, [wM]
    ld [wR], a
    ld a, [wM + 1]
    ld [wR + 1], a
    ld a, [wM + 2]
    ld [wR + 2], a
    ld a, [wS]
    ld b, a
    ld a, [wM + 3]
    or b
    ld [wR + 3], a
    jp sf_return
.small
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_a
    ld a, [wMask]
    or a
    jp z, sf_ret_zero
    cp 1
    jr z, .small_floor
    cp 2
    jr z, .small_ceil
    ld a, d
    cp 126
    jp nz, sf_ret_zero
    ld hl, wA
    ld a, [hl+]
    or [hl]
    inc hl
    ld b, a
    ld a, [hl]
    and $7f
    or b
    jp z, sf_ret_zero
.one
    xor a
    ld [wR], a
    ld [wR + 1], a
    ld a, $80
    ld [wR + 2], a
    ld a, [wS]
    or $3f
    ld [wR + 3], a
    jp sf_return
.small_floor
    ld a, [wS]
    or a
    jr nz, .one
    jp sf_ret_zero
.small_ceil
    ld a, [wS]
    or a
    jr z, .one
    jp sf_ret_zero
