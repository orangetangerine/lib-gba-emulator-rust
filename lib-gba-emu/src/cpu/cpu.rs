use std::ops::Add;
use super::instruction::*;
use super::register::*;

// https://rylev.github.io/DMG-01/public/book/appendix/instruction_guide/index.html
pub struct CPU {
    pc: u8,
    registers: Register,
}

impl CPU {
    pub fn execute(&mut self, ins: Instruction) {
        match ins {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {}
                    ArithmeticTarget::B => {}
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value)
                    }
                    ArithmeticTarget::D => {}
                    ArithmeticTarget::E => {}
                    ArithmeticTarget::H => {}
                    ArithmeticTarget::L => {}
                }
            }
        }
        println!(ins);
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, overflow) = self.registers.a.overflowing_add(value);
        // TODO set flags ...
        const LOW_NIBBLE_1: u8 = 0x0F;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (value & LOW_NIBBLE_1) + (self.registers.a & LOW_NIBBLE_1) > LOW_NIBBLE_1;

        return new_value;
    }
}