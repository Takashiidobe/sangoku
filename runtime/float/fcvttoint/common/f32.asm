INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fcvttoint32", ROM0


; f32 argument, then a mode byte: bit 0 unsigned, bit 1 saturating,
; bits 2-3 result width code (0 = 8, 1 = 16, 2 = 32 bits). Out of range or
; NaN traps unless saturating; the result is a sign-extended i32.

__gbc_fcvttoint32::
    LOAD_A
    ld hl, sp + 3
    ld a, [hl]
    ld [wMask], a
    CLASSIFY wA
    cp 2
    jp z, .nan
    ld d, a
    ld a, [wA + 3]
    and $80
    ld [wS], a
    ld a, [wMask]
    rrca
    rrca
    and 3
    ld b, a
    xor a
    ld [wMk + 1], a
    ld [wMk + 2], a
    ld [wMk + 3], a
    ld a, $ff
    ld [wMk], a
    ld a, b
    or a
    jr z, .mask_done
    ld a, $ff
    ld [wMk + 1], a
    ld a, b
    cp 2
    jr c, .mask_done
    ld a, $ff
    ld [wMk + 2], a
    ld [wMk + 3], a
.mask_done
    ld a, [wMask]
    bit 0, a
    jr nz, .unsigned_limit
    ld hl, wMk + 3
    srl [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    dec hl
    rr [hl]
    ld a, [wS]
    or a
    jr z, .have_limit
    ld hl, wMk
    inc [hl]
    jr nz, .have_limit
    inc hl
    inc [hl]
    jr nz, .have_limit
    inc hl
    inc [hl]
    jr nz, .have_limit
    inc hl
    inc [hl]
    jr .have_limit
.unsigned_limit
    ld a, [wS]
    or a
    jr z, .have_limit
    xor a
    FOR i, 4
    ld [wMk + i], a
    ENDR
.have_limit
    ld a, d
    or a
    jr nz, .overflow
    ld a, [wA + 3]
    add a
    ld b, a
    ld a, [wA + 2]
    rlca
    ld a, b
    adc 0
    ld d, a
    cp 127
    jr c, .zero
    cp 159
    jr nc, .overflow
    ld a, [wA]
    ld [wM], a
    ld a, [wA + 1]
    ld [wM + 1], a
    ld a, [wA + 2]
    and $7f
    or $80
    ld [wM + 2], a
    xor a
    ld [wM + 3], a
    ld a, d
    sub 127
    cp 23
    jr c, .shift_right
    sub 23
    jr z, .have_magnitude
    ld c, a
.shift_left
    ld hl, wM
    call sf_shl1
    dec c
    jr nz, .shift_left
    jr .have_magnitude
.shift_right
    ld c, a
    ld a, 23
    sub c
    ld c, a
.shift_right_loop
    ld hl, wM
    call sf_shr1
    dec c
    jr nz, .shift_right_loop
.have_magnitude
    ld hl, wM
    ld de, wMk
    call sf_cmp32
    jr c, .emit
    jr z, .emit
.overflow
    ld a, [wMask]
    bit 1, a
    jr nz, .saturate
    rst $10
.saturate
    FOR i, 4
    ld a, [wMk + i]
    ld [wM + i], a
    ENDR
    jr .emit
.zero
    xor a
    FOR i, 4
    ld [wM + i], a
    ENDR
.emit
    ld a, [wS]
    or a
    jr z, .return
    ld a, [wM]
    cpl
    add 1
    ld [wM], a
    FOR i, 1, 4
    ld a, [wM + i]
    cpl
    adc 0
    ld [wM + i], a
    ENDR
.return
    ld hl, wM
    jp sf_ret_hl
.nan
    ld a, [wMask]
    bit 1, a
    jr nz, .nan_zero
    rst $10
.nan_zero
    xor a
    ld b, a
    ld c, a
    ld d, a
    ret
