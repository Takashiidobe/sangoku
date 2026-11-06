audit_popcount::
    xor a
    REPT 8
        srl b
        adc 0
    ENDR
    REPT 8
        srl c
        adc 0
    ENDR
    ret
audit_popcount_end::
