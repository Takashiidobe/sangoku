SECTION "16-bit population count", ROM0

__gbc_popcnt16::
    ld a, b
    call __gbc_popcnt_byte
    ld b, a
    ld a, c
    call __gbc_popcnt_byte
    add b
    ld b, 0
    ret
