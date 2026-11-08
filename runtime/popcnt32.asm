SECTION "32-bit population count", ROM0

EXPORT __gbc_popcnt32
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
.byte{u:I}
        ld a, {REG}
        or a
        jr z, .byte_done{u:I}
        srl {REG}
        jr nc, .byte{u:I}
        inc h
        jr .byte{u:I}
.byte_done{u:I}
        PURGE REG
    ENDR
    ld a, h
    ld b, 0
    ld c, 0
    ld d, 0
    ret
