SECTION "128-bit variable shifts (size)", ROM0

; Shift or rotate the 16-byte little-endian value at BC in place. Byte 16 holds
; the op (0 shl, 1 ushr, 2 sshr, 3 rotl, 4 rotr) and byte 17 the count,
; taken modulo 128. Every register is clobbered.
__gbc_shift128::
    ld d, c
    ld e, b
    ld hl, 16
    add hl, de
    ld a, [hli]
    ld b, a
    ld a, [hl]
    and 127
    ld c, a
    ret z
.loop
    call __gbc_wshift_bit128
    dec c
    jr nz, .loop
    ret
