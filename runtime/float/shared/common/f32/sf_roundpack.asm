INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_roundpack", ROM0

; Round wM (leading bit 30, or lower when subnormal) with sign wS and
; exponent wE (signed 16-bit, may be <= 0) and pack the result into wR.

sf_roundpack::
    ld a, [wE + 1]
    bit 7, a
    jr nz, .denormal
    ld b, a
    ld a, [wE]
    or b
    jr nz, .round
.denormal
    ld a, [wE]
    cpl
    add 2
    ld l, a
    ld a, [wE + 1]
    cpl
    adc 0
    ld h, a
    ld a, 1
    ld [wE], a
    xor a
    ld [wE + 1], a
    ld a, h
    or a
    jr nz, .jam
    ld a, l
    cp 32
    jr nc, .jam
    ld b, l
    ld hl, wM
.shift
    call sf_shr1s
    dec b
    jr nz, .shift
    jr .round
.jam
    ld hl, wM
    call sf_is_zero32
    ld a, 0
    jr z, .store_jam
    ld a, 1
.store_jam
    ld hl, wM
    ld [hl+], a
    xor a
    ld [hl+], a
    ld [hl+], a
    ld [hl], a
.round
    ld a, [wM]
    ld b, a
    and $7f
    cp $40
    ld c, 0
    jr c, .no_increment
    jr nz, .increment
    bit 7, b
    jr z, .no_increment
.increment
    ld c, 1
.no_increment
    ld hl, wM
    call sf_shl1
    ld a, c
    or a
    jr z, .pack
    ld hl, wM + 1
    inc [hl]
    jr nz, .pack
    inc hl
    inc [hl]
    jr nz, .pack
    inc hl
    inc [hl]
    jr nz, .pack
    ld a, $80
    ld [wM + 3], a
    call sf_inc_e
.pack
    ld a, [wE + 1]
    or a
    jp nz, sf_ret_inf
    ld a, [wE]
    cp 255
    jp nc, sf_ret_inf
    ld a, [wM + 3]
    bit 7, a
    jr nz, .normal
    ld d, 0
    jr .have_field
.normal
    ld a, [wE]
    ld d, a
.have_field
    ld a, [wM + 3]
    and $7f
    ld c, a
    ld a, d
    srl a
    ld b, a
    ld a, c
    jr nc, .store_high
    or $80
.store_high
    ld [wR + 2], a
    ld a, [wS]
    or b
    ld [wR + 3], a
    ld a, [wM + 1]
    ld [wR], a
    ld a, [wM + 2]
    ld [wR + 1], a
    jp sf_return
