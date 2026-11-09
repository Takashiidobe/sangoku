SECTION "32-bit clz", ROM0

__gbc_clz32::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
    xor a
.loop
    sla b
    rl c
    rl d
    rl e
    jr c, .done
    inc a
    cp 32
    jr nz, .loop
.done
    ld b, 0
    ld c, b
    ld d, b
    ret
