typedef unsigned char u8;
typedef unsigned short u16;
typedef signed short i16;

typedef struct {
    u8 sign;
    u8 kind;
    i16 exponent;
    u8 mantissa[16];
} Sf64;

static void sf64_clear(u8 *a) {
    u8 i;
    for (i = 0; i < 16; ++i)
        a[i] = 0;
}

static void sf64_copy(u8 *a, const u8 *b) {
    u8 i;
    for (i = 0; i < 16; ++i)
        a[i] = b[i];
}

static u8 sf64_nonzero(const u8 *a) {
    u8 i, bits = 0;
    for (i = 0; i < 16; ++i)
        bits |= a[i];
    return bits != 0;
}

static u8 sf64_left(u8 *a) {
    u8 i, carry = 0, next;
    for (i = 0; i < 16; ++i) {
        next = a[i] >> 7;
        a[i] = (a[i] << 1) | carry;
        carry = next;
    }
    return carry;
}

static u8 sf64_right(u8 *a) {
    u8 i = 16, carry = 0, next;
    while (i) {
        --i;
        next = a[i] & 1;
        a[i] = (a[i] >> 1) | (carry << 7);
        carry = next;
    }
    return carry;
}

static void sf64_shl(u8 *a, u8 count) {
    while (count--)
        sf64_left(a);
}

static void sf64_shr(u8 *a, i16 count, u8 jam) {
    u8 sticky = 0;
    if (count > 128)
        count = 128;
    while (count-- > 0)
        sticky |= sf64_right(a);
    if (jam)
        a[0] |= sticky;
}

static u8 sf64_add(u8 *a, const u8 *b) {
    u8 i, carry = 0;
    u16 sum;
    for (i = 0; i < 16; ++i) {
        sum = (u16)a[i] + b[i] + carry;
        a[i] = (u8)sum;
        carry = (u8)(sum >> 8);
    }
    return carry;
}

static void sf64_increment(u8 *a) {
    u8 i;
    for (i = 0; i < 16; ++i)
        if (++a[i])
            break;
}

static void sf64_sub(u8 *a, const u8 *b) {
    u8 i, borrow = 0, next;
    u16 sub;
    for (i = 0; i < 16; ++i) {
        sub = (u16)b[i] + borrow;
        next = (u16)a[i] < sub;
        a[i] = (u8)((u16)a[i] - sub);
        borrow = next;
    }
}

static signed char sf64_cmp(const u8 *a, const u8 *b) {
    u8 i = 16;
    while (i) {
        --i;
        if (a[i] != b[i])
            return a[i] < b[i] ? -1 : 1;
    }
    return 0;
}

static void sf64_copy_float(Sf64 *a, const Sf64 *b) {
    a->sign = b->sign;
    a->kind = b->kind;
    a->exponent = b->exponent;
    sf64_copy(a->mantissa, b->mantissa);
}

static void sf64_unpack(Sf64 *a, const u8 *raw) {
    u8 i;
    u16 exponent = ((u16)(raw[7] & 127) << 4) | (raw[6] >> 4);
    a->sign = raw[7] >> 7;
    a->exponent = 0;
    sf64_clear(a->mantissa);
    for (i = 0; i < 6; ++i)
        a->mantissa[i] = raw[i];
    a->mantissa[6] = raw[6] & 15;
    if (exponent == 2047) {
        a->kind = sf64_nonzero(a->mantissa) ? 3 : 2;
        return;
    }
    if (!exponent && !sf64_nonzero(a->mantissa)) {
        a->kind = 0;
        return;
    }
    a->kind = 1;
    a->exponent = exponent ? (i16)exponent - 1023 : -1022;
    if (exponent)
        a->mantissa[6] |= 16;
    while (!(a->mantissa[6] & 16)) {
        sf64_left(a->mantissa);
        --a->exponent;
    }
    sf64_shl(a->mantissa, 68);
}

static void sf64_normalize(Sf64 *a) {
    if (!sf64_nonzero(a->mantissa)) {
        a->kind = 0;
        return;
    }
    while (a->mantissa[15] > 1) {
        sf64_shr(a->mantissa, 1, 1);
        ++a->exponent;
    }
    while (!a->mantissa[15]) {
        sf64_left(a->mantissa);
        --a->exponent;
    }
}

static void sf64_special(u8 *raw, u8 sign, u8 kind) {
    u8 i;
    for (i = 0; i < 8; ++i)
        raw[i] = 0;
    raw[7] = sign << 7;
    if (kind >= 2) {
        raw[7] |= 127;
        raw[6] = kind == 3 ? 248 : 240;
    }
}

