SECTION "16-bit shifts", ROM0

; 16-bit shift or rotate by a variable count. Arguments: B,C value (low,
; high), A = op << 4 | count (op 0 shl, 1 ushr, 2 sshr, 3 rotl, 4 rotr,
; count 0..15). Result in A (low) and B (high). Every register is clobbered.
EXPORT __gbc_shift16
__gbc_shift16::
    ld d, a
    and $0f
    ld e, a
    ld a, d
    swap a
    and $0f
    ld d, a
.loop
    ld a, e
    or a
    jr z, .done
    dec e
    ld a, d
    or a
    jr z, .shl
    dec a
    jr z, .ushr
    dec a
    jr z, .sshr
    dec a
    jr z, .rotl
    srl c
    rr b
    jr nc, .loop
    set 7, c
    jr .loop
.shl
    sla b
    rl c
    jr .loop
.ushr
    srl c
    rr b
    jr .loop
.sshr
    sra c
    rr b
    jr .loop
.rotl
    sla b
    rl c
    jr nc, .loop
    set 0, b
    jr .loop
.done
    ld a, b
    ld b, c
    ret
