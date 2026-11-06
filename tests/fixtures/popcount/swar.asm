audit_popcount::
    ld a, b
    rrca
    and $55
    ld d, a
    ld a, b
    sub d
    ld b, a
    and $33
    ld d, a
    ld a, b
    rrca
    rrca
    and $33
    add d
    ld b, a
    swap a
    add b
    and $0f
    ld e, a
    ld a, c
    rrca
    and $55
    ld d, a
    ld a, c
    sub d
    ld c, a
    and $33
    ld d, a
    ld a, c
    rrca
    rrca
    and $33
    add d
    ld c, a
    swap a
    add c
    and $0f
    add e
    ld b, 0
    ret
audit_popcount_end::
