#include "serial.h"
static void write_u8(uint8_t n) {
    if (n >= 100) return;
    if (n >= 10) {
        uint8_t tens = 0;
        while (n >= 10) { n -= 10; tens++; }
        write_byte('0' + tens);
    }
    write_byte('0' + n);
}
uint8_t gbc_main(void) {
    uint8_t composite[50] = {0};
    for (uint16_t n = 2; n < 50; n++) {
        if (composite[n]) continue;
        write_u8((uint8_t)n);
        write_str(" ");
        for (uint16_t m = n * n; m < 50; m += n) composite[m] = 1;
    }
    uint8_t count = 0;
    for (uint8_t i = 2; i < 50; i++) if (!composite[i]) count++;
    write_str("\n");
    write_u8(count);
    write_str(" primes below 50\n");
    return count;
}
