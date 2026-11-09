INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_f16tof32", ROM0


; f16 bits zero-extended to i32 in, f32 bits out

__gbc_f16tof32::
    LOAD_A
    ld a, [wA + 1]
    and $80
    ld [wS], a
    ld a, [wA + 1]
    and $7c
    rrca
    rrca
    ld d, a
    ld a, [wA]
    ld [wM], a
    ld a, [wA + 1]
    and 3
    ld [wM + 1], a
    ld a, d
    cp 31
    jr z, .special
    or a
    jr nz, .normal
    ld a, [wM]
    ld b, a
    ld a, [wM + 1]
    or b
    jp z, sf_ret_zero
    ld a, 113
    ld [wE], a
.subnormal
    ld a, [wM + 1]
    bit 2, a
    jr nz, .subnormal_done
    ld hl, wM
    sla [hl]
    inc hl
    rl [hl]
    ld hl, wE
    dec [hl]
    jr .subnormal
.subnormal_done
    ld a, [wM + 1]
    and 3
    ld [wM + 1], a
    ld a, [wE]
    ld d, a
    jr .pack
.normal
    ld a, d
    add 112
    ld d, a
.pack
    ld a, [wM]
    and 7
    swap a
    sla a
    ld [wR + 1], a
    ld a, [wM + 1]
    rrca
    rrca
    rrca
    ld b, a
    ld a, [wM]
    rrca
    rrca
    rrca
    and $1f
    or b
    ld b, a
    ld a, d
    srl a
    ld c, a
    ld a, b
    jr nc, .store_high
    or $80
.store_high
    ld [wR + 2], a
    ld a, [wS]
    or c
    ld [wR + 3], a
    xor a
    ld [wR], a
    jp sf_return
.special
    ld a, [wM]
    ld b, a
    ld a, [wM + 1]
    or b
    jp nz, sf_ret_nan
    jp sf_ret_inf
