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
        self._sp = 0
        self.reset_metrics()

    def reset_metrics(self):
        self.cycles = 0
        self._stack_depth = 0
        self.peak_stack_bytes = 0
        self.written_addresses = set()

    @property
    def sp(self):
        return self._sp

    @sp.setter
    def sp(self, value):
        value &= 0xFFFF
        self._stack_depth += ((self._sp - value + 0x8000) & 0xFFFF) - 0x8000
        self.peak_stack_bytes = max(self.peak_stack_bytes, self._stack_depth)
        self._sp = value

    def write8(self, address, value):
        address &= 0xFFFF
        self.memory[address] = value & 0xFF
        self.written_addresses.add(address)

    @property
    def hl(self):
        return self.h << 8 | self.l

    @hl.setter
    def hl(self, value):
        self.h, self.l = (value >> 8) & 0xFF, value & 0xFF

    def read16(self, address):
        return self.memory[address & 0xFFFF] | self.memory[(address + 1) & 0xFFFF] << 8

    def write16(self, address, value):
        self.write8(address, value)
        self.write8(address + 1, value >> 8)

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
            self.write8(self.hl, value)
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

    def _fetch16(self):
        return self._fetch() | self._fetch() << 8

    def _push(self, value):
        self.sp -= 2
        self.write16(self.sp, value)

    def _pop(self):
        value = self.read16(self.sp)
        self.sp += 2
        return value

    def _condition(self, condition):
        flag = self.ZERO if condition < 2 else self.CARRY
        return bool(self.f & flag) == bool(condition & 1)

    def _alu(self, operation, rhs):
        lhs = self.a
        carry = int(bool(self.f & self.CARRY)) if operation in (1, 3) else 0
        if operation < 4 or operation == 7:
            subtract = operation in (2, 3, 7)
            result = lhs - rhs - carry if subtract else lhs + rhs + carry
            half = (
                (lhs & 15) - (rhs & 15) - carry < 0
                if subtract
                else ((lhs & 15) + (rhs & 15) + carry > 15)
            )
            self.f = (
                (self.SUBTRACT if subtract else 0)
                | (self.ZERO if result & 255 == 0 else 0)
                | (self.HALF_CARRY if half else 0)
                | (self.CARRY if result < 0 or result > 255 else 0)
            )
        else:
            result = {4: lhs & rhs, 5: lhs ^ rhs, 6: lhs | rhs}[operation]
            self.f = (self.ZERO if result == 0 else 0) | (
                self.HALF_CARRY if operation == 4 else 0
            )
        if operation != 7:
            self.a = result & 255

    def step(self):
        address = self.pc
        opcode = self._fetch()
        if opcode == 0x00:
            cost = 4
        elif opcode == 0xC9:
            self.pc = self._pop()
            cost = 16
        elif opcode in (0xC0, 0xC8, 0xD0, 0xD8):
            taken = self._condition((opcode >> 3) & 3)
            if taken:
                self.pc = self._pop()
            cost = 20 if taken else 8
        elif opcode in (0xCD, 0xC4, 0xCC, 0xD4, 0xDC):
            target = self._fetch16()
            taken = opcode == 0xCD or self._condition((opcode >> 3) & 3)
            if taken:
                self._push(self.pc)
                self.pc = target
            cost = 24 if taken else 12
        elif opcode & 0xC7 == 0xC7:
            self._push(self.pc)
            self.pc = opcode & 0x38
            cost = 16
        elif opcode & 0xCF == 0xC5:
            pair = (opcode >> 4) & 3
            value = (
                (self.a << 8 | self.f & 0xF0) if pair == 3 else self._read_pair(pair)
            )
            self._push(value)
            cost = 16
        elif opcode & 0xCF == 0xC1:
            pair = (opcode >> 4) & 3
            value = self._pop()
            if pair == 3:
                self.a, self.f = value >> 8, value & 0xF0
            else:
                self._write_pair(pair, value)
            cost = 12
        elif opcode in (0xC3, 0xC2, 0xCA, 0xD2, 0xDA):
            target = self._fetch16()
            taken = opcode == 0xC3 or self._condition((opcode >> 3) & 3)
            if taken:
                self.pc = target
            cost = 16 if taken else 12
        elif opcode == 0xE9:
            self.pc = self.hl
            cost = 4
        elif opcode in (0xEA, 0xFA):
            target = self._fetch16()
            if opcode == 0xEA:
                self.write8(target, self.a)
            else:
                self.a = self.memory[target]
            cost = 16
        elif opcode == 0xF9:
            self.sp = self.hl
            cost = 8
        elif opcode & 0xCF in (0x03, 0x0B):
            pair = (opcode >> 4) & 3
            self._write_pair(pair, self._read_pair(pair) + (-1 if opcode & 8 else 1))
            cost = 8
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
        elif opcode in (0xF8, 0xE8):
            offset = self._fetch()
            self.f = (self.HALF_CARRY if (self.sp & 15) + (offset & 15) > 15 else 0) | (
                self.CARRY if (self.sp & 255) + offset > 255 else 0
            )
            result = self.sp + (offset if offset < 128 else offset - 256)
            if opcode == 0xF8:
                self.hl = result
            else:
                self.sp = result
            cost = 12 if opcode == 0xF8 else 16
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
            if 0x40 <= sub < 0x80:
                self.f = (
                    (self.f & self.CARRY)
                    | self.HALF_CARRY
                    | (self.ZERO if value & (1 << ((sub >> 3) & 7)) == 0 else 0)
                )
                cost = 12 if register == 6 else 8
                self.cycles += cost
                return cost
            match sub >> 3:
                case 0:
                    result, carry = value << 1 | value >> 7, value >> 7
                case 1:
                    result, carry = value >> 1 | (value & 1) << 7, value & 1
                case 2:
                    result, carry = value << 1 | carry, value >> 7
                case 3:
                    result, carry = value >> 1 | carry << 7, value & 1
                case 4:
                    result, carry = value << 1, value >> 7
                case 6:
                    result, carry = value << 4 | value >> 4, 0
                case 7:
                    result, carry = value >> 1, value & 1
                case _:
                    raise NotImplementedError(f"opcode CB {sub:02X} at {address:04X}")
            self._write8(register, result)
            self.f = (self.ZERO if result & 255 == 0 else 0) | (
                self.CARRY if carry else 0
            )
            cost = 16 if register == 6 else 8
        elif opcode in (0x07, 0x0F, 0x17, 0x1F):
            value = self.a
            if opcode in (0x07, 0x17):
                carry = value >> 7
                incoming = carry if opcode == 0x07 else bool(self.f & self.CARRY)
                self.a = (value << 1 | incoming) & 255
            else:
                carry = value & 1
                incoming = carry if opcode == 0x0F else bool(self.f & self.CARRY)
                self.a = value >> 1 | incoming << 7
            self.f = self.CARRY if carry else 0
            cost = 4
        elif opcode in (0x37, 0x3F):
            carry = opcode == 0x37 or not self.f & self.CARRY
            self.f = (self.f & self.ZERO) | (self.CARRY if carry else 0)
            cost = 4
        elif opcode == 0x2F:
            self.a ^= 255
            self.f |= self.SUBTRACT | self.HALF_CARRY
            cost = 4
        elif 0x80 <= opcode <= 0xBF or opcode & 0xC7 == 0xC6:
            immediate = opcode >= 0xC0
            rhs = self._fetch() if immediate else self._read8(opcode & 7)
            self._alu((opcode >> 3) & 7, rhs)
            cost = 8 if immediate or opcode & 7 == 6 else 4
        elif opcode & 0xC7 == 0x04:
            register = (opcode >> 3) & 7
            value = self._read8(register)
            result = (value + 1) & 255
            self._write8(register, result)
            self.f = (
                (self.f & self.CARRY)
                | (self.HALF_CARRY if value & 15 == 15 else 0)
                | (self.ZERO if result == 0 else 0)
            )
            cost = 12 if register == 6 else 4
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
            taken = opcode == 0x18 or self._condition((opcode >> 3) & 3)
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
