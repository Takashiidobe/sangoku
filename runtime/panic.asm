SECTION "panic", ROM0

rust_begin_unwind::
    di
.halt:
    halt
    nop
    jr .halt
