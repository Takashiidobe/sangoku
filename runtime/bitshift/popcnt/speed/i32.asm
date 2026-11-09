SECTION "32-bit population count", ROM0

__gbc_popcnt32::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
    ld a, b
    call __gbc_popcnt_byte
    ld b, a
    ld a, c
    call __gbc_popcnt_byte
    add b
    ld b, a
    ld a, d
    call __gbc_popcnt_byte
    add b
    ld b, a
    ld a, e
    call __gbc_popcnt_byte
    add b
    ld b, 0
    ld c, b
    ld d, b
    ret
