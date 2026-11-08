#include <stdint.h>
#include <stdlib.h>
static void trap(void) { for (;;) {} }
typedef struct { uint8_t *ptr; uint8_t len, cap; } vec;
static void push(vec *v, uint8_t x) {
    if (v->len == v->cap) {
        uint8_t cap = v->cap ? v->cap * 2 : 4;
        uint8_t *p = realloc(v->ptr, cap);
        if (!p) trap();
        v->ptr = p; v->cap = cap;
    }
    v->ptr[v->len++] = x;
}
uint8_t gbc_main(void) {
    vec global = {0}, scoped = {0};
    push(&global, 10); push(&global, 20); push(&global, 12);
    push(&scoped, 1); push(&scoped, 2);
    uint8_t *boxed = malloc(1);
    if (!boxed) trap();
    *boxed = 7;
    uint8_t sum = 0;
    for (uint8_t i = 0; i < global.len; i++) sum += global.ptr[i];
    for (uint8_t i = 0; i < scoped.len; i++) sum += scoped.ptr[i];
    sum += *boxed;
    free(boxed); free(scoped.ptr); free(global.ptr);
    return sum;
}
