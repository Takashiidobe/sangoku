INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fdiv32", ROM0



__gbc_fdiv32::
    LOAD_AB
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld d, a
    CLASSIFY wB
    cp 2
    jp z, sf_ret_nan
    ld e, a
    ld a, [wA + 3]
    ld b, a
    ld a, [wB + 3]
    xor b
    and $80
    ld [wS], a
    ld a, d
    or e
    jr z, .finite
    ld a, d
    or a
    jp z, sf_ret_zero
    ld a, e
    or a
    jp nz, sf_ret_nan
    jp sf_ret_inf
.finite
    ld hl, wB
    call sf_is_zero_mag
    jr nz, .divisor_nonzero
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_nan
    jp sf_ret_inf
.divisor_nonzero
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_zero
    UNPACK wA, wXa
    UNPACK wB, wXb
    NORMALIZE wXa
    NORMALIZE wXb
    ld a, [wXa + X_E]
    ld hl, wXb + X_E
    sub [hl]
    ld [wE], a
    ld a, [wXa + X_E + 1]
    inc hl
    sbc [hl]
    ld [wE + 1], a
    ld a, [wE]
    add 127
    ld [wE], a
    ld a, [wE + 1]
    adc 0
    ld [wE + 1], a
    FOR i, 4
    ld a, [wXa + X_M + i]
    ld [wRm + i], a
    ENDR
    ld hl, wXa + X_M
    ld de, wXb + X_M
    call sf_cmp32
    jr nc, .quotient
    ld hl, wRm
    call sf_shl1
    call sf_dec_e
.quotient
    xor a
    ld [wM], a
    ld [wM + 1], a
    ld [wM + 2], a
    ld [wM + 3], a
    ld a, 26
    ld [wCnt], a
.divide
    ld hl, wM
    call sf_shl1
    ld hl, wRm
    ld de, wXb + X_M
    call sf_cmp32
    jr c, .no_subtract
    ld hl, wRm
    ld de, wXb + X_M
    call sf_sub32
    ld hl, wM
    set 0, [hl]
.no_subtract
    ld hl, wRm
    call sf_shl1
    ld hl, wCnt
    dec [hl]
    jr nz, .divide
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
