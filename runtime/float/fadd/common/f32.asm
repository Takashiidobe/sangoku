INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fadd32", ROM0



__gbc_fadd32::
    LOAD_AB
    CLASSIFY wA
    cp 2
    jp z, sf_ret_nan
    ld d, a
    CLASSIFY wB
    cp 2
    jp z, sf_ret_nan
    ld e, a
    ld a, d
    or e
    jr z, .finite
    ld a, d
    or a
    jp z, sf_ret_b
    ld a, e
    or a
    jp z, sf_ret_a
    ld a, [wA + 3]
    ld b, a
    ld a, [wB + 3]
    xor b
    and $80
    jp nz, sf_ret_nan
    jp sf_ret_a
.finite
    UNPACK wA, wXa
    UNPACK wB, wXb
    ld a, [wXa + X_E]
    ld b, a
    ld a, [wXb + X_E]
    cp b
    jr c, .ordered
    jr nz, .swap
    ld hl, wXa + X_M
    ld de, wXb + X_M
    call sf_cmp32
    jr nc, .ordered
.swap
    call sf_swap_x
.ordered
    ld a, [wXb + X_E]
    ld b, a
    ld a, [wXa + X_E]
    sub b
    jr z, .aligned
    ld b, a
    cp 32
    jr nc, .jam
    ld hl, wXb + X_M
.align
    call sf_shr1s
    dec b
    jr nz, .align
    jr .aligned
.jam
    ld hl, wXb + X_M
    call sf_is_zero32
    ld a, 0
    jr z, .store_jam
    ld a, 1
.store_jam
    ld hl, wXb + X_M
    ld [hl+], a
    xor a
    ld [hl+], a
    ld [hl+], a
    ld [hl], a
.aligned
    ld a, [wXa + X_E]
    ld [wE], a
    xor a
    ld [wE + 1], a
    ld a, [wXa + X_S]
    ld [wS], a
    ld b, a
    ld a, [wXb + X_S]
    xor b
    jr nz, .subtract
    ld hl, wXa + X_M
    ld de, wXb + X_M
    ld a, [de]
    add [hl]
    ld [wM], a
    inc de
    inc hl
    ld a, [de]
    adc [hl]
    ld [wM + 1], a
    inc de
    inc hl
    ld a, [de]
    adc [hl]
    ld [wM + 2], a
    inc de
    inc hl
    ld a, [de]
    adc [hl]
    ld [wM + 3], a
    bit 7, a
    jr z, .normalize
    ld hl, wM
    call sf_shr1s
    call sf_inc_e
    jr .normalize
.subtract
    ld hl, wXa + X_M
    ld de, wXb + X_M
    ld a, [hl]
    ld b, a
    ld a, [de]
    ld c, a
    ld a, b
    sub c
    ld [wM], a
    inc de
    inc hl
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [wM + 1], a
    inc de
    inc hl
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [wM + 2], a
    inc de
    inc hl
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [wM + 3], a
    xor a
    ld [wS], a
    ld hl, wM
    call sf_is_zero32
    jr z, .zero
    ld a, [wXa + X_S]
    ld [wS], a
.normalize
    ld hl, wM
    call sf_is_zero32
    jr z, .zero
.step
    ld a, [wM + 3]
    bit 6, a
    jr nz, .done
    ld a, [wE]
    dec a
    ld b, a
    ld a, [wE + 1]
    or b
    jr z, .done
    ld hl, wM
    call sf_shl1
    call sf_dec_e
    jr .step
.done
    jp sf_roundpack
.zero
    jp sf_ret_zero
