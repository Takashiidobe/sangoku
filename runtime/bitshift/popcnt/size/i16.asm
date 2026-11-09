SECTION "16-bit population count", ROM0

__gbc_popcnt16::
    ld d, 16
    xor a
.loop:
    srl c
    rr b
    adc 0
    dec d
    jr nz, .loop
    ld b, 0
    ret
