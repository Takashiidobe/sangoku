SECTION "32-bit population count", ROM0

__gbc_popcnt32::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
    ld h, 0
    FOR I, 4
        IF I == 0
            DEF REG EQUS "b"
        ELIF I == 1
            DEF REG EQUS "c"
        ELIF I == 2
            DEF REG EQUS "d"
        ELSE
            DEF REG EQUS "e"
        ENDC
        ld a, {REG}
        rrca
        and $55
        ld l, a
        ld a, {REG}
        sub l
        ld {REG}, a
        and $33
        ld l, a
        ld a, {REG}
        rrca
        rrca
        and $33
        add l
        ld {REG}, a
        swap a
        add {REG}
        and $0f
        add h
        ld h, a
        PURGE REG
    ENDR
    ld a, h
    ld b, 0
    ld c, b
    ld d, b
    ret
