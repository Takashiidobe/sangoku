; 12-bytes/284-cycles
popcount8:
    ld b, a
    ld c, 8
    xor a

.loop:
    srl b
    adc a, 0
    dec c
    jr nz, .loop
    ret
