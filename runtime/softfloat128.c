#include <stdint.h>

typedef unsigned char u8;
typedef unsigned short u16;
typedef int32_t exponent_t;
typedef signed short i16;

typedef struct {
    u8 sign;
    u8 kind;
    exponent_t exponent;
    u8 mantissa[16];
} Sf128;

static void sf128_clear(u8 *a, u8 n) {
    u8 i;
    for (i = 0; i < n; ++i)
        a[i] = 0;
}

static void sf128_copy(u8 *a, const u8 *b, u8 n) {
    u8 i;
    for (i = 0; i < n; ++i)
        a[i] = b[i];
}

static u8 sf128_nonzero(const u8 *a, u8 n) {
    u8 i, bits = 0;
    for (i = 0; i < n; ++i)
        bits |= a[i];
    return bits != 0;
}

static u8 sf128_left(u8 *a, u8 n) {
    u8 i, carry = 0, next;
    for (i = 0; i < n; ++i) {
        next = a[i] >> 7;
        a[i] = (a[i] << 1) | carry;
        carry = next;
    }
    return carry;
}

static u8 sf128_right(u8 *a, u8 n) {
    u8 i = n, carry = 0, next;
    while (i) {
        --i;
        next = a[i] & 1;
        a[i] = (a[i] >> 1) | (carry << 7);
        carry = next;
    }
    return carry;
}

static void sf128_shl(u8 *a, u8 count, u8 n) {
    while (count--)
        sf128_left(a, n);
}

static void sf128_shr(u8 *a, exponent_t count, u8 jam, u8 n) {
    u8 sticky = 0;
    u16 remaining;
    if (count > (u16)n * 8)
        count = (u16)n * 8;
    remaining = (u16)count;
    while (remaining--)
        sticky |= sf128_right(a, n);
    if (jam)
        a[0] |= sticky;
}

static u8 sf128_add(u8 *a, const u8 *b, u8 n) {
    u8 i, carry = 0;
    u16 sum;
    for (i = 0; i < n; ++i) {
        sum = (u16)a[i] + b[i] + carry;
        a[i] = (u8)sum;
        carry = (u8)(sum >> 8);
    }
    return carry;
}

static void sf128_increment(u8 *a, u8 n) {
    u8 i;
    for (i = 0; i < n; ++i)
        if (++a[i])
            break;
}

static void sf128_sub(u8 *a, const u8 *b, u8 n) {
    u8 i, borrow = 0, next;
    u16 sub;
    for (i = 0; i < n; ++i) {
        sub = (u16)b[i] + borrow;
        next = (u16)a[i] < sub;
        a[i] = (u8)((u16)a[i] - sub);
        borrow = next;
    }
}

static signed char sf128_cmp(const u8 *a, const u8 *b, u8 n) {
    u8 i = n;
    while (i) {
        --i;
        if (a[i] != b[i])
            return a[i] < b[i] ? -1 : 1;
    }
    return 0;
}

static void sf128_copy_float(Sf128 *a, const Sf128 *b) {
    a->sign = b->sign;
    a->kind = b->kind;
    a->exponent = b->exponent;
    sf128_copy(a->mantissa, b->mantissa, 16);
}

static void sf128_unpack(Sf128 *a, const u8 *raw) {
    u8 i;
    u16 exponent = ((u16)(raw[15] & 127) << 8) | raw[14];
    a->sign = raw[15] >> 7;
    a->exponent = 0;
    sf128_clear(a->mantissa, 16);
    for (i = 0; i < 14; ++i)
        a->mantissa[i] = raw[i];
    if (exponent == 32767) {
        a->kind = sf128_nonzero(a->mantissa, 16) ? 3 : 2;
        return;
    }
    if (!exponent && !sf128_nonzero(a->mantissa, 16)) {
        a->kind = 0;
        return;
    }
    a->kind = 1;
    a->exponent = exponent ? (exponent_t)exponent - 16383 : -16382;
    if (exponent)
        a->mantissa[14] |= 1;
    while (!(a->mantissa[14] & 1)) {
        sf128_left(a->mantissa, 16);
        --a->exponent;
    }
    sf128_shl(a->mantissa, 8, 16);
}

