INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 __gbc_f32tof16", ROM0


; f32 in, f16 bits out in A, B (C and D are zero), round to nearest even

__gbc_f32tof16::
    LOAD_A
    CLASSIFY wA
    ld d, a
    ld a, [wA + 3]
    and $80
    ld [wS], a
    ld a, d
    cp 2
    jp z, .nan
    or a
    jp nz, .inf
    ld hl, wA
    call sf_is_zero_mag
    jp z, .zero
    UNPACK wA, wXa
    NORMALIZE wXa
    ld a, [wXa + X_E]
    sub 112
    ld [wE], a
    ld a, [wXa + X_E + 1]
    sbc 0
    ld [wE + 1], a
    FOR i, 4
    ld a, [wXa + X_M + i]
    ld [wM + i], a
    ENDR
    ld a, [wE + 1]
    bit 7, a
    jr nz, .low_exponent
    or a
    jp nz, .inf
    ld a, [wE]
    cp 31
    jp nc, .inf
    or a
    jr nz, .extract
.low_exponent
    ld a, [wE]
    cpl
    add 2
    ld l, a
    ld a, [wE + 1]
    cpl
    adc 0
    or a
    jp nz, .zero
    ld a, l
    cp 12
    jp nc, .zero
    ld b, a
.denormalize
    ld hl, wM
    call sf_shr1s
    dec b
    jr nz, .denormalize
    ld a, 1
    ld [wE], a
.extract
    ld a, [wM + 3]
    swap a
    ld b, a
    and $0f
    ld h, a
    ld a, b
    and $f0
    ld b, a
    ld a, [wM + 2]
    swap a
    and $0f
    or b
    ld l, a
    ld a, [wE]
    dec a
    sla a
    sla a
    add h
    ld h, a
    ld a, [wM + 2]
    bit 3, a
    jr z, .pack
    and 7
    ld b, a
    ld a, [wM + 1]
    or b
    ld b, a
    ld a, [wM]
    or b
    jr nz, .round_up
    ld a, l
    bit 0, a
    jr z, .pack
.round_up
    inc l
    jr nz, .pack
    inc h
.pack
    ld a, h
    cp $7c
    jr c, .out
    ld h, $7c
    ld l, 0
.out
    ld a, [wS]
    or h
    ld b, a
    ld a, l
    ld c, 0
    ld d, 0
    ret
.nan
    xor a
    ld b, $7e
    ld c, a
    ld d, a
    ret
.inf
    ld a, [wS]
    or $7c
    ld b, a
    xor a
    ld c, a
    ld d, a
    ret
.zero
    ld a, [wS]
    ld b, a
    xor a
    ld c, a
    ld d, a
    ret
