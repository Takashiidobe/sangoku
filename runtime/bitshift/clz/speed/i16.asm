SECTION "16-bit clz", ROM0

__gbc_clz16::
    ld a, c
    or a
    jr z, .next0
    ld b, 0
    jp __gbc_clz_byte
.next0
    ld a, b
    or a
    jr z, .next1
    call __gbc_clz_byte
    add 8
    jr .done
.next1
    ld a, 16
.done
    ld b, 0
    ret
