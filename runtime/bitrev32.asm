SECTION "32-bit bit reversal", ROM0

MACRO REVERSE_BYTE
    ld a, \1
    ld \1, 0
    ld l, 8
.bit\@
    rra
    rl \1
    dec l
    jr nz, .bit\@
ENDM

EXPORT __gbc_bitrev32
__gbc_bitrev32::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]
    REVERSE_BYTE b
    REVERSE_BYTE c
    REVERSE_BYTE d
    REVERSE_BYTE e
    ld a, e
    ld e, b
    ld b, d
    ld d, e
    ret
