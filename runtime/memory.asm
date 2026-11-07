SECTION "memory", ROM0

__gbc_memset::
    push bc
    ld d, a
    ld a, b
    ld b, c
    ld c, a
    ld hl, sp + 7
    ld a, [hli]
    ld h, [hl]
    ld l, a
.loop:
    ld a, h
    or l
    jr z, .done
    ld a, d
    ld [bc], a
    inc bc
    dec hl
    jr .loop
.done:
    pop bc
    ld a, b
    ld b, c
    ret


; memcpy and memmove(dst, src, len) -> dst. dst = B (low), C (high); src = A
; (low) and first stack byte (high); len = next two stack bytes. Arguments are
; left for the caller to pop. memcpy is the same routine: it copies forward
; when dst < src and backward otherwise, which is correct for overlap too.
; The forward loop is SDCC 4.6.4 device/lib/sm83/memcpy.s (see NOTICE): four
; bytes per iteration, entered by Duff's device.
memcpy::
memmove::
    push bc
    ld d, c
    ld e, b
    ld c, a
    ld hl, sp + 4
    ld b, [hl]
    ld hl, sp + 5
    ld a, [hli]
    ld h, [hl]
    ld l, a
    push bc
    push hl
    pop bc
    pop hl
    ld a, e
    sub l
    ld a, d
    sbc h
    jr c, .forward
    add hl, bc
    ld a, e
    add c
    ld e, a
    ld a, d
    adc b
    ld d, a
.backward:
    ld a, b
    or c
    jr z, .done
    dec hl
    dec de
    ld a, [hl]
    ld [de], a
    dec bc
    jr .backward
.forward:
    srl b
    rr c
    jr nc, .skip_one
    ld a, [hli]
    ld [de], a
    inc de
.skip_one:
    srl b
    rr c
    inc b
    inc c
    jr nc, .test
    jr .copy_two
.copy_four:
    ld a, [hli]
    ld [de], a
    inc de
    ld a, [hli]
    ld [de], a
    inc de
.copy_two:
    ld a, [hli]
    ld [de], a
    inc de
    ld a, [hli]
    ld [de], a
    inc de
.test:
    dec c
    jr nz, .copy_four
    dec b
    jr nz, .copy_four
.done:
    pop bc
    ld a, b
    ld b, c
    ret
