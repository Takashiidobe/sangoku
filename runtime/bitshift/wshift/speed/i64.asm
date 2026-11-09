SECTION "64-bit variable shifts (speed)", ROM0

; Same contract as the size variant. Moves whole bytes first, then at most
; seven single-bit steps.
__gbc_shift64::
    ld d, c
    ld e, b
    ld hl, 8
    add hl, de
    ld a, [hli]
    ld b, a
    ld a, [hl]
    and 63
    ret z
    srl a
    srl a
    srl a
    ld c, a
    jr z, .bits
.byte_loop
    call __gbc_wshift_byte64
    dec c
    jr nz, .byte_loop
.bits
    ld hl, 9
    add hl, de
    ld a, [hl]
    and 7
    ret z
    ld c, a
.bit_loop
    call __gbc_wshift_bit64
    dec c
    jr nz, .bit_loop
    ret
