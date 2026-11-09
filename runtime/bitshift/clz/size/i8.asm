SECTION "8-bit clz", ROM0

__gbc_clz8::
    xor a
.loop
    sla b
    jr c, .done
    inc a
    cp 8
    jr nz, .loop
.done
    ret
