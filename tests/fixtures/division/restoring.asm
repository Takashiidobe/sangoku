INCLUDE "tests/fixtures/division/wrappers.asm"

MACRO DIV_STEP
    sla c
    rl b
    rl l
    rl h
    jr c, .subtract\@
    ld a, h
    cp d
    jr c, .next\@
    jr nz, .subtract\@
    ld a, l
    cp e
    jr c, .next\@
.subtract\@
    ld a, l
    sub e
    ld l, a
    ld a, h
    sbc d
    ld h, a
    inc c
.next\@
ENDM

audit_core::
    ld a, d
    or e
    jr nz, .nonzero
    rst $10
.nonzero
IF DEF(AUDIT_EARLY)
INCLUDE "tests/fixtures/division/early.asm"
ENDC
    ld hl, 0
IF AUDIT_UNROLL == 16
    REPT 16
        DIV_STEP
    ENDR
ELSE
    ld a, 16 / AUDIT_UNROLL
.loop
    push af
    REPT AUDIT_UNROLL
        DIV_STEP
    ENDR
    pop af
    dec a
    jr nz, .loop
ENDC
    ret
