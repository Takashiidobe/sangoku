SECTION "8-bit bitrev", ROM0

__gbc_bitrev8::
    ld a, b
    jp __gbc_bitrev_byte
