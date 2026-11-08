#include "serial.h"
uint8_t gbc_main(void) {
    write_str("Hello, Game Boy!\n");
    write_str("6 * 7 = 42\n");
    return 0;
}
