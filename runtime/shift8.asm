SECTION "8-bit shifts", ROM0

; 8-bit shift or rotate by a variable count. Arguments: B value, C count,
; A op (0 shl, 1 ushr, 2 sshr, 3 rotl, 4 rotr). Result in A; the count is
; taken modulo 8 as CLIF specifies. Every register is clobbered.
EXPORT __gbc_shift8
__gbc_shift8::
    ld d, a
    ld a, c
    and 7
    ld c, a
    ld e, b
    ld b, d
.loop
    ld a, c
    or a
    jr z, .done
    dec c
    ld a, b
    or a
    jr z, .shl
    dec a
    jr z, .ushr
    dec a
    jr z, .sshr
    dec a
    jr z, .rotl
    rrc e
    jr .loop
.shl
    sla e
    jr .loop
.ushr
    srl e
    jr .loop
.sshr
    sra e
    jr .loop
.rotl
    rlc e
    jr .loop
.done
    ld a, e
    ret
