    .module crt0
    .globl _gbc_main
    .area _HEADER (ABS)
    .org 0x100
    nop
    jp init
    .org 0x150
init:
    ld sp, #0xE000
    call _gbc_main
halt_loop:
    jr halt_loop
    .area _HOME
    .area _CODE
    .area _INITIALIZER
    .area _GSINIT
    .area _GSFINAL
    .area _DATA
    .area _INITIALIZED
    .area _BSS
    .area _HEAP
