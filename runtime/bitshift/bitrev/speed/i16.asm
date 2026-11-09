SECTION "16-bit bitrev", ROM0

__gbc_bitrev16::
    ld a, b
    call __gbc_bitrev_byte
    ld b, a
    ld a, c
    jp __gbc_bitrev_byte
