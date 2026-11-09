INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fsqrt32", ROM0



__gbc_fsqrt32::
    LOAD_A
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld d, a
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_a
    ld a, [wA + 3]
    bit 7, a
    jp nz, sf_ret_nan
    ld a, d
    or a
    jp nz, sf_ret_a
    xor a
    ld [wS], a
    UNPACK wA, wXa
    NORMALIZE wXa
    ld a, [wXa + X_E]
    bit 0, a
    jr nz, .exponent_odd
    ld hl, wXa + X_M
    call sf_shl1
    ld a, 126
    jr .have_bias
.exponent_odd
    ld a, 127
.have_bias
    ld hl, wXa + X_E
    add [hl]
    ld [wE], a
    inc hl
    ld a, [hl]
    adc 0
    sra a
    ld [wE + 1], a
    ld a, [wE]
    rra
    ld [wE], a
    xor a
    FOR i, 4
    ld [wRad + i], a
    ld [wRm + i], a
    ld [wM + i], a
    ENDR
    FOR i, 4
    ld a, [wXa + X_M + i]
    ld [wRad + 4 + i], a
    ENDR
    ld a, 26
    ld [wCnt], a
.root
    REPT 2
    ld hl, wRad
    call sf_shl64
    ld hl, wRm
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    ENDR
    FOR i, 4
    ld a, [wM + i]
    ld [wFr + i], a
    ENDR
    ld hl, wFr
    call sf_shl1
    ld hl, wFr
    call sf_shl1
    ld hl, wFr
    set 0, [hl]
    ld hl, wM
    call sf_shl1
    ld hl, wRm
    ld de, wFr
    call sf_cmp32
    jr c, .next_digit
    ld hl, wRm
    ld de, wFr
    call sf_sub32
    ld hl, wM
    set 0, [hl]
.next_digit
    ld hl, wCnt
    dec [hl]
    jr nz, .root
    ld hl, wM
    REPT 5
    call sf_shl1
    ld hl, wM
    ENDR
    ld hl, wRm
    call sf_is_zero32
    jp z, sf_roundpack
    ld hl, wM
    set 0, [hl]
    jp sf_roundpack
