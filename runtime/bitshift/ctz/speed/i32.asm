SECTION "32-bit ctz", ROM0

__gbc_ctz32::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
    ld a, b
    or a
    jr z, .next0
    ld b, 0
    ld c, b
    ld d, b
    jp __gbc_ctz_byte
.next0
    ld a, c
    or a
    jr z, .next1
    call __gbc_ctz_byte
    add 8
    jr .done
.next1
    ld a, d
    or a
    jr z, .next2
    call __gbc_ctz_byte
    add 16
    jr .done
.next2
    ld a, e
    or a
    jr z, .next3
    call __gbc_ctz_byte
    add 24
    jr .done
.next3
    ld a, 32
.done
    ld b, 0
    ld c, b
    ld d, b
    ret
