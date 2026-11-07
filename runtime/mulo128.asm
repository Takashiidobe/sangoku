SECTION "u128 overflow multiply", ROM0

; out = BC, lhs = A and stack byte 0, rhs = stack bytes 1..2; out holds 16 product bytes and a bool
__rust___gbc_u128_mulo::
    ld d, a
    add sp, -38
    ld hl, sp + 32
    ld [hl], b
    inc hl
    ld [hl], c
    ld hl, sp + 40
    ld a, [hli]
    ld e, d
    ld d, a
    ld a, [hli]
    ld c, [hl]
    ld hl, sp + 34
    ld [hli], a
    ld [hl], c
    ld hl, sp + 0
    ld b, 16
.copy_lhs:
    ld a, [de]
    ld [hli], a
    inc de
    dec b
    jr nz, .copy_lhs
    xor a
    ld b, 16
.clear:
    ld [hli], a
    dec b
    jr nz, .clear
    ld hl, sp + 36
    ld [hl], 128
    inc hl
    ld [hl], a
.bit:
    ld hl, sp + 16
    ld b, 16
    and a
.shift_result:
    rl [hl]
    inc hl
    dec b
    jr nz, .shift_result
    jr nc, .shift_lhs
    ld hl, sp + 37
    ld [hl], 1
.shift_lhs:
    ld hl, sp + 0
    ld b, 16
    and a
.shift:
    rl [hl]
    inc hl
    dec b
    jr nz, .shift
    jr nc, .next
    ld hl, sp + 34
    ld a, [hli]
    ld d, [hl]
    ld e, a
    ld hl, sp + 16
    ld b, 16
    and a
.add:
    ld a, [de]
    adc [hl]
    ld [hli], a
    inc de
    dec b
    jr nz, .add
    jr nc, .next
    ld hl, sp + 37
    ld [hl], 1
.next:
    ld hl, sp + 36
    dec [hl]
    jr nz, .bit
    ld hl, sp + 32
    ld a, [hli]
    ld d, [hl]
    ld e, a
    ld hl, sp + 16
    ld b, 16
.copy_result:
    ld a, [hli]
    ld [de], a
    inc de
    dec b
    jr nz, .copy_result
    ld hl, sp + 37
    ld a, [hl]
    ld [de], a
    add sp, 38
    ret
