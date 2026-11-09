SECTION "16-bit ctz", ROM0

__gbc_ctz16::
    xor a
.loop
    srl c
    rr b
    jr c, .done
    inc a
    cp 16
    jr nz, .loop
.done
    ld b, 0
    ret