static void sf128_normalize(Sf128 *a) {
    if (!sf128_nonzero(a->mantissa, 16)) {
        a->kind = 0;
        return;
    }
    while (a->mantissa[15] > 1) {
        sf128_shr(a->mantissa, 1, 1, 16);
        ++a->exponent;
    }
    while (!a->mantissa[15]) {
        sf128_left(a->mantissa, 16);
        --a->exponent;
    }
}

static void sf128_special(u8 *raw, u8 sign, u8 kind) {
    u8 i;
    for (i = 0; i < 16; ++i)
        raw[i] = 0;
    raw[15] = sign << 7;
    if (kind >= 2) {
        raw[15] |= 127;
        raw[14] = 255;
        if (kind == 3)
            raw[13] = 128;
    }
}

static void sf128_pack(u8 *raw, Sf128 *a) {
    u8 i, tail;
    u16 exponent;
    if (a->kind != 1) {
        sf128_special(raw, a->kind == 3 ? 0 : a->sign, a->kind);
        return;
    }
    sf128_normalize(a);
    if (!a->kind) {
        sf128_special(raw, a->sign, 0);
        return;
    }
    if (a->exponent > 16383) {
        sf128_special(raw, a->sign, 2);
        return;
    }
    if (a->exponent < -16382) {
        sf128_shr(a->mantissa, -16382 - a->exponent, 1, 16);
        a->exponent = -16382;
    }
    sf128_shr(a->mantissa, 5, 1, 16);
    tail = a->mantissa[0] & 7;
    sf128_shr(a->mantissa, 3, 0, 16);
    if (tail > 4 || (tail == 4 && (a->mantissa[0] & 1)))
        sf128_increment(a->mantissa, 16);
    if (a->mantissa[14] & 2) {
        sf128_right(a->mantissa, 16);
        ++a->exponent;
    }
    if (a->exponent > 16383) {
        sf128_special(raw, a->sign, 2);
        return;
    }
    exponent = a->exponent == -16382 && !(a->mantissa[14] & 1)
                   ? 0
                   : (u16)(a->exponent + 16383);
    for (i = 0; i < 14; ++i)
        raw[i] = a->mantissa[i];
    raw[14] = (u8)exponent;
    raw[15] = (exponent >> 8) | (a->sign << 7);
}

static void sf128_sum(Sf128 *r, Sf128 *a, Sf128 *b) {
    Sf128 *large = a, *small = b;
    signed char order;
    if (a->kind == 3 || b->kind == 3 ||
        (a->kind == 2 && b->kind == 2 && a->sign != b->sign)) {
        r->kind = 3;
        return;
    }
    if (a->kind == 2) {
        sf128_copy_float(r, a);
        return;
    }
    if (b->kind == 2) {
        sf128_copy_float(r, b);
        return;
    }
    if (!a->kind && !b->kind) {
        r->kind = 0;
        r->sign = a->sign & b->sign;
        return;
    }
    if (!a->kind) {
        sf128_copy_float(r, b);
        return;
    }
    if (!b->kind) {
        sf128_copy_float(r, a);
        return;
    }
    if (a->exponent < b->exponent ||
        (a->exponent == b->exponent &&
         sf128_cmp(a->mantissa, b->mantissa, 16) < 0)) {
        large = b;
        small = a;
    }
    sf128_shr(small->mantissa, large->exponent - small->exponent, 1, 16);
    if (large->sign == small->sign)
        sf128_add(large->mantissa, small->mantissa, 16);
    else {
        order = sf128_cmp(large->mantissa, small->mantissa, 16);
        if (!order) {
            r->kind = 0;
            r->sign = 0;
            return;
        }
        sf128_sub(large->mantissa, small->mantissa, 16);
    }
    sf128_normalize(large);
    sf128_copy_float(r, large);
}

