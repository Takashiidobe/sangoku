audit_popcount::
    ld h, HIGH(audit_table)
    ld a, b
    and $0f
    add LOW(audit_table)
    ld l, a
    ld e, [hl]
    ld a, b
    swap a
    and $0f
    add LOW(audit_table)
    ld l, a
    ld a, [hl]
    add e
    ld e, a
    ld a, c
    and $0f
    add LOW(audit_table)
    ld l, a
    ld a, [hl]
    add e
    ld e, a
    ld a, c
    swap a
    and $0f
    add LOW(audit_table)
    ld l, a
    ld a, [hl]
    add e
    ld b, 0
    ret
audit_popcount_end::
SECTION "Audit table", ROM0[$810], ALIGN[4]
audit_table::
    db 0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4
