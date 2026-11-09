SECTION "8-bit population count", ROM0

__gbc_popcnt8::
    ld a, b
    jp __gbc_popcnt_byte