static void sf64_pack(u8 *raw, Sf64 *a) {
    u8 i, tail;
    u16 exponent;
    if (a->kind != 1) {
        sf64_special(raw, a->kind == 3 ? 0 : a->sign, a->kind);
        return;
    }
    sf64_normalize(a);
    if (!a->kind) {
        sf64_special(raw, a->sign, 0);
        return;
    }
    if (a->exponent > 1023) {
        sf64_special(raw, a->sign, 2);
        return;
    }
    if (a->exponent < -1022) {
        sf64_shr(a->mantissa, -1022 - a->exponent, 1);
        a->exponent = -1022;
    }
    sf64_shr(a->mantissa, 65, 1);
    tail = a->mantissa[0] & 7;
    sf64_shr(a->mantissa, 3, 0);
    if (tail > 4 || (tail == 4 && (a->mantissa[0] & 1)))
        sf64_increment(a->mantissa);
    if (a->mantissa[6] & 32) {
        sf64_right(a->mantissa);
        ++a->exponent;
    }
    if (a->exponent > 1023) {
        sf64_special(raw, a->sign, 2);
        return;
    }
    exponent = a->exponent == -1022 && !(a->mantissa[6] & 16)
                   ? 0
                   : (u16)(a->exponent + 1023);
    for (i = 0; i < 6; ++i)
        raw[i] = a->mantissa[i];
    raw[6] = (a->mantissa[6] & 15) | ((exponent & 15) << 4);
    raw[7] = (exponent >> 4) | (a->sign << 7);
}

static void sf64_sum(Sf64 *r, const Sf64 *a, const Sf64 *b) {
    u8 other[16];
    const Sf64 *large = a, *small = b;
    signed char order;
    if (a->kind == 3 || b->kind == 3 ||
        (a->kind == 2 && b->kind == 2 && a->sign != b->sign)) {
        r->kind = 3;
        return;
    }
    if (a->kind == 2) {
        sf64_copy_float(r, a);
        return;
    }
    if (b->kind == 2) {
        sf64_copy_float(r, b);
        return;
    }
    if (!a->kind && !b->kind) {
        r->kind = 0;
        r->sign = a->sign & b->sign;
        return;
    }
    if (!a->kind) {
        sf64_copy_float(r, b);
        return;
    }
    if (!b->kind) {
        sf64_copy_float(r, a);
        return;
    }
    if (a->exponent < b->exponent || (a->exponent == b->exponent &&
                                      sf64_cmp(a->mantissa, b->mantissa) < 0)) {
        large = b;
        small = a;
    }
    sf64_copy_float(r, large);
    sf64_copy(other, small->mantissa);
    sf64_shr(other, large->exponent - small->exponent, 1);
    if (large->sign == small->sign)
        sf64_add(r->mantissa, other);
    else {
        order = sf64_cmp(r->mantissa, other);
        if (!order) {
            r->kind = 0;
            r->sign = 0;
            return;
        }
        sf64_sub(r->mantissa, other);
    }
    sf64_normalize(r);
}

static void sf64_product(Sf64 *r, const Sf64 *a, const Sf64 *b) {
    u8 x[16], y[16], i;
    r->sign = a->sign ^ b->sign;
    r->exponent = 0;
    sf64_clear(r->mantissa);
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
    sf64_copy(x, a->mantissa);
    sf64_shr(x, 68, 0);
    sf64_copy(y, b->mantissa);
    sf64_shr(y, 68, 0);
    sf64_clear(r->mantissa);
    for (i = 0; i < 53; ++i) {
        if (y[0] & 1)
            sf64_add(r->mantissa, x);
        sf64_left(x);
        sf64_right(y);
    }
    r->kind = 1;
    r->exponent = a->exponent + b->exponent;
    sf64_shl(r->mantissa, 16);
    sf64_normalize(r);
}

static void sf64_quotient(Sf64 *r, const Sf64 *a, const Sf64 *b) {
    u8 x[16], divisor[16], rem[16], i, bit;
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
    sf64_copy(x, a->mantissa);
    sf64_shr(x, 68, 0);
    sf64_copy(divisor, b->mantissa);
    sf64_shr(divisor, 68, 0);
    if (sf64_cmp(x, divisor) < 0) {
        sf64_shl(x, 56);
        --r->exponent;
    } else
        sf64_shl(x, 55);
    sf64_clear(rem);
    for (i = 0; i < 128; ++i) {
        bit = sf64_left(x);
        sf64_left(rem);
        rem[0] |= bit;
        if (sf64_cmp(rem, divisor) >= 0) {
            sf64_sub(rem, divisor);
            x[0] |= 1;
        }
    }
    if (sf64_nonzero(rem))
        x[0] |= 1;
    sf64_copy(r->mantissa, x);
    sf64_shl(r->mantissa, 65);
}

