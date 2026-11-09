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

SECTION "i128 overflow multiply", ROM0

__rust___gbc_i128_mulo::
    ld d, a
    add sp, -35
    ld hl, sp + 32
    ld [hl], b
    inc hl
    ld [hl], c
    ld hl, sp + 37
    ld a, [hli]
    ld e, d
    ld d, a
    inc hl
    ld c, [hl]
    ld hl, sp + 0
    ld b, 16
.copy_lhs:
    ld a, [de]
    ld [hli], a
    inc de
    dec b
    jr nz, .copy_lhs
    ld d, c
    ld hl, sp + 38
    ld e, [hl]
    ld hl, sp + 16
    ld b, 16
.copy_rhs:
    ld a, [de]
    ld [hli], a
    inc de
    dec b
    jr nz, .copy_rhs
    ld hl, sp + 34
    ld [hl], 0
    ld hl, sp + 15
    bit 7, [hl]
    jr z, .rhs_sign
    ld hl, sp + 34
    ld [hl], 1
    ld hl, sp + 0
    call .negate
.rhs_sign:
    ld hl, sp + 31
    bit 7, [hl]
    jr z, .multiply
    ld hl, sp + 34
    ld a, [hl]
    xor 1
    ld [hl], a
    ld hl, sp + 16
    call .negate
.multiply:
    ld hl, sp + 16
    ld d, h
    ld e, l
    push de
    ld hl, sp + 2
    ld a, h
    ld d, l
    push af
    inc sp
    ld hl, sp + 35
    ld b, [hl]
    inc hl
    ld c, [hl]
    ld a, d
    call __rust___gbc_u128_mulo
    add sp, 3
    ld hl, sp + 32
    ld a, [hli]
    ld h, [hl]
    ld l, a
    ld de, 15
    add hl, de
    bit 7, [hl]
    jr z, .apply_sign
    ld a, [hl]
    cp $80
    jr nz, .overflow
    ld b, 15
.check_min:
    dec hl
    ld a, [hl]
    or a
    jr nz, .overflow
    dec b
    jr nz, .check_min
    ld hl, sp + 34
    bit 0, [hl]
    jr nz, .apply_sign
.overflow:
    ld hl, sp + 32
    ld a, [hli]
    ld h, [hl]
    ld l, a
    ld de, 16
    add hl, de
    ld [hl], 1
.apply_sign:
    ld hl, sp + 34
    bit 0, [hl]
    jr z, .done
    ld hl, sp + 32
    ld a, [hli]
    ld h, [hl]
    ld l, a
    call .negate
.done:
    add sp, 35
    ret
.negate:
    ld b, 16
    and a
.negate_byte:
    ld a, 0
    sbc [hl]
    ld [hli], a
    dec b
    jr nz, .negate_byte
    ret
