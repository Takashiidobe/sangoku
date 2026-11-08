#include <stdint.h>
#define SERIAL_DATA (*(volatile uint8_t *)0xFF01)
#define SERIAL_CONTROL (*(volatile uint8_t *)0xFF02)
static void write_byte(uint8_t b) {
    SERIAL_DATA = b;
    SERIAL_CONTROL = 0x81;
    while (SERIAL_CONTROL & 0x80) {}
}
static void write_str(const char *s) {
    while (*s) write_byte((uint8_t)*s++);
}
