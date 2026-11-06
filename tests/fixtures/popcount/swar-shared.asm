audit_popcount::
    call audit_byte
    ld e, a
    ld b, c
    call audit_byte
    add e
    ld b, 0
    ret
audit_popcount_end::
audit_byte:
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
    ret
