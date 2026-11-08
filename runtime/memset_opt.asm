SECTION "memset", ROM0

__gbc_memset::
    push bc
    ld d, a
    ld hl, sp + 7
    ld a, [hli]
    ld h, [hl]
    ld l, a
    push hl
    ld h, c
    ld l, b
    pop bc
    ld a, d
    srl b
    rr c
    jr nc, .skip_one
    ld [hli], a
.skip_one:
    srl b
    rr c
    inc b
    inc c
    jr nc, .test
    jr .fill_two
.fill_four:
    ld [hli], a
    ld [hli], a
.fill_two:
    ld [hli], a
    ld [hli], a
.test:
    dec c
    jr nz, .fill_four
    dec b
    jr nz, .fill_four
.done:
    pop bc
    ld a, b
    ld b, c
    ret
