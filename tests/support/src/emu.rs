use crate::{Result, bail, ensure};
use std::collections::BTreeSet;

pub const ZERO: u8 = 0x80;
pub const SUBTRACT: u8 = 0x40;
pub const HALF_CARRY: u8 = 0x20;
pub const CARRY: u8 = 0x10;

#[derive(Clone)]
pub struct Emulator {
    pub memory: Box<[u8; 65536]>,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    sp: u16,
    stack_depth: i32,
    pub cycles: u64,
    pub peak_stack_bytes: i32,
    pub written_addresses: BTreeSet<u16>,
}

impl Emulator {
    pub fn new(code: &[u8], pc: u16) -> Result<Self> {
        ensure!(code.len() <= 65536, "code exceeds the 16-bit address space");
        let mut memory = Box::new([0; 65536]);
        memory[..code.len()].copy_from_slice(code);
        Ok(Self {
            memory,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc,
            sp: 0,
            stack_depth: 0,
            cycles: 0,
            peak_stack_bytes: 0,
            written_addresses: BTreeSet::new(),
        })
    }
    pub fn reset_metrics(&mut self) {
        self.cycles = 0;
        self.stack_depth = 0;
        self.peak_stack_bytes = 0;
        self.written_addresses.clear();
    }
    pub fn sp(&self) -> u16 {
        self.sp
    }
    pub fn set_sp(&mut self, value: u16) {
        self.stack_depth += self.sp.wrapping_sub(value) as i16 as i32;
        self.peak_stack_bytes = self.peak_stack_bytes.max(self.stack_depth);
        self.sp = value;
    }
    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }
    pub fn set_hl(&mut self, value: u16) {
        [self.h, self.l] = value.to_be_bytes();
    }
    pub fn write8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
        self.written_addresses.insert(address);
    }
    pub fn read16(&self, address: u16) -> u16 {
        u16::from_le_bytes([
            self.memory[address as usize],
            self.memory[address.wrapping_add(1) as usize],
        ])
    }
    pub fn write16(&mut self, address: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write8(address, lo);
        self.write8(address.wrapping_add(1), hi);
    }
    fn fetch(&mut self) -> u8 {
        let value = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        value
    }
    fn fetch16(&mut self) -> u16 {
        let lo = self.fetch();
        u16::from_le_bytes([lo, self.fetch()])
    }
    pub fn read8(&self, r: u8) -> u8 {
        match r {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => self.memory[self.hl() as usize],
            7 => self.a,
            _ => unreachable!(),
        }
    }
    pub fn write_reg(&mut self, r: u8, v: u8) {
        match r {
            0 => self.b = v,
            1 => self.c = v,
            2 => self.d = v,
            3 => self.e = v,
            4 => self.h = v,
            5 => self.l = v,
            6 => self.write8(self.hl(), v),
            7 => self.a = v,
            _ => unreachable!(),
        }
    }
    pub fn reg(&self, name: char) -> u8 {
        match name {
            'a' => self.a,
            'b' => self.b,
            'c' => self.c,
            'd' => self.d,
            'e' => self.e,
            'f' => self.f,
            'h' => self.h,
            'l' => self.l,
            _ => panic!("unknown register {name}"),
        }
    }
    pub fn set_reg(&mut self, name: char, v: u8) {
        match name {
            'a' => self.a = v,
            'b' => self.b = v,
            'c' => self.c = v,
            'd' => self.d = v,
            'e' => self.e = v,
            'f' => self.f = v,
            'h' => self.h = v,
            'l' => self.l = v,
            _ => panic!("unknown register {name}"),
        }
    }
    pub fn read_pair(&self, p: u8) -> u16 {
        if p == 3 {
            self.sp
        } else {
            u16::from_be_bytes([self.read8(p * 2), self.read8(p * 2 + 1)])
        }
    }
    fn write_pair(&mut self, p: u8, v: u16) {
        if p == 3 {
            self.set_sp(v)
        } else {
            let [hi, lo] = v.to_be_bytes();
            self.write_reg(p * 2, hi);
            self.write_reg(p * 2 + 1, lo)
        }
    }
    fn push(&mut self, v: u16) {
        self.set_sp(self.sp.wrapping_sub(2));
        self.write16(self.sp, v)
    }
    fn pop(&mut self) -> u16 {
        let v = self.read16(self.sp);
        self.set_sp(self.sp.wrapping_add(2));
        v
    }
    fn condition(&self, c: u8) -> bool {
        (self.f & if c < 2 { ZERO } else { CARRY } != 0) == (c & 1 != 0)
    }
    fn alu(&mut self, op: u8, rhs: u8) {
        let lhs = self.a as i32;
        let rhs = rhs as i32;
        let carry = if op == 1 || op == 3 {
            i32::from(self.f & CARRY != 0)
        } else {
            0
        };
        let result = if op < 4 || op == 7 {
            let sub = op == 2 || op == 3 || op == 7;
            let v = if sub {
                lhs - rhs - carry
            } else {
                lhs + rhs + carry
            };
            let half = if sub {
                (lhs & 15) - (rhs & 15) - carry < 0
            } else {
                (lhs & 15) + (rhs & 15) + carry > 15
            };
            self.f = flag(sub, SUBTRACT)
                | flag(v & 255 == 0, ZERO)
                | flag(half, HALF_CARRY)
                | flag(!(0..=255).contains(&v), CARRY);
            v as u8
        } else {
            let v = match op {
                4 => lhs & rhs,
                5 => lhs ^ rhs,
                6 => lhs | rhs,
                _ => unreachable!(),
            } as u8;
            self.f = flag(v == 0, ZERO) | flag(op == 4, HALF_CARRY);
            v
        };
        if op != 7 {
            self.a = result
        }
    }
    pub fn step(&mut self) -> Result<u32> {
        let address = self.pc;
        let op = self.fetch();
        let cost = if op == 0 {
            4
        } else if op == 0xC9 {
            self.pc = self.pop();
            16
        } else if [0xC0, 0xC8, 0xD0, 0xD8].contains(&op) {
            let taken = self.condition((op >> 3) & 3);
            if taken {
                self.pc = self.pop()
            }
            if taken { 20 } else { 8 }
        } else if [0xCD, 0xC4, 0xCC, 0xD4, 0xDC].contains(&op) {
            let target = self.fetch16();
            let taken = op == 0xCD || self.condition((op >> 3) & 3);
            if taken {
                self.push(self.pc);
                self.pc = target
            }
            if taken { 24 } else { 12 }
        } else if op & 0xC7 == 0xC7 {
            self.push(self.pc);
            self.pc = (op & 0x38) as u16;
            16
        } else if op & 0xCF == 0xC5 {
            let p = (op >> 4) & 3;
            let v = if p == 3 {
                u16::from_be_bytes([self.a, self.f & 0xF0])
            } else {
                self.read_pair(p)
            };
            self.push(v);
            16
        } else if op & 0xCF == 0xC1 {
            let p = (op >> 4) & 3;
            let v = self.pop();
            if p == 3 {
                self.a = (v >> 8) as u8;
                self.f = v as u8 & 0xF0
            } else {
                self.write_pair(p, v)
            }
            12
        } else if [0xC3, 0xC2, 0xCA, 0xD2, 0xDA].contains(&op) {
            let target = self.fetch16();
            let taken = op == 0xC3 || self.condition((op >> 3) & 3);
            if taken {
                self.pc = target
            }
            if taken { 16 } else { 12 }
        } else if op == 0xE9 {
            self.pc = self.hl();
            4
        } else if op == 0xEA || op == 0xFA {
            let target = self.fetch16();
            if op == 0xEA {
                self.write8(target, self.a)
            } else {
                self.a = self.memory[target as usize]
            }
            16
        } else if op == 0xF9 {
            self.set_sp(self.hl());
            8
        } else if op & 0xCF == 3 || op & 0xCF == 0x0B {
            let p = (op >> 4) & 3;
            let v = self
                .read_pair(p)
                .wrapping_add(if op & 8 != 0 { u16::MAX } else { 1 });
            self.write_pair(p, v);
            8
        } else if (0x40..=0x7F).contains(&op) && op != 0x76 {
            let dst = (op >> 3) & 7;
            let src = op & 7;
            self.write_reg(dst, self.read8(src));
            if dst == 6 || src == 6 { 8 } else { 4 }
        } else if op & 0xC7 == 6 {
            let r = (op >> 3) & 7;
            let v = self.fetch();
            self.write_reg(r, v);
            if r == 6 { 12 } else { 8 }
        } else if op & 0xCF == 1 {
            let v = self.fetch16();
            self.write_pair((op >> 4) & 3, v);
            12
        } else if op == 0xF8 || op == 0xE8 {
            let offset = self.fetch();
            self.f = flag((self.sp & 15) + (offset as u16 & 15) > 15, HALF_CARRY)
                | flag((self.sp & 255) + offset as u16 > 255, CARRY);
            let v = self.sp.wrapping_add_signed(offset as i8 as i16);
            if op == 0xF8 {
                self.set_hl(v)
            } else {
                self.set_sp(v)
            }
            if op == 0xF8 { 12 } else { 16 }
        } else if op & 0xCF == 9 {
            let lhs = self.hl() as u32;
            let rhs = self.read_pair((op >> 4) & 3) as u32;
            self.set_hl((lhs + rhs) as u16);
            self.f = (self.f & ZERO)
                | flag((lhs & 0xFFF) + (rhs & 0xFFF) > 0xFFF, HALF_CARRY)
                | flag(lhs + rhs > 65535, CARRY);
            8
        } else if op == 0xCB {
            let sub = self.fetch();
            let r = sub & 7;
            let v = self.read8(r);
            let incoming = u8::from(self.f & CARRY != 0);
            if (0x40..0x80).contains(&sub) {
                self.f =
                    (self.f & CARRY) | HALF_CARRY | flag(v & (1 << ((sub >> 3) & 7)) == 0, ZERO);
                if r == 6 { 12 } else { 8 }
            } else {
                let (result, carry) = match sub >> 3 {
                    0 => (v.rotate_left(1), v >> 7),
                    1 => (v.rotate_right(1), v & 1),
                    2 => (v.wrapping_shl(1) | incoming, v >> 7),
                    3 => (v >> 1 | incoming << 7, v & 1),
                    4 => (v.wrapping_shl(1), v >> 7),
                    6 => (v.rotate_left(4), 0),
                    7 => (v >> 1, v & 1),
                    _ => bail!("opcode CB {sub:02X} at {address:04X}"),
                };
                self.write_reg(r, result);
                self.f = flag(result == 0, ZERO) | flag(carry != 0, CARRY);
                if r == 6 { 16 } else { 8 }
            }
        } else if [7, 0x0F, 0x17, 0x1F].contains(&op) {
            let v = self.a;
            let (result, carry) = if op == 7 || op == 0x17 {
                let carry = v >> 7;
                let incoming = if op == 7 {
                    carry
                } else {
                    u8::from(self.f & CARRY != 0)
                };
                (v.wrapping_shl(1) | incoming, carry)
            } else {
                let carry = v & 1;
                let incoming = if op == 0x0F {
                    carry
                } else {
                    u8::from(self.f & CARRY != 0)
                };
                (v >> 1 | incoming << 7, carry)
            };
            self.a = result;
            self.f = flag(carry != 0, CARRY);
            4
        } else if op == 0x37 || op == 0x3F {
            self.f = (self.f & ZERO) | flag(op == 0x37 || self.f & CARRY == 0, CARRY);
            4
        } else if op == 0x2F {
            self.a ^= 255;
            self.f |= SUBTRACT | HALF_CARRY;
            4
        } else if (0x80..=0xBF).contains(&op) || op & 0xC7 == 0xC6 {
            let imm = op >= 0xC0;
            let rhs = if imm {
                self.fetch()
            } else {
                self.read8(op & 7)
            };
            self.alu((op >> 3) & 7, rhs);
            if imm || op & 7 == 6 { 8 } else { 4 }
        } else if op & 0xC7 == 4 || op & 0xC7 == 5 {
            let r = (op >> 3) & 7;
            let v = self.read8(r);
            let dec = op & 1 != 0;
            let result = if dec {
                v.wrapping_sub(1)
            } else {
                v.wrapping_add(1)
            };
            self.write_reg(r, result);
            self.f = (self.f & CARRY)
                | flag(dec, SUBTRACT)
                | flag(if dec { v & 15 == 0 } else { v & 15 == 15 }, HALF_CARRY)
                | flag(result == 0, ZERO);
            if r == 6 { 12 } else { 4 }
        } else if [0x18, 0x20, 0x28, 0x30, 0x38].contains(&op) {
            let off = self.fetch();
            let taken = op == 0x18 || self.condition((op >> 3) & 3);
            if taken {
                self.pc = self.pc.wrapping_add_signed(off as i8 as i16)
            }
            if taken { 12 } else { 8 }
        } else {
            bail!("opcode {op:02X} at {address:04X}")
        };
        self.cycles += u64::from(cost);
        Ok(cost)
    }
    pub fn run_until(&mut self, address: u16, max_instructions: usize) -> Result<u64> {
        let start = self.cycles;
        for _ in 0..max_instructions {
            if self.pc == address {
                return Ok(self.cycles - start);
            }
            self.step()?;
        }
        ensure!(
            self.pc == address,
            "did not reach {address:04X}; PC={:04X}",
            self.pc
        );
        Ok(self.cycles - start)
    }
}
fn flag(value: bool, mask: u8) -> u8 {
    if value { mask } else { 0 }
}
