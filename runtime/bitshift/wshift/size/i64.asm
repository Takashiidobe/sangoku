SECTION "64-bit variable shifts (size)", ROM0

; Shift or rotate the 8-byte little-endian value at BC in place. Byte 8 holds
; the op (0 shl, 1 ushr, 2 sshr, 3 rotl, 4 rotr) and byte 9 the count,
; taken modulo 64. Every register is clobbered.
__gbc_shift64::
    ld d, c
    ld e, b
    ld hl, 8
    add hl, de
    ld a, [hli]
    ld b, a
    ld a, [hl]
    and 63
    ld c, a
    ret z
.loop
    call __gbc_wshift_bit64
    dec c
    jr nz, .loop
    ret
