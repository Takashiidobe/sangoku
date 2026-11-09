INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fmul32", ROM0



__gbc_fmul32::
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
    jr nz, .a_infinite
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_nan
    jp sf_ret_inf
.a_infinite
    ld a, e
    or a
    jp nz, sf_ret_inf
    ld hl, wB
    call sf_is_zero_mag
    jp z, sf_ret_nan
    jp sf_ret_inf
.finite
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_zero
    ld hl, wB
    call sf_is_zero_mag
    jp z, sf_ret_zero
    UNPACK wA, wXa
    UNPACK wB, wXb
    NORMALIZE wXa
    NORMALIZE wXb
    ld a, [wXa + X_E]
    ld hl, wXb + X_E
    add [hl]
    ld [wE], a
    ld a, [wXa + X_E + 1]
    inc hl
    adc [hl]
    ld [wE + 1], a
    ld a, [wE]
    sub 127
    ld [wE], a
    ld a, [wE + 1]
    sbc 0
    ld [wE + 1], a
    call sf_mul48
    ld a, [wP + 2]
    ld [wM], a
    ld a, [wP + 3]
    ld [wM + 1], a
    ld a, [wP + 4]
    ld [wM + 2], a
    ld a, [wP + 5]
    ld [wM + 3], a
    bit 7, a
    jr z, .sticky
    ld hl, wM
    call sf_shr1s
    call sf_inc_e
.sticky
    ld a, [wP]
    ld b, a
    ld a, [wP + 1]
    or b
    jr z, .round
    ld hl, wM
    set 0, [hl]
.round
    jp sf_roundpack