static void sf128_product(Sf128 *r, Sf128 *a, Sf128 *b) {
    u8 i, sticky = 0;
    r->sign = a->sign ^ b->sign;
    r->exponent = 0;
    sf128_clear(r->mantissa, 16);
    if (a->kind == 3 || b->kind == 3 || (a->kind == 2 && !b->kind) ||
        (b->kind == 2 && !a->kind)) {
        r->kind = 3;
        return;
    }
    if (a->kind == 2 || b->kind == 2) {
        r->kind = 2;
        return;
    }
    if (!a->kind || !b->kind) {
        r->kind = 0;
        return;
    }
    sf128_shl(a->mantissa, 1, 16);
    sf128_shr(b->mantissa, 8, 0, 16);
    for (i = 0; i < 113; ++i) {
        if (b->mantissa[0] & 1)
            sf128_add(r->mantissa, a->mantissa, 16);
        sticky |= sf128_right(r->mantissa, 16);
        sf128_right(b->mantissa, 16);
    }
    r->mantissa[0] |= sticky;
    r->kind = 1;
    r->exponent = a->exponent + b->exponent;
    sf128_normalize(r);
}

static void sf128_quotient(Sf128 *r, Sf128 *a, Sf128 *b) {
    u8 i;
    r->sign = a->sign ^ b->sign;
    if (a->kind == 3 || b->kind == 3 || (a->kind == 2 && b->kind == 2) ||
        (!a->kind && !b->kind)) {
        r->kind = 3;
        return;
    }
    if (a->kind == 2 || !b->kind) {
        r->kind = 2;
        return;
    }
    if (!a->kind || b->kind == 2) {
        r->kind = 0;
        return;
    }
    r->kind = 1;
    r->exponent = a->exponent - b->exponent;
    sf128_shr(a->mantissa, 8, 0, 16);
    sf128_shr(b->mantissa, 8, 0, 16);
    if (sf128_cmp(a->mantissa, b->mantissa, 16) < 0) {
        sf128_left(a->mantissa, 16);
        --r->exponent;
    }
    sf128_clear(r->mantissa, 16);
    for (i = 0; i < 116; ++i) {
        sf128_left(r->mantissa, 16);
        if (sf128_cmp(a->mantissa, b->mantissa, 16) >= 0) {
            sf128_sub(a->mantissa, b->mantissa, 16);
            r->mantissa[0] |= 1;
        }
        sf128_left(a->mantissa, 16);
    }
    if (sf128_nonzero(a->mantissa, 16))
        r->mantissa[0] |= 1;
    sf128_shl(r->mantissa, 5, 16);
}

static void sf128_root(Sf128 *r, Sf128 *a) {
    u8 rem[16], trial[16], i, j, position, pair;
    sf128_copy_float(r, a);
    if (a->kind == 3 || (a->sign && a->kind)) {
        r->kind = 3;
        return;
    }
    if (a->kind != 1)
        return;
    sf128_shr(a->mantissa, 8, 0, 16);
    if (r->exponent & 1) {
        sf128_left(a->mantissa, 16);
        --r->exponent;
    }
    r->exponent /= 2;
    sf128_clear(rem, 16);
    sf128_clear(r->mantissa, 16);
    position = 231;
    for (i = 0; i < 116; ++i) {
        pair = 0;
        for (j = 0; j < 2; ++j) {
            pair <<= 1;
            if (position >= 118) {
                u8 bit = position - 118;
                pair |= (a->mantissa[bit >> 3] >> (bit & 7)) & 1;
            }
            --position;
        }
        sf128_shl(rem, 2, 16);
        rem[0] |= pair;
        sf128_left(r->mantissa, 16);
        sf128_copy(trial, r->mantissa, 16);
        sf128_left(trial, 16);
        trial[0] |= 1;
        if (sf128_cmp(rem, trial, 16) >= 0) {
            sf128_sub(rem, trial, 16);
            r->mantissa[0] |= 1;
        }
    }
    if (sf128_nonzero(rem, 16))
        r->mantissa[0] |= 1;
    sf128_shl(r->mantissa, 5, 16);
}

