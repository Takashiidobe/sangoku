typedef unsigned int u16;
typedef unsigned long u32;
#define SHIFTS(T, S, W, N) \
T shl##W##_##N(T v) { return v << N; } \
T shr##W##_##N(T v) { return v >> N; } \
T sar##W##_##N(S v) { return v >> N; } \
T rol##W##_##N(T v) { return (v << N) | (v >> ((W - N) & (W - 1))); } \
T ror##W##_##N(T v) { return (v >> N) | (v << ((W - N) & (W - 1))); }
#define BOTH(N) SHIFTS(u16, int, 16, N) SHIFTS(u32, long, 32, N)
BOTH(0) BOTH(1) BOTH(2) BOTH(3) BOTH(4) BOTH(5) BOTH(6) BOTH(7)
BOTH(8) BOTH(9) BOTH(10) BOTH(11) BOTH(12) BOTH(13) BOTH(14) BOTH(15)
SHIFTS(u32, long, 32, 16) SHIFTS(u32, long, 32, 17)
SHIFTS(u32, long, 32, 18) SHIFTS(u32, long, 32, 19)
SHIFTS(u32, long, 32, 20) SHIFTS(u32, long, 32, 21)
SHIFTS(u32, long, 32, 22) SHIFTS(u32, long, 32, 23)
SHIFTS(u32, long, 32, 24) SHIFTS(u32, long, 32, 25)
SHIFTS(u32, long, 32, 26) SHIFTS(u32, long, 32, 27)
SHIFTS(u32, long, 32, 28) SHIFTS(u32, long, 32, 29)
SHIFTS(u32, long, 32, 30) SHIFTS(u32, long, 32, 31)
#define VARIABLE(T, S, W) \
T shl##W##_var(T v, unsigned char n) { return v << (n & (W - 1)); } \
T shr##W##_var(T v, unsigned char n) { return v >> (n & (W - 1)); } \
T sar##W##_var(S v, unsigned char n) { return v >> (n & (W - 1)); } \
T rol##W##_var(T v, unsigned char n) { n &= W - 1; return (v << n) | (v >> ((-n) & (W - 1))); } \
T ror##W##_var(T v, unsigned char n) { n &= W - 1; return (v >> n) | (v << ((-n) & (W - 1))); }
VARIABLE(u16, int, 16) VARIABLE(u32, long, 32)
