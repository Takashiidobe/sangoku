INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_ffma32", ROM0


; a * b + c with a single rounding; fourth argument byte: 1 rounds the result
; to odd at f32 precision (for f16 callers that demote afterwards)

__gbc_ffma32::
    LOAD_AB
    ld hl, sp + 7
    ld a, [hl+]
    ld [wC], a
    ld a, [hl+]
    ld [wC + 1], a
    ld a, [hl+]
    ld [wC + 2], a
    ld a, [hl+]
    ld [wC + 3], a
    ld a, [hl]
    ld [wMask], a
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld [wKa], a
    CLASSIFY wB
    cp 2
    jp z, sf_ret_nan
    ld [wKb], a
    CLASSIFY wC
    cp 2
    jp z, sf_ret_nan
    ld [wKc], a
    ld a, [wA + 3]
    ld b, a
    ld a, [wB + 3]
    xor b
    and $80
    ld [wSp], a
    ld a, [wC + 3]
    and $80
    ld [wSc], a
    ld a, [wKa]
    ld b, a
    ld a, [wKb]
    or b
    jr z, .product_finite
    ld a, [wKa]
    or a
    jr nz, .a_infinite
    ld hl, wA
    call sf_is_zero_mag
    jp z, sf_ret_nan
    jr .product_infinite
.a_infinite
    ld a, [wKb]
    or a
    jr nz, .product_infinite
    ld hl, wB
    call sf_is_zero_mag
    jp z, sf_ret_nan
.product_infinite
    ld a, [wKc]
    or a
    jr z, .return_product_infinity
    ld a, [wSp]
    ld b, a
    ld a, [wSc]
    xor b
    jp nz, sf_ret_nan
.return_product_infinity
    ld a, [wSp]
    ld [wS], a
    jp sf_ret_inf
.product_finite
    ld a, [wKc]
    or a
    jp nz, sf_ret_c
    ld hl, wA
    call sf_is_zero_mag
    jr z, .product_zero
    ld hl, wB
    call sf_is_zero_mag
    jr nz, .nonzero_product
.product_zero
    ld hl, wC
    call sf_is_zero_mag
    jp nz, sf_ret_c
    ld a, [wSp]
    ld b, a
    ld a, [wSc]
    xor b
    jr z, .same_zero_sign
    xor a
    ld [wS], a
    jp sf_ret_zero
.same_zero_sign
    ld a, b
    ld [wS], a
    jp sf_ret_zero
.nonzero_product
    UNPACK wA, wXa
    UNPACK wB, wXb
    NORMALIZE wXa
    NORMALIZE wXb
    ld a, [wXa + X_E]
    ld hl, wXb + X_E
    add [hl]
    ld [wEp], a
    ld a, [wXa + X_E + 1]
    inc hl
    adc [hl]
    ld [wEp + 1], a
    ld a, [wEp]
    sub 127
    ld [wEp], a
    ld a, [wEp + 1]
    sbc 0
    ld [wEp + 1], a
    call sf_mul48
    xor a
    FOR i, 8
    ld [wFp + i], a
    ld [wFc + i], a
    ENDR
    FOR i, 6
    ld a, [wP + i]
    ld [wFp + 2 + i], a
    ENDR
    ld hl, wFp
    call sf_shr1j64
    ld hl, wC
    call sf_is_zero_mag
    jr nz, .addend_nonzero
    ld a, [wSp]
    ld [wSc], a
    ld a, [wEp]
    ld [wEc], a
    ld a, [wEp + 1]
    ld [wEc + 1], a
    jp .have_addend
.addend_nonzero
    UNPACK wC, wXb
    NORMALIZE wXb
    ld a, [wXb + X_E]
    ld [wEc], a
    ld a, [wXb + X_E + 1]
    ld [wEc + 1], a
    FOR i, 4
    ld a, [wXb + X_M + i]
    ld [wFc + 4 + i], a
    ENDR
    ld hl, wFc
    call sf_shr1j64
.have_addend
    ld a, [wEp]
    ld hl, wEc
    sub [hl]
    ld c, a
    ld a, [wEp + 1]
    inc hl
    sbc [hl]
    ld b, a
    bit 7, b
    jr nz, .product_smaller
    ld a, [wEp]
    ld [wE], a
    ld a, [wEp + 1]
    ld [wE + 1], a
    ld hl, wFc
    call sf_shift_jam
    jr .aligned
.product_smaller
    ld a, c
    cpl
    add 1
    ld c, a
    ld a, b
    cpl
    adc 0
    ld b, a
    ld a, [wEc]
    ld [wE], a
    ld a, [wEc + 1]
    ld [wE + 1], a
    ld hl, wFp
    call sf_shift_jam
.aligned
    ld a, [wSp]
    ld b, a
    ld a, [wSc]
    xor b
    jr nz, .subtract
    ld a, b
    ld [wS], a
    ld hl, wFp
    ld de, wFc
    call sf_add64
    jr .normalize
.subtract
    ld hl, wFp
    ld de, wFc
    call sf_cmp64
    jr z, .exact_zero
    jr c, .addend_larger
    ld a, [wSp]
    ld [wS], a
    ld hl, wFp
    ld de, wFc
    call sf_sub64
    jr .normalize
.addend_larger
    ld a, [wSc]
    ld [wS], a
    ld hl, wFc
    ld de, wFp
    call sf_sub64
    FOR i, 8
    ld a, [wFc + i]
    ld [wFp + i], a
    ENDR
    jr .normalize
.exact_zero
    xor a
    ld [wS], a
    jp sf_ret_zero
.normalize
    ld a, [wFp + 7]
    bit 7, a
    jr nz, .leading
    ld hl, wFp
    call sf_shl64
    call sf_dec_e
    jr .normalize
.leading
    call sf_inc_e
    call sf_inc_e
    FOR i, 4
    ld a, [wFp + 4 + i]
    ld [wM + i], a
    ENDR
    ld hl, wM
    call sf_shr1s
    ld hl, wFp
    call sf_is_zero32
    jr z, .odd_check
    ld hl, wM
    set 0, [hl]
.odd_check
    ld a, [wMask]
    or a
    jp z, sf_roundpack
    ld a, [wM]
    and $7f
    jp z, sf_roundpack
    ld a, [wM]
    and $80
    or $80
    ld [wM], a
    jp sf_roundpack
