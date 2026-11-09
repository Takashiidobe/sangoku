SECTION "16-bit clz", ROM0

__gbc_clz16::
    xor a
.loop
    sla b
    rl c
    jr c, .done
    inc a
    cp 16
    jr nz, .loop
.done
    ld b, 0
    ret
