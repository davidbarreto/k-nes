use crate::memory::types::AddressingMode;

pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6D;
pub const ADC_ABSOLUTE_X: u8 = 0x7D;
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
pub const ADC_INDIRECT_X: u8 = 0x61;
pub const ADC_INDIRECT_Y: u8 = 0x71;

pub const SBC_IMMEDIATE: u8 = 0xE9;
pub const SBC_ZERO_PAGE: u8 = 0xE5;
pub const SBC_ZERO_PAGE_X: u8 = 0xF5;
pub const SBC_ABSOLUTE: u8 = 0xED;
pub const SBC_ABSOLUTE_X: u8 = 0xFD;
pub const SBC_ABSOLUTE_Y: u8 = 0xF9;
pub const SBC_INDIRECT_X: u8 = 0xE1;
pub const SBC_INDIRECT_Y: u8 = 0xF1;

pub const CMP_IMMEDIATE: u8 = 0xC9;
pub const CMP_ZERO_PAGE: u8 = 0xC5;
pub const CMP_ZERO_PAGE_X: u8 = 0xD5;
pub const CMP_ABSOLUTE: u8 = 0xCD;
pub const CMP_ABSOLUTE_X: u8 = 0xDD;
pub const CMP_ABSOLUTE_Y: u8 = 0xD9;
pub const CMP_INDIRECT_X: u8 = 0xC1;
pub const CMP_INDIRECT_Y: u8 = 0xD1;

pub const AND_IMMEDIATE: u8 = 0x29;
pub const AND_ZERO_PAGE: u8 = 0x25;
pub const AND_ZERO_PAGE_X: u8 = 0x35;
pub const AND_ABSOLUTE: u8 = 0x2D;
pub const AND_ABSOLUTE_X: u8 = 0x3D;
pub const AND_ABSOLUTE_Y: u8 = 0x39;
pub const AND_INDIRECT_X: u8 = 0x21;
pub const AND_INDIRECT_Y: u8 = 0x31;

pub const ORA_IMMEDIATE: u8 = 0x09;
pub const ORA_ZERO_PAGE: u8 = 0x05;
pub const ORA_ZERO_PAGE_X: u8 = 0x15;
pub const ORA_ABSOLUTE: u8 = 0x0D;
pub const ORA_ABSOLUTE_X: u8 = 0x1D;
pub const ORA_ABSOLUTE_Y: u8 = 0x19;
pub const ORA_INDIRECT_X: u8 = 0x01;
pub const ORA_INDIRECT_Y: u8 = 0x11;

pub const EOR_IMMEDIATE: u8 = 0x49;
pub const EOR_ZERO_PAGE: u8 = 0x45;
pub const EOR_ZERO_PAGE_X: u8 = 0x55;
pub const EOR_ABSOLUTE: u8 = 0x4D;
pub const EOR_ABSOLUTE_X: u8 = 0x5D;
pub const EOR_ABSOLUTE_Y: u8 = 0x59;
pub const EOR_INDIRECT_X: u8 = 0x41;
pub const EOR_INDIRECT_Y: u8 = 0x51;

pub const BIT_ZERO_PAGE: u8 = 0x24;
pub const BIT_ABSOLUTE: u8 = 0x2C;

pub const BPL: u8 = 0x10;
pub const BMI: u8 = 0x30;
pub const BVC: u8 = 0x50;
pub const BVS: u8 = 0x70;
pub const BCC: u8 = 0x90;
pub const BCS: u8 = 0xB0;
pub const BNE: u8 = 0xD0;
pub const BEQ: u8 = 0xF0;