static u8 sf128_raw_kind(const u8 *raw) {
    u8 i, bits = 0;
    u16 exponent = ((u16)(raw[15] & 127) << 8) | raw[14];
    for (i = 0; i < 14; ++i)
        bits |= raw[i];
    if (exponent == 32767)
        return bits ? 3 : 2;
    return exponent || bits ? 1 : 0;
}

static u8 sf128_compare_raw(const u8 *a, const u8 *b) {
    u8 ak = sf128_raw_kind(a), bk = sf128_raw_kind(b), as = a[15] >> 7,
       bs = b[15] >> 7;
    u8 i = 16, x, y, mag = 1;
    if (ak == 3 || bk == 3)
        return 8;
    if (!ak && !bk)
        return 1;
    if (as != bs)
        return as ? 2 : 4;
    while (i) {
        --i;
        x = a[i];
        y = b[i];
        if (i == 15) {
            x &= 127;
            y &= 127;
        }
        if (x != y) {
            mag = x < y ? 2 : 4;
            break;
        }
    }
    if (as && mag != 1)
        mag ^= 6;
    return mag;
}

static void sf128_round(u8 *raw, Sf128 *a, u8 mode) {
    u8 *work = a->mantissa;
    u8 half = 0, sticky = 0, bit, up = 0;
    i16 count, i;
    if (a->kind != 1 || a->exponent >= 112) {
        sf128_pack(raw, a);
        return;
    }
    if (a->exponent < 0) {
        if (mode == 1)
            up = a->sign;
        if (mode == 2)
            up = !a->sign;
        if (mode == 3 && a->exponent == -1) {
            work[15] &= 254;
            up = sf128_nonzero(work, 16);
        }
        sf128_special(raw, a->sign, 0);
        if (up) {
            raw[14] = 255;
            raw[15] |= 63;
        }
        return;
    }
    count = 120 - a->exponent;
    for (i = 0; i < count; ++i) {
        bit = sf128_right(work, 16);
        if (i + 1 == count)
            half = bit;
        else
            sticky |= bit;
    }
    if (mode == 1)
        up = a->sign && (half || sticky);
    if (mode == 2)
        up = !a->sign && (half || sticky);
    if (mode == 3)
        up = half && (sticky || (work[0] & 1));
    if (up)
        sf128_increment(work, 16);
    sf128_shl(work, (u8)count, 16);
    sf128_pack(raw, a);
}

static void sf128_fused_finite(Sf128 *r, const Sf128 *a, Sf128 *b,
                               const Sf128 *c) {
    u8 product[29], addend[29], i;
    exponent_t pe = a->exponent + b->exponent, ce = c->exponent;
    u8 ps = a->sign ^ b->sign;
    sf128_clear(product, 29);
    sf128_clear(addend, 29);
    for (i = 0; i < 16; ++i)
        addend[i] = a->mantissa[i];
    sf128_shr(addend, 8, 0, 29);
    sf128_shr(b->mantissa, 8, 0, 16);
    for (i = 0; i < 113; ++i) {
        if (b->mantissa[0] & 1)
            sf128_add(product, addend, 29);
        sf128_left(addend, 29);
        sf128_right(b->mantissa, 16);
    }
    sf128_shl(product, 4, 29);
    if (product[28] & 32) {
        sf128_shr(product, 1, 1, 29);
        ++pe;
    }
    sf128_clear(addend, 29);
    for (i = 0; i < 16; ++i)
        addend[i] = c->mantissa[i];
    sf128_shl(addend, 108, 29);
    if (pe > ce) {
        sf128_shr(addend, pe - ce, 1, 29);
        ce = pe;
    } else if (ce > pe) {
        sf128_shr(product, ce - pe, 1, 29);
        pe = ce;
    }
    r->kind = 1;
    r->exponent = pe;
    if (ps == c->sign) {
        sf128_add(product, addend, 29);
        r->sign = ps;
    } else if (sf128_cmp(product, addend, 29) >= 0) {
        sf128_sub(product, addend, 29);
        r->sign = ps;
    } else {
        sf128_sub(addend, product, 29);
        sf128_copy(product, addend, 29);
        r->sign = c->sign;
    }
    if (!sf128_nonzero(product, 29)) {
        r->kind = 0;
        r->sign = 0;
        return;
    }
    if (product[28] & 32) {
        sf128_shr(product, 1, 1, 29);
        ++r->exponent;
    }
    while (!(product[28] & 16)) {
        sf128_left(product, 29);
        --r->exponent;
    }
    sf128_shr(product, 108, 1, 29);
    sf128_copy(r->mantissa, product, 16);
}

