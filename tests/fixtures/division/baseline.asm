SECTION "Unsigned 16-bit division", ROM0

EXPORT __gbc_udiv16
__gbc_udiv16::
    ld d, a
    ld hl, sp + 2
    ld a, [hl]
    ld e, a
    ld hl, 0
    ld a, d
    or e
    jr nz, .divide
    rst $10

.divide
    ld a, 16
    push af
.loop
    sla c
    rl b
    rl l
    rl h
    jr c, .subtract
    ld a, h
    cp e
    jr c, .next
    jr nz, .subtract
    ld a, l
    cp d
    jr c, .next
.subtract
    ld a, l
    sub d
    ld l, a
    ld a, h
    sbc e
    ld h, a
    inc c
.next
    pop af
    dec a
    jr z, .done
    push af
    jr .loop
.done
    ld a, c
    ret

EXPORT __gbc_urem16
__gbc_urem16::
    ld a, d
    or e
    jr nz, .divide
    rst $10

.divide
    ld a, 16
    push af
.loop
    sla c
    rl b
    rl l
    rl h
    jr c, .subtract
    ld a, h
    cp e
    jr c, .next
    jr nz, .subtract
    ld a, l
    cp d
    jr c, .next
.subtract
    ld a, l
    sub d
    ld l, a
    ld a, h
    sbc e
    ld h, a
    inc c
.next
    pop af
    dec a
    jr z, .done
    push af
    jr .loop
.done
    ld a, l
    ld b, h
    ret