static void sf64_root(Sf64 *r, const Sf64 *a) {
    u8 rad[16], rem[16], trial[16], i, pair;
    sf64_copy_float(r, a);
    if (a->kind == 3 || (a->sign && a->kind)) {
        r->kind = 3;
        return;
    }
    if (a->kind != 1)
        return;
    sf64_copy(rad, a->mantissa);
    sf64_shr(rad, 68, 0);
    if (r->exponent & 1) {
        sf64_left(rad);
        --r->exponent;
    }
    r->exponent /= 2;
    sf64_shl(rad, 58);
    sf64_clear(rem);
    sf64_clear(r->mantissa);
    for (i = 0; i < 64; ++i) {
        pair = rad[15] >> 6;
        sf64_shl(rad, 2);
        sf64_shl(rem, 2);
        rem[0] |= pair;
        sf64_left(r->mantissa);
        sf64_copy(trial, r->mantissa);
        sf64_left(trial);
        trial[0] |= 1;
        if (sf64_cmp(rem, trial) >= 0) {
            sf64_sub(rem, trial);
            r->mantissa[0] |= 1;
        }
    }
    if (sf64_nonzero(rem))
        r->mantissa[0] |= 1;
    sf64_shl(r->mantissa, 65);
}

static u8 sf64_compare_raw(const u8 *a, const u8 *b, const Sf64 *fa,
                           const Sf64 *fb) {
    u8 i = 8, x, y, mag = 1;
    if (fa->kind == 3 || fb->kind == 3)
        return 8;
    if (!fa->kind && !fb->kind)
        return 1;
    if (fa->sign != fb->sign)
        return fa->sign ? 2 : 4;
    while (i) {
        --i;
        x = a[i];
        y = b[i];
        if (i == 7) {
            x &= 127;
            y &= 127;
        }
        if (x != y) {
            mag = x < y ? 2 : 4;
            break;
        }
    }
    if (fa->sign && mag != 1)
        mag ^= 6;
    return mag;
}

static void sf64_round(u8 *raw, Sf64 *a, u8 mode) {
    u8 work[16], half = 0, sticky = 0, bit, up = 0;
    i16 count, i;
    if (a->kind != 1 || a->exponent >= 52) {
        sf64_pack(raw, a);
        return;
    }
    if (a->exponent < 0) {
        if (mode == 1)
            up = a->sign;
        if (mode == 2)
            up = !a->sign;
        if (mode == 3 && a->exponent == -1) {
            sf64_copy(work, a->mantissa);
            work[15] &= 254;
            up = sf64_nonzero(work);
        }
        sf64_special(raw, a->sign, 0);
        if (up) {
            raw[6] = 240;
            raw[7] |= 63;
        }
        return;
    }
    sf64_copy(work, a->mantissa);
    count = 120 - a->exponent;
    for (i = 0; i < count; ++i) {
        bit = sf64_right(work);
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
        sf64_increment(work);
    sf64_shl(work, (u8)count);
    sf64_copy(a->mantissa, work);
    sf64_pack(raw, a);
}

void sangoku_sf64(u8 *ctx) {
    Sf64 a, b, c, product, result;
    u8 op = ctx[32], relation, i;
    const u8 *selected;
    sf64_unpack(&a, ctx);
    if (op != 4 && (op < 8 || op == 12))
        sf64_unpack(&b, ctx + 8);
    switch (op) {
    case 0:
        sf64_sum(&result, &a, &b);
        break;
    case 1:
        b.sign ^= 1;
        sf64_sum(&result, &a, &b);
        break;
    case 2:
        sf64_product(&result, &a, &b);
        break;
    case 3:
        sf64_quotient(&result, &a, &b);
        break;
    case 4:
        sf64_root(&result, &a);
        break;
    case 5:
        sf64_unpack(&c, ctx + 16);
        sf64_product(&product, &a, &b);
        sf64_sum(&result, &product, &c);
        break;
    case 6:
    case 7:
    case 12:
        relation = sf64_compare_raw(ctx, ctx + 8, &a, &b);
        if (op == 12) {
            ctx[24] = (relation & ctx[33]) != 0;
            return;
        }
        if (relation == 8) {
            sf64_special(ctx + 24, 0, 3);
            return;
        }
        if (!a.kind && !b.kind) {
            sf64_special(ctx + 24, op == 6 ? a.sign | b.sign : a.sign & b.sign,
                         0);
            return;
        }
        selected = (op == 6 && relation == 4) || (op == 7 && relation == 2)
                       ? ctx + 8
                       : ctx;
        for (i = 0; i < 8; ++i)
            ctx[24 + i] = selected[i];
        return;
    default:
        sf64_round(ctx + 24, &a, op - 8);
        return;
    }
    sf64_pack(ctx + 24, &result);
}
