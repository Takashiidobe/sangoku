INCLUDE "runtime/wide32_div.inc"

SECTION "32-bit division core", ROM0

__gbc_div32_core::
    ld hl, sp + 2 + F_COUNT
    ld [hl], 32
.loop
    ld hl, sp + 2 + F_N
    sla [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    inc hl
    rl [hl]
    sbc a, a
    ld b, a
    ld hl, sp + 2 + F_D
    ld d, h
    ld e, l
    ld hl, sp + 2 + F_R
    ld a, [de]
    ld c, a
    ld a, [hl]
    sub c
    ld [hl+], a
    inc de
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [hl+], a
    inc de
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [hl+], a
    inc de
    ld a, [de]
    ld c, a
    ld a, [hl]
    sbc c
    ld [hl], a
    sbc a, a
    ld c, a
    ld a, b
    cpl
    and c
    jr nz, .restore
    ld hl, sp + 2 + F_N
    set 0, [hl]
    jr .next
.restore
    ld hl, sp + 2 + F_D
    ld d, h
    ld e, l
    ld hl, sp + 2 + F_R
    ld a, [de]
    add [hl]
    ld [hl+], a
    inc de
    ld a, [de]
    adc [hl]
    ld [hl+], a
    inc de
    ld a, [de]
    adc [hl]
    ld [hl+], a
    inc de
    ld a, [de]
    adc [hl]
    ld [hl], a
.next
    ld hl, sp + 2 + F_COUNT
    dec [hl]
    jr nz, .loop
    ret
