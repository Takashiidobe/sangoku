#include <stdint.h>

uint16_t mix16(uint16_t a, uint16_t b, uint16_t c) {
    return ((a ^ b) + c) ^ (uint16_t)((c << 3) | (c >> 13));
}

uint8_t pressure(uint8_t a, uint8_t b, uint8_t c, uint8_t d, uint8_t e, uint8_t f) {
    uint8_t t1 = a + b, t2 = c ^ d, t3 = e & f;
    uint8_t t4 = t1 + t2, t5 = t2 | t3, t6 = t1 ^ t3;
    return (uint8_t)(t4 + t5) ^ (uint8_t)(t6 + (a ^ f));
}

uint16_t pressure16(uint16_t a, uint16_t b, uint16_t c) {
    uint16_t x = a + b, y = b ^ c, z = c - a;
    return (uint16_t)(((x & y) + (y | z)) ^ (x + z));
}

uint16_t sum_u8(const uint8_t *p, uint8_t n) {
    uint16_t s = 0;
    while (n) { s += *p; p++; n--; }
    return s;
}

uint16_t sum_u16(const uint16_t *p, uint8_t n) {
    uint16_t s = 0;
    while (n) { s += *p; p++; n--; }
    return s;
}

void fill(uint8_t *p, uint8_t v, uint8_t n) {
    while (n) { *p = v; p++; n--; }
}

void copy_u8(uint8_t *d, const uint8_t *s, uint8_t n) {
    while (n) { *d = *s; d++; s++; n--; }
}

uint8_t max_u8(const uint8_t *p, uint8_t n) {
    uint8_t m = 0;
    while (n) { uint8_t x = *p; if (x > m) m = x; p++; n--; }
    return m;
}

uint8_t count_eq(const uint8_t *p, uint8_t v, uint8_t n) {
    uint8_t c = 0;
    while (n) { if (*p == v) c++; p++; n--; }
    return c;
}

uint16_t strlen16(const uint8_t *p) {
    uint16_t len = 0;
    while (*p) { len++; p++; }
    return len;
}

uint16_t fib16(uint8_t n) {
    uint16_t a = 0, b = 1;
    while (n) { uint16_t t = a + b; a = b; b = t; n--; }
    return a;
}

uint8_t sat_chain(uint8_t a, uint8_t b, uint8_t c) {
    unsigned s = (unsigned)a + b;
    uint8_t x = s > 255 ? 255 : (uint8_t)s;
    uint8_t y = x > c ? (uint8_t)(x - c) : (uint8_t)(c - x);
    unsigned r = (unsigned)y + a;
    return r > 255 ? 255 : (uint8_t)r;
}
