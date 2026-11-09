SECTION "8-bit population count", ROM0

__gbc_popcnt8::
    ld c, 8
    xor a
.loop:
    srl b
    adc 0
    dec c
    jr nz, .loop
    ret
