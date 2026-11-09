SECTION "32-bit trailing zeros", ROM0

EXPORT __gbc_ctz32
__gbc_ctz32::
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
        or a
        jr z, .empty{u:I}
.shift{u:I}
        bit 0, {REG}
        jr nz, .found
        srl {REG}
        inc h
        jr .shift{u:I}
.empty{u:I}
        ld a, h
        add 8
        ld h, a
        PURGE REG
    ENDR
.found
    ld a, h
    ld b, 0
    ld c, 0
    ld d, 0
    ret
