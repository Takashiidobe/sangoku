audit_popcount::
    xor a
    ld d, 8
.low
    srl b
    adc 0
    dec d
    jr nz, .low
    ld d, 8
.high
    srl c
    adc 0
    dec d
    jr nz, .high
    ret
audit_popcount_end::