static void sf128_fma_context(u8 *ctx) {
    Sf128 a, b, c, result;
    sf128_unpack(&a, ctx);
    sf128_unpack(&b, ctx + 16);
    sf128_unpack(&c, ctx + 32);
    if (a.kind != 1 || b.kind != 1 || c.kind != 1) {
        if (a.kind == 1 && b.kind == 1 && c.kind >= 2)
            sf128_copy_float(&result, &c);
        else {
            sf128_product(&result, &a, &b);
            if (c.kind)
                sf128_sum(&result, &result, &c);
            else if (!result.kind)
                result.sign &= c.sign;
        }
    } else
        sf128_fused_finite(&result, &a, &b, &c);
    sf128_pack(ctx + 48, &result);
}

static void sf128_compare_context(u8 *ctx) {
    u8 op = ctx[64], relation = sf128_compare_raw(ctx, ctx + 16), i;
    const u8 *selected;
    if (op == 12) {
        ctx[48] = (relation & ctx[65]) != 0;
        return;
    }
    if (relation == 8) {
        sf128_special(ctx + 48, 0, 3);
        return;
    }
    if (!sf128_raw_kind(ctx) && !sf128_raw_kind(ctx + 16)) {
        u8 sign = op == 6 ? ctx[15] | ctx[31] : ctx[15] & ctx[31];
        sf128_special(ctx + 48, sign >> 7, 0);
        return;
    }
    selected = (op == 6 && relation == 4) || (op == 7 && relation == 2)
                   ? ctx + 16
                   : ctx;
    for (i = 0; i < 16; ++i)
        ctx[48 + i] = selected[i];
}

static void sf128_binary(u8 *ctx) {
    Sf128 a, b, result;
    sf128_unpack(&a, ctx);
    sf128_unpack(&b, ctx + 16);
    switch (ctx[64]) {
    case 0:
        sf128_sum(&result, &a, &b);
        break;
    case 1:
        b.sign ^= 1;
        sf128_sum(&result, &a, &b);
        break;
    case 2:
        sf128_product(&result, &a, &b);
        break;
    default:
        sf128_quotient(&result, &a, &b);
        break;
    }
    sf128_pack(ctx + 48, &result);
}

static void sf128_unary(u8 *ctx) {
    Sf128 a, result;
    sf128_unpack(&a, ctx);
    if (ctx[64] == 4) {
        sf128_root(&result, &a);
        sf128_pack(ctx + 48, &result);
    } else
        sf128_round(ctx + 48, &a, ctx[64] - 8);
}

void sangoku_sf128(u8 *ctx) {
    u8 op = ctx[64];
    if (op < 4)
        sf128_binary(ctx);
    else if (op == 5)
        sf128_fma_context(ctx);
    else if (op == 6 || op == 7 || op == 12)
        sf128_compare_context(ctx);
    else
        sf128_unary(ctx);
}
