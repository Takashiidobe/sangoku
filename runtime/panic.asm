SECTION "panic", ROM0

; core references the v0-mangled std-internal panic handler symbol
__rust__RNvCsbOwsw4zHH1Z_7___rustc17rust_begin_unwind::
    di
.halt:
    halt
    nop
    jr .halt