/// Opcodes of instruction set for 6502 processor
/// see: [6502 docs](http://www.6502.org/tutorials/6502opcodes.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// Add with Carry
    Adc(u8, AddressingMode),
    /// Subtract with Carry
    Sbc(u8, AddressingMode),
    /// Compare
    Cmp(u8, AddressingMode),
    /// Bitwise And with accumulator
    And(u8, AddressingMode),
    /// Bitwise Or with accumulator
    Ora(u8, AddressingMode),
    /// Bitwise Exclusive Or with accumulator
    Eor(u8, AddressingMode),
    /// Bit Test
    Bit(u8, AddressingMode),
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
            ADC_IMMEDIATE => Some(Opcode::Adc(value, AddressingMode::Immediate)),
            ADC_ZERO_PAGE => Some(Opcode::Adc(value, AddressingMode::ZeroPage)),
            ADC_ZERO_PAGE_X => Some(Opcode::Adc(value, AddressingMode::ZeroPageX)),
            ADC_ABSOLUTE => Some(Opcode::Adc(value, AddressingMode::Absolute)),
            ADC_ABSOLUTE_X => Some(Opcode::Adc(value, AddressingMode::AbsoluteX)),
            ADC_ABSOLUTE_Y => Some(Opcode::Adc(value, AddressingMode::AbsoluteY)),
            ADC_INDIRECT_X => Some(Opcode::Adc(value, AddressingMode::IndirectX)),
            ADC_INDIRECT_Y => Some(Opcode::Adc(value, AddressingMode::IndirectY)),
            SBC_IMMEDIATE => Some(Opcode::Sbc(value, AddressingMode::Immediate)),
            SBC_ZERO_PAGE => Some(Opcode::Sbc(value, AddressingMode::ZeroPage)),
            SBC_ZERO_PAGE_X => Some(Opcode::Sbc(value, AddressingMode::ZeroPageX)),
            SBC_ABSOLUTE => Some(Opcode::Sbc(value, AddressingMode::Absolute)),
            SBC_ABSOLUTE_X => Some(Opcode::Sbc(value, AddressingMode::AbsoluteX)),
            SBC_ABSOLUTE_Y => Some(Opcode::Sbc(value, AddressingMode::AbsoluteY)),
            SBC_INDIRECT_X => Some(Opcode::Sbc(value, AddressingMode::IndirectX)),
            SBC_INDIRECT_Y => Some(Opcode::Sbc(value, AddressingMode::IndirectY)),
            CMP_IMMEDIATE => Some(Opcode::Cmp(value, AddressingMode::Immediate)),
            CMP_ZERO_PAGE => Some(Opcode::Cmp(value, AddressingMode::ZeroPage)),
            CMP_ZERO_PAGE_X => Some(Opcode::Cmp(value, AddressingMode::ZeroPageX)),
            CMP_ABSOLUTE => Some(Opcode::Cmp(value, AddressingMode::Absolute)),
            CMP_ABSOLUTE_X => Some(Opcode::Cmp(value, AddressingMode::AbsoluteX)),
            CMP_ABSOLUTE_Y => Some(Opcode::Cmp(value, AddressingMode::AbsoluteY)),
            CMP_INDIRECT_X => Some(Opcode::Cmp(value, AddressingMode::IndirectX)),
            CMP_INDIRECT_Y => Some(Opcode::Cmp(value, AddressingMode::IndirectY)),
            AND_IMMEDIATE => Some(Opcode::And(value, AddressingMode::Immediate)),
            AND_ZERO_PAGE => Some(Opcode::And(value, AddressingMode::ZeroPage)),
            AND_ZERO_PAGE_X => Some(Opcode::And(value, AddressingMode::ZeroPageX)),
            AND_ABSOLUTE => Some(Opcode::And(value, AddressingMode::Absolute)),
            AND_ABSOLUTE_X => Some(Opcode::And(value, AddressingMode::AbsoluteX)),
            AND_ABSOLUTE_Y => Some(Opcode::And(value, AddressingMode::AbsoluteY)),
            AND_INDIRECT_X => Some(Opcode::And(value, AddressingMode::IndirectX)),
            AND_INDIRECT_Y => Some(Opcode::And(value, AddressingMode::IndirectY)),
            ORA_IMMEDIATE => Some(Opcode::Ora(value, AddressingMode::Immediate)),
            ORA_ZERO_PAGE => Some(Opcode::Ora(value, AddressingMode::ZeroPage)),
            ORA_ZERO_PAGE_X => Some(Opcode::Ora(value, AddressingMode::ZeroPageX)),
            ORA_ABSOLUTE => Some(Opcode::Ora(value, AddressingMode::Absolute)),
            ORA_ABSOLUTE_X => Some(Opcode::Ora(value, AddressingMode::AbsoluteX)),
            ORA_ABSOLUTE_Y => Some(Opcode::Ora(value, AddressingMode::AbsoluteY)),
            ORA_INDIRECT_X => Some(Opcode::Ora(value, AddressingMode::IndirectX)),
            ORA_INDIRECT_Y => Some(Opcode::Ora(value, AddressingMode::IndirectY)),
            EOR_IMMEDIATE => Some(Opcode::Eor(value, AddressingMode::Immediate)),
            EOR_ZERO_PAGE => Some(Opcode::Eor(value, AddressingMode::ZeroPage)),
            EOR_ZERO_PAGE_X => Some(Opcode::Eor(value, AddressingMode::ZeroPageX)),
            EOR_ABSOLUTE => Some(Opcode::Eor(value, AddressingMode::Absolute)),
            EOR_ABSOLUTE_X => Some(Opcode::Eor(value, AddressingMode::AbsoluteX)),
            EOR_ABSOLUTE_Y => Some(Opcode::Eor(value, AddressingMode::AbsoluteY)),
            EOR_INDIRECT_X => Some(Opcode::Eor(value, AddressingMode::IndirectX)),
            EOR_INDIRECT_Y => Some(Opcode::Eor(value, AddressingMode::IndirectY)),
            BIT_ZERO_PAGE => Some(Opcode::Bit(value, AddressingMode::ZeroPage)),
            BIT_ABSOLUTE => Some(Opcode::Bit(value, AddressingMode::Absolute)),
            BPL => Some(Opcode::Bpl(value)),
            BMI => Some(Opcode::Bmi(value)),
            BVC => Some(Opcode::Bvc(value)),
            BVS => Some(Opcode::Bvs(value)),
            BCC => Some(Opcode::Bcc(value)),
            BCS => Some(Opcode::Bcs(value)),
            BNE => Some(Opcode::Bne(value)),
            BEQ => Some(Opcode::Beq(value)),
            _ => None,
        }
    }
}
