INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_fcvtfromint32", ROM0


; i32 argument, then a byte: 1 when the integer is signed

__gbc_fcvtfromint32::
    LOAD_A
    ld hl, sp + 3
    ld a, [hl]
    ld [wMask], a
    xor a
    ld [wS], a
    ld a, [wMask]
    or a
    jr z, .magnitude
    ld a, [wA + 3]
    bit 7, a
    jr z, .magnitude
    ld a, $80
    ld [wS], a
    ld a, [wA]
    cpl
    add 1
    ld [wA], a
    FOR i, 1, 4
    ld a, [wA + i]
    cpl
    adc 0
    ld [wA + i], a
    ENDR
.magnitude
    ld hl, wA
    call sf_is_zero32
    jp z, sf_ret_zero
    FOR i, 4
    ld a, [wA + i]
    ld [wM + i], a
    ENDR
    ld a, 158
    ld [wE], a
    xor a
    ld [wE + 1], a
.normalize
    ld a, [wM + 3]
    bit 7, a
    jr nz, .normalized
    ld hl, wM
    call sf_shl1
    call sf_dec_e
    jr .normalize
.normalized
    ld hl, wM
    call sf_shr1s
    jp sf_roundpack
