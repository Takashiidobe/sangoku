#include <stdint.h>

typedef unsigned char u8;
typedef unsigned short u16;
typedef int32_t i32;

typedef struct {
    u8 bytes, fraction, exponent;
    i32 bias;
} Format;

typedef struct {
    u8 sign, kind, mantissa[17];
    i32 exponent;
} Number;

static void clear(u8 *a, u8 n) {
    u8 i;
    for (i = 0; i < n; ++i)
        a[i] = 0;
}

static u8 nonzero(const u8 *a, u8 n) {
    u8 i, result = 0;
    for (i = 0; i < n; ++i)
        result |= a[i];
    return result != 0;
}

static u8 bit(const u8 *a, u8 index) {
    return (a[index >> 3] >> (index & 7)) & 1;
}

static void setbit(u8 *a, u8 index) {
    a[index >> 3] |= (u8)1 << (index & 7);
}

static void left(u8 *a) {
    u8 i, carry = 0, next;
    for (i = 0; i < 17; ++i) {
        next = a[i] >> 7;
        a[i] = (a[i] << 1) | carry;
        carry = next;
    }
}

static u8 right(u8 *a) {
    u8 i = 17, carry = 0, next;
    while (i) {
        --i;
        next = a[i] & 1;
        a[i] = (a[i] >> 1) | (carry << 7);
        carry = next;
    }
    return carry;
}

static void shift(u8 *a, i32 count, u8 jam) {
    u8 sticky = 0;
    u16 n;
    if (count > 136)
        count = 136;
    n = (u16)count;
    while (n--)
        sticky |= right(a);
    if (jam)
        a[0] |= sticky;
}

static void increment(u8 *a) {
    u8 i;
    for (i = 0; i < 17; ++i)
        if (++a[i])
            break;
}

static void negate(u8 *a, u8 n) {
    u8 i, carry = 1;
    u16 value;
    for (i = 0; i < n; ++i) {
        value = (u16)(u8)~a[i] + carry;
        a[i] = (u8)value;
        carry = value >> 8;
    }
}

static void format(Format *f, u8 bytes) {
    f->bytes = bytes;
    if (bytes == 2) {
        f->fraction = 10;
        f->exponent = 5;
        f->bias = 15;
    } else if (bytes == 4) {
        f->fraction = 23;
        f->exponent = 8;
        f->bias = 127;
    } else if (bytes == 8) {
        f->fraction = 52;
        f->exponent = 11;
        f->bias = 1023;
    } else {
        f->fraction = 112;
        f->exponent = 15;
        f->bias = 16383;
    }
}

static void normalize(Number *a) {
    if (!nonzero(a->mantissa, 17)) {
        a->kind = 0;
        return;
    }
    while (!a->mantissa[16]) {
        left(a->mantissa);
        --a->exponent;
    }
}

static void unpack(Number *a, const u8 *raw, const Format *f) {
    u8 i;
    u16 exponent = 0, maximum = ((u16)1 << f->exponent) - 1;
    clear(a->mantissa, 17);
    a->sign = raw[f->bytes - 1] >> 7;
    for (i = 0; i < f->fraction; ++i)
        if (bit(raw, i))
            setbit(a->mantissa, i);
    for (i = 0; i < f->exponent; ++i)
        exponent |= (u16)bit(raw, f->fraction + i) << i;
    a->kind = 1;
    a->exponent = 128;
    if (exponent == maximum) {
        a->kind = nonzero(a->mantissa, 17) ? 3 : 2;
        return;
    }
    if (exponent)
        setbit(a->mantissa, f->fraction);
    normalize(a);
    a->exponent += (exponent ? (i32)exponent : 1) - f->bias - f->fraction;
}

static void pack(u8 *raw, Number *a, const Format *f) {
    u8 i, tail;
    u16 exponent = 0, maximum = ((u16)1 << f->exponent) - 1;
    clear(raw, f->bytes);
    if (a->kind == 1) {
        if (a->exponent < 1 - f->bias) {
            shift(a->mantissa, 1 - f->bias - a->exponent, 1);
            a->exponent = 1 - f->bias;
        }
        shift(a->mantissa, 126 - f->fraction, 1);
        tail = a->mantissa[0] & 3;
        right(a->mantissa);
        right(a->mantissa);
        if (tail > 2 || (tail == 2 && (a->mantissa[0] & 1)))
            increment(a->mantissa);
        if (bit(a->mantissa, f->fraction + 1)) {
            right(a->mantissa);
            ++a->exponent;
        }
        if (a->exponent > f->bias)
            a->kind = 2;
        else {
            exponent = bit(a->mantissa, f->fraction)
                           ? (u16)(a->exponent + f->bias) : 0;
            for (i = 0; i < f->fraction; ++i)
                if (bit(a->mantissa, i))
                    setbit(raw, i);
        }
    }
    if (a->kind >= 2) {
        exponent = maximum;
        if (a->kind == 3)
            setbit(raw, f->fraction - 1);
    }
    for (i = 0; i < f->exponent; ++i)
        if ((exponent >> i) & 1)
            setbit(raw, f->fraction + i);
    raw[f->bytes - 1] |= a->sign << 7;
}

static void from_integer(Number *a, const u8 *raw, u8 bytes, u8 signed_) {
    u8 i;
    clear(a->mantissa, 17);
    for (i = 0; i < bytes; ++i)
        a->mantissa[i] = raw[i];
    a->sign = signed_ && (raw[bytes - 1] & 128);
    if (a->sign)
        negate(a->mantissa, bytes);
    a->kind = 1;
    a->exponent = 128;
    normalize(a);
}

static u8 to_integer(u8 *raw, Number *a, u8 bytes, u8 signed_) {
    u8 i, overflow = 0, bits = bytes * 8, limit = bits - signed_;
    clear(raw, bytes);
    if (a->kind == 3)
        return 2;
    if (!a->kind)
        return 0;
    if (a->kind == 2 || a->exponent >= bits)
        overflow = 1;
    else if (a->exponent < 0)
        return 0;
    else {
        shift(a->mantissa, 128 - a->exponent, 0);
        if (signed_ && bit(a->mantissa, limit)) {
            a->mantissa[bytes - 1] &= 127;
            overflow = !a->sign || nonzero(a->mantissa, 17);
            a->mantissa[bytes - 1] |= 128;
        }
        if (!signed_ && a->sign)
            overflow = nonzero(a->mantissa, 17);
    }
    if (overflow) {
        if (!a->sign) {
            for (i = 0; i < bytes; ++i)
                raw[i] = 255;
            if (signed_)
                raw[bytes - 1] = 127;
        } else if (signed_)
            raw[bytes - 1] = 128;
        return 1;
    }
    for (i = 0; i < bytes; ++i)
        raw[i] = a->mantissa[i];
    if (a->sign)
        negate(raw, bytes);
    return 0;
}

void sangoku_convert(u8 *ctx) {
    Number a;
    Format f;
    u8 source = ctx[32], destination = ctx[33], mode = ctx[34];
    if (mode & 1)
        from_integer(&a, ctx, source, (mode >> 2) & 1);
    else {
        format(&f, source);
        unpack(&a, ctx, &f);
    }
    ctx[35] = 0;
    if (mode & 2)
        ctx[35] = to_integer(ctx + 16, &a, destination, (mode >> 2) & 1);
    else {
        format(&f, destination);
        pack(ctx + 16, &a, &f);
    }
}
