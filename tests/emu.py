class Emulator:
    ZERO = 0x80
    SUBTRACT = 0x40
    HALF_CARRY = 0x20
    CARRY = 0x10
    _REGISTERS = ("b", "c", "d", "e", "h", "l", None, "a")

    def __init__(self, code=b"", *, pc=0):
        if len(code) > 65536:
            raise ValueError("code exceeds the 16-bit address space")
        self.memory = bytearray(65536)
        self.memory[: len(code)] = code
        self.a = self.b = self.c = self.d = self.e = self.h = self.l = self.f = 0
        self.pc = pc & 0xFFFF
        self.sp = 0
        self.cycles = 0

    @property
    def hl(self):
        return self.h << 8 | self.l

    @hl.setter
    def hl(self, value):
        self.h, self.l = (value >> 8) & 0xFF, value & 0xFF

    def read16(self, address):
        return self.memory[address & 0xFFFF] | self.memory[(address + 1) & 0xFFFF] << 8

    def write16(self, address, value):
        self.memory[address & 0xFFFF] = value & 0xFF
        self.memory[(address + 1) & 0xFFFF] = (value >> 8) & 0xFF

    def _fetch(self):
        value = self.memory[self.pc]
        self.pc = (self.pc + 1) & 0xFFFF
        return value

    def _read8(self, register):
        if register == 6:
            return self.memory[self.hl]
        return getattr(self, self._REGISTERS[register])

    def _write8(self, register, value):
        value &= 0xFF
        if register == 6:
            self.memory[self.hl] = value
        else:
            setattr(self, self._REGISTERS[register], value)

    def _read_pair(self, pair):
        if pair == 3:
            return self.sp
        return self._read8(pair * 2) << 8 | self._read8(pair * 2 + 1)

    def _write_pair(self, pair, value):
        if pair == 3:
            self.sp = value & 0xFFFF
        else:
            self._write8(pair * 2, value >> 8)
            self._write8(pair * 2 + 1, value)

    def step(self):
        address = self.pc
        opcode = self._fetch()
        if opcode == 0x00:
            cost = 4
        elif opcode == 0xC9:
            self.pc = self.read16(self.sp)
            self.sp = (self.sp + 2) & 0xFFFF
            cost = 16
        elif 0x40 <= opcode <= 0x7F and opcode != 0x76:
            dst, src = (opcode >> 3) & 7, opcode & 7
            self._write8(dst, self._read8(src))
            cost = 8 if 6 in (dst, src) else 4
        elif opcode & 0xC7 == 0x06:
            register = (opcode >> 3) & 7
            self._write8(register, self._fetch())
            cost = 12 if register == 6 else 8
        elif opcode & 0xCF == 0x01:
            value = self._fetch() | self._fetch() << 8
            self._write_pair((opcode >> 4) & 3, value)
            cost = 12
        elif opcode == 0xF8:
            offset = self._fetch()
            self.f = (self.HALF_CARRY if (self.sp & 15) + (offset & 15) > 15 else 0) | (
                self.CARRY if (self.sp & 255) + offset > 255 else 0
            )
            self.hl = self.sp + (offset if offset < 128 else offset - 256)
            cost = 12
        elif opcode & 0xCF == 0x09:
            lhs, rhs = self.hl, self._read_pair((opcode >> 4) & 3)
            self.hl = lhs + rhs
            self.f = (
                (self.f & self.ZERO)
                | (self.HALF_CARRY if (lhs & 0xFFF) + (rhs & 0xFFF) > 0xFFF else 0)
                | (self.CARRY if lhs + rhs > 0xFFFF else 0)
            )
            cost = 8
        elif opcode == 0xCB:
            sub = self._fetch()
            register = sub & 7
            value = self._read8(register)
            carry = bool(self.f & self.CARRY)
            match sub >> 3:
                case 2:
                    result, carry = value << 1 | carry, value >> 7
                case 3:
                    result, carry = value >> 1 | carry << 7, value & 1
                case 4:
                    result, carry = value << 1, value >> 7
                case 7:
                    result, carry = value >> 1, value & 1
                case _:
                    raise NotImplementedError(f"opcode CB {sub:02X} at {address:04X}")
            self._write8(register, result)
            self.f = (self.ZERO if result & 255 == 0 else 0) | (
                self.CARRY if carry else 0
            )
            cost = 16 if register == 6 else 8
        elif opcode == 0x17:
            value = self.a
            self.a = ((value << 1) | bool(self.f & self.CARRY)) & 0xFF
            self.f = self.CARRY if value & 0x80 else 0
            cost = 4
        elif 0xB0 <= opcode <= 0xB7:
            register = opcode & 7
            self.a |= self._read8(register)
            self.f = self.ZERO if self.a == 0 else 0
            cost = 8 if register == 6 else 4
        elif opcode & 0xC7 == 0x05:
            register = (opcode >> 3) & 7
            value = self._read8(register)
            result = (value - 1) & 0xFF
            self._write8(register, result)
            self.f = (
                (self.f & self.CARRY)
                | self.SUBTRACT
                | (self.HALF_CARRY if value & 15 == 0 else 0)
                | (self.ZERO if result == 0 else 0)
            )
            cost = 12 if register == 6 else 4
        elif opcode in (0x18, 0x20, 0x28, 0x30, 0x38):
            offset = self._fetch()
            taken = {
                0x18: True,
                0x20: not self.f & self.ZERO,
                0x28: bool(self.f & self.ZERO),
                0x30: not self.f & self.CARRY,
                0x38: bool(self.f & self.CARRY),
            }[opcode]
            if taken:
                self.pc = (
                    self.pc + (offset if offset < 128 else offset - 256)
                ) & 0xFFFF
            cost = 12 if taken else 8
        else:
            raise NotImplementedError(f"opcode {opcode:02X} at {address:04X}")
        self.cycles += cost
        return cost

    def run_until(self, address, *, max_instructions=1000):
        start = self.cycles
        for _ in range(max_instructions):
            if self.pc == address:
                return self.cycles - start
            self.step()
        if self.pc == address:
            return self.cycles - start
        raise TimeoutError(f"did not reach {address:04X}; PC={self.pc:04X}")
