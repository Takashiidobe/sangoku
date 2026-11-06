audit_popcount::
    xor a
    ld d, 16
.loop
    srl c
    rr b
    adc 0
    dec d
    jr nz, .loop
    ret
audit_popcount_end::
