SECTION "Audit division", ROM0[$400]

MACRO PREPARE
    ld e, a
    ld hl, sp + 2
    ld d, [hl]
    ld a, b
    ld b, c
    ld c, a
ENDM

audit_q::
    PREPARE
    call audit_core
    ld a, c
    ret

audit_r::
    PREPARE
    call audit_core
    ld a, l
    ld b, h
    ret

audit_both::
    PREPARE
    call audit_core
    ld a, c
    ld c, l
    ld d, h
    ret
