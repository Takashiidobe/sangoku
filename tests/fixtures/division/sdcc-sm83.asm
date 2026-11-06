INCLUDE "tests/fixtures/division/wrappers.asm"

audit_core::
    ld a, e
    or d
    jr nz, .divide
    rst $10
.divide
    ld l, c
    ld h, b
    ld bc, 0
    or a
    ld a, 16
.loop
    push af
    rl l
    rl h
    rl c
    rl b
    push bc
    ld a, c
    sbc e
    ld c, a
    ld a, b
    sbc d
    ld b, a
    ccf
    jr c, .drop
    pop bc
    pop af
    dec a
    or a
    jr nz, .loop
    jr .done
.drop
    pop af
    pop af
    dec a
    scf
    jr nz, .loop
.done
    ld d, b
    ld e, c
    rl l
    ld c, l
    rl h
    ld b, h
    ld l, e
    ld h, d
    or a
    ret
