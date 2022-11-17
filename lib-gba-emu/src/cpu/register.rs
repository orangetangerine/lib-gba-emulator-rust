// GameBoy's CPU is Sharp LR35902, 8-bit
// https://rylev.github.io/DMG-01/public/book/cpu/registers.html
pub struct Register {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister, // "flags" register, see #FlagsRegister
    pub h: u8,
    pub l: u8,
}

impl Register {
    // allow 8-bit CPU a read 16-bits data (read 2 registers)
    // af(register a and f), bc(register b and c), de(register d and e), hl(register h and l)
    // fn get_af(&self) -> u16 {
    //     return (self.a as u16) << 8 | (self.f as u16);
    // }
    //
    // fn set_af(&mut self, v: u16) {
    //     self.a = ((v & 0xFF00) >> 8) as u8;
    //     self.f = (v & 0x00FF) as u8;
    // }

    fn get_bc(&self) -> u16 {
        return (self.b as u16) << 8 | (self.c as u16);
    }

    fn set_bc(&mut self, v: u16) {
        self.b = ((v & 0xFF00) >> 8) as u8;
        self.c = (v & 0x00FF) as u8;
    }

    fn get_de(&self) -> u16 {
        return (self.d as u16) << 8 | (self.e as u16);
    }

    fn set_de(&mut self, v: u16) {
        self.d = ((v & 0xFF00) >> 8) as u8;
        self.e = (v & 0x00FF) as u8;
    }
}

// "Flags" Register
// Bit-7: zero, Bit-6: "subtraction", Bit-5: "half carry", Bit-4: "carry"
struct FlagsRegister {
    pub zero: bool, // set to true if the result of the operation is equal to 0.
    pub subtract: bool, // set to true if the operation was a subtraction.
    pub half_carry: bool, // set to true if there is an overflow from the lower nibble (a.k.a the lower four bits) to the upper nibble (a.k.a the upper four bits).
    pub carry: bool, // set to true if the operation resulted in an overflow.
}

const BIT_POS_FLAGS_ZERO: u8 = 7;
const BIT_POS_FLAGS_SUBTRACT: u8 = 6;
const BIT_POS_FLAGS_HALF_CARRY: u8 = 5;
const BIT_POS_FLAGS_CARRY: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero {1} else {0}) << BIT_POS_FLAGS_ZERO |
        (if flag.subtract {1} else {0}) << BIT_POS_FLAGS_SUBTRACT |
        (if flag.half_carry {1} else {0}) << BIT_POS_FLAGS_HALF_CARRY |
        (if flag.carry {1} else {0}) << BIT_POS_FLAGS_CARRY
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(v: u8) -> FlagsRegister {
        let zero = ((v >> BIT_POS_FLAGS_ZERO) & 0b1) != 0;
        let subtract = ((v >> BIT_POS_FLAGS_SUBTRACT) & 0b1) != 0;
        let half_carry = ((v >> BIT_POS_FLAGS_HALF_CARRY) & 0b1) != 0;
        let carry = ((v >> BIT_POS_FLAGS_CARRY) & 0b1) != 0;
        return FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        };
    }
}