SECTION "8-bit ctz", ROM0

__gbc_ctz8::
    xor a
.loop
    srl b
    jr c, .done
    inc a
    cp 8
    jr nz, .loop
.done
    ret
