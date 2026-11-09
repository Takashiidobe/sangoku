INCLUDE "runtime/float/shared/common/f32/macros.inc"

SECTION "f32 sf_shift_jam", ROM0


; hl -> 64-bit value, bc = shift count (capped at 64): shift right with jamming

sf_shift_jam::
    ld a, b
    or a
    jr nz, .cap
    ld a, c
    cp 64
    jr c, .go
.cap
    ld c, 64
.go
    ld a, c
    or a
    ret z
.loop
    push hl
    call sf_shr1j64
    pop hl
    dec c
    jr nz, .loop
    ret
