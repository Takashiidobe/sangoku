SECTION "128-bit variable shifts (speed)", ROM0

; Same contract as the size variant. Moves whole bytes first, then at most
; seven single-bit steps.
__gbc_shift128::
    ld d, c
    ld e, b
    ld hl, 16
    add hl, de
    ld a, [hli]
    ld b, a
    ld a, [hl]
    and 127
    ret z
    srl a
    srl a
    srl a
    ld c, a
    jr z, .bits
.byte_loop
    call __gbc_wshift_byte128
    dec c
    jr nz, .byte_loop
.bits
    ld hl, 17
    add hl, de
    ld a, [hl]
    and 7
    ret z
    ld c, a
.bit_loop
    call __gbc_wshift_bit128
    dec c
    jr nz, .bit_loop
    ret
