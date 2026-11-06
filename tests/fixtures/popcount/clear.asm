audit_popcount::
    ld d, 0
.loop
    ld a, b
    or c
    jr z, .done
    ld a, b
    sub 1
    ld e, a
    ld a, c
    sbc 0
    and c
    ld c, a
    ld a, e
    and b
    ld b, a
    inc d
    jr .loop
.done
    ld a, d
    ret
audit_popcount_end::
