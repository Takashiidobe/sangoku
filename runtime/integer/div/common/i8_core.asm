SECTION "8-bit unsigned magnitude division", ROM0
__gbc_div8_core::
    ld a, c
    or a
    jr nz, .nonzero
    rst $10
.nonzero
    xor a
    ld e, 8
.loop
    sla b
    rla
    cp c
    jr c, .next
    sub c
    inc b
.next
    dec e
    jr nz, .loop
    ret
