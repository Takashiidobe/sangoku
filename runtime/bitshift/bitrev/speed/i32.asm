SECTION "32-bit bitrev", ROM0

__gbc_bitrev32::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
    ld a, b
    call __gbc_bitrev_byte
    ld b, a
    ld a, c
    call __gbc_bitrev_byte
    ld c, a
    ld a, d
    call __gbc_bitrev_byte
    ld d, a
    ld a, e
    call __gbc_bitrev_byte
    ld e, b
    ld b, d
    ld d, e
    ret
