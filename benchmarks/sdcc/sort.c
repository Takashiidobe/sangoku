#include "serial.h"
static const uint16_t places[] = {10000, 1000, 100, 10, 1};
static void write_u16(uint16_t value) {
    uint8_t started = 0;
    for (uint8_t i = 0; i < 5; i++) {
        uint16_t place = places[i];
        uint8_t digit = 0;
        while (value >= place) { value -= place; digit++; }
        if (digit || started || place == 1) { write_byte('0' + digit); started = 1; }
    }
}
static void write_values(const uint16_t *v, uint8_t len) {
    write_byte('[');
    for (uint8_t i = 0; i < len; i++) {
        if (i) write_str(", ");
        write_u16(v[i]);
    }
    write_str("]\n");
}
static void sort(uint16_t *v, uint8_t len) {
    uint8_t changed = 1;
    if (!len) return;
    while (changed) {
        changed = 0;
        for (uint8_t i = 1; i < len; i++) {
            if (v[i - 1] > v[i]) { uint16_t t = v[i - 1]; v[i - 1] = v[i]; v[i] = t; changed = 1; }
        }
    }
}
uint8_t gbc_main(void) {
    uint16_t values[8] = {42, 7, 1999, 3, 256, 88, 1024, 5};
    write_str("before: "); write_values(values, 8);
    sort(values, 8);
    write_str("after:  "); write_values(values, 8);
    uint16_t total = 0, max = 0;
    for (uint8_t i = 0; i < 8; i++) { total += values[i]; if (values[i] > max) max = values[i]; }
    write_str("sum = "); write_u16(total);
    write_str(", max = Some("); write_u16(max); write_byte(')');
    write_byte('\n');
    return 0;
}
