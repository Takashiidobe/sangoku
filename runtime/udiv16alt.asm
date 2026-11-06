SECTION "Unsigned 16-bit division", ROM0

; One restoring-division step.
; BC = evolving dividend / quotient
; DE = divisor (D high, E low)
; HL = partial remainder

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


EXPORT __gbc_udiv16
__gbc_udiv16::
    ld d, a
    ld hl, sp + 2
    ld e, [hl]

    call .divide
    ld a, c
    ret


EXPORT __gbc_urem16
__gbc_urem16::
    call .divide
    ld a, l
    ld b, h
    ret


; Shared division core.
; Input:  BC = dividend, DE = divisor
; Output: BC = quotient, HL = remainder
; Clobbers: AF, BC, HL

.divide
    ld a, d
    or e
    jr nz, .nonzero
    rst $10

.nonzero
    ld hl, 0
    ld a, 4

.loop
    push af

    REPT 4
        DIV_STEP
    ENDR

    pop af
    dec a
    jr nz, .loop
    ret
