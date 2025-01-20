use crate::memory::types::AddressingMode;

pub const ADC_WITH_IMMEDIATE: u8 = 0x69;

/// Opcodes for instruction set for 6402 processor
/// see: [6502 docs](http://www.6502.org/tutorials/6502opcodes.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// Add with Carry
    Adc(u8, AddressingMode),
    /// Bitwise And with accumulator
    And(u8, AddressingMode),
    /// Branch on PLus
    Bpl(u8),
    /// Branch on Minus
    Bmi(u8),
    /// Branch on Overflow Clear
    Bvc(u8),
    /// Branch on Overflow Set
    Bvs(u8),
    /// Branch on Carry Clear
    Bcc(u8),
    /// Branch on Carry Set
    Bcs(u8),
    /// Branch on Not Equal
    Bne(u8),
    /// Branch on Equal
    Beq(u8),
}

impl Opcode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            ADC_WITH_IMMEDIATE => Some(Opcode::Adc(value, AddressingMode::Immediate)),
            0x65 => Some(Opcode::Adc(value, AddressingMode::ZeroPage)),
            0x75 => Some(Opcode::Adc(value, AddressingMode::ZeroPageX)),
            0x6D => Some(Opcode::Adc(value, AddressingMode::Absolute)),
            0x7D => Some(Opcode::Adc(value, AddressingMode::AbsoluteX)),
            0x79 => Some(Opcode::Adc(value, AddressingMode::AbsoluteY)),
            0x61 => Some(Opcode::Adc(value, AddressingMode::IndirectX)),
            0x71 => Some(Opcode::Adc(value, AddressingMode::IndirectY)),
            0x29 => Some(Opcode::And(value, AddressingMode::Immediate)),
            0x25 => Some(Opcode::And(value, AddressingMode::ZeroPage)),
            0x35 => Some(Opcode::And(value, AddressingMode::ZeroPageX)),
            0x2D => Some(Opcode::And(value, AddressingMode::Absolute)),
            0x3D => Some(Opcode::And(value, AddressingMode::AbsoluteX)),
            0x39 => Some(Opcode::And(value, AddressingMode::AbsoluteY)),
            0x21 => Some(Opcode::And(value, AddressingMode::IndirectX)),
            0x31 => Some(Opcode::And(value, AddressingMode::IndirectY)),
            0x10 => Some(Opcode::Bpl(value)),
            0x30 => Some(Opcode::Bmi(value)),
            0x50 => Some(Opcode::Bvc(value)),
            0x70 => Some(Opcode::Bvs(value)),
            0x90 => Some(Opcode::Bcc(value)),
            0xB0 => Some(Opcode::Bcs(value)),
            0xD0 => Some(Opcode::Bne(value)),
            0xF0 => Some(Opcode::Beq(value)),
            _ => None,
        }
    }
}
