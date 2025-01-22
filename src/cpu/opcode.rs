use crate::memory::types::AddressingMode;

pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6D;
pub const ADC_ABSOLUTE_X: u8 = 0x7D;
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
pub const ADC_INDIRECT_X: u8 = 0x61;
pub const ADC_INDIRECT_Y: u8 = 0x71;
const SBC_IMMEDIATE: u8 = 0xE9;
const SBC_ZERO_PAGE: u8 = 0xE5;
const SBC_ZERO_PAGE_X: u8 = 0xF5;
const SBC_ABSOLUTE: u8 = 0xED;
const SBC_ABSOLUTE_X: u8 = 0xFD;
const SBC_ABSOLUTE_Y: u8 = 0xF9;
const SBC_INDIRECT_X: u8 = 0xE1;
const SBC_INDIRECT_Y: u8 = 0xF1;
const CMP_IMMEDIATE: u8 = 0xC9;
const CMP_ZERO_PAGE: u8 = 0xC5;
const CMP_ZERO_PAGE_X: u8 = 0xD5;
const CMP_ABSOLUTE: u8 = 0xCD;
const CMP_ABSOLUTE_X: u8 = 0xDD;
const CMP_ABSOLUTE_Y: u8 = 0xD9;
const CMP_INDIRECT_X: u8 = 0xC1;
const CMP_INDIRECT_Y: u8 = 0xD1;
const CPX_IMMEDIATE: u8 = 0xE0;
const CPX_ZERO_PAGE: u8 = 0xE4;
const CPX_ABSOLUTE: u8 = 0xEC;
const CPY_IMMEDIATE: u8 = 0xC0;
const CPY_ZERO_PAGE: u8 = 0xC4;
const CPY_ABSOLUTE: u8 = 0xCC;
const AND_IMMEDIATE: u8 = 0x29;
const AND_ZERO_PAGE: u8 = 0x25;
const AND_ZERO_PAGE_X: u8 = 0x35;
const AND_ABSOLUTE: u8 = 0x2D;
const AND_ABSOLUTE_X: u8 = 0x3D;
const AND_ABSOLUTE_Y: u8 = 0x39;
const AND_INDIRECT_X: u8 = 0x21;
const AND_INDIRECT_Y: u8 = 0x31;
const ORA_IMMEDIATE: u8 = 0x09;
const ORA_ZERO_PAGE: u8 = 0x05;
const ORA_ZERO_PAGE_X: u8 = 0x15;
const ORA_ABSOLUTE: u8 = 0x0D;
const ORA_ABSOLUTE_X: u8 = 0x1D;
const ORA_ABSOLUTE_Y: u8 = 0x19;
const ORA_INDIRECT_X: u8 = 0x01;
const ORA_INDIRECT_Y: u8 = 0x11;
const EOR_IMMEDIATE: u8 = 0x49;
const EOR_ZERO_PAGE: u8 = 0x45;
const EOR_ZERO_PAGE_X: u8 = 0x55;
const EOR_ABSOLUTE: u8 = 0x4D;
const EOR_ABSOLUTE_X: u8 = 0x5D;
const EOR_ABSOLUTE_Y: u8 = 0x59;
const EOR_INDIRECT_X: u8 = 0x41;
const EOR_INDIRECT_Y: u8 = 0x51;
const BIT_ZERO_PAGE: u8 = 0x24;
const BIT_ABSOLUTE: u8 = 0x2C;
const LDA_IMMEDIATE: u8 = 0xA9;
const LDA_ZERO_PAGE: u8 = 0xA5;
const LDA_ZERO_PAGE_X: u8 = 0xB5;
const LDA_ABSOLUTE: u8 = 0xAD;
const LDA_ABSOLUTE_X: u8 = 0xBD;
const LDA_ABSOLUTE_Y: u8 = 0xB9;
const LDA_INDIRECT_X: u8 = 0xA1;
const LDA_INDIRECT_Y: u8 = 0xB1;
const LDX_IMMEDIATE: u8 = 0xA2;
const LDX_ZERO_PAGE: u8 = 0xA6;
const LDX_ZERO_PAGE_Y: u8 = 0xB6;
const LDX_ABSOLUTE: u8 = 0xAE;
const LDX_ABSOLUTE_Y: u8 = 0xBE;
const LDY_IMMEDIATE: u8 = 0xA0;
const LDY_ZERO_PAGE: u8 = 0xA4;
const LDY_ZERO_PAGE_X: u8 = 0xB4;
const LDY_ABSOLUTE: u8 = 0xAC;
const LDY_ABSOLUTE_X: u8 = 0xBC;
const STA_ZERO_PAGE: u8 = 0x85;
const STA_ZERO_PAGE_X: u8 = 0x95;
const STA_ABSOLUTE: u8 = 0x8D;
const STA_ABSOLUTE_X: u8 = 0x9D;
const STA_ABSOLUTE_Y: u8 = 0x99;
const STA_INDIRECT_X: u8 = 0x81;
const STA_INDIRECT_Y: u8 = 0x91;
const STX_ZERO_PAGE: u8 = 0x86;
const STX_ZERO_PAGE_Y: u8 = 0x96;
const STX_ABSOLUTE: u8 = 0x8E;
const STY_ZERO_PAGE: u8 = 0x84;
const STY_ZERO_PAGE_X: u8 = 0x94;
const STY_ABSOLUTE: u8 = 0x8C;
const CLC: u8 = 0x18;
const CLD: u8 = 0xD8;
const CLI: u8 = 0x58;
const CLV: u8 = 0xB8;
const SEC: u8 = 0x38;
const SED: u8 = 0xF8;
const SEI: u8 = 0x78;
const BCC: u8 = 0x90;
const BCS: u8 = 0xB0;
const BEQ: u8 = 0xF0;
const BMI: u8 = 0x30;
const BNE: u8 = 0xD0;
const BPL: u8 = 0x10;
const BVC: u8 = 0x50;
const BVS: u8 = 0x70;
const TAX: u8 = 0xAA;
const TAY: u8 = 0xA8;
const TXA: u8 = 0x8A;
const TYA: u8 = 0x98;
const TSX: u8 = 0xBA;
const TXS: u8 = 0x9A;
const INC_ZERO_PAGE: u8 = 0xE6;
const INC_ZERO_PAGE_X: u8 = 0xF6;
const INC_ABSOLUTE: u8 = 0xEE;
const INC_ABSOLUTE_X: u8 = 0xFE;
const INX: u8 = 0xE8;
const INY: u8 = 0xC8;
const DEC_ZERO_PAGE: u8 = 0xC6;
const DEC_ZERO_PAGE_X: u8 = 0xD6;
const DEC_ABSOLUTE: u8 = 0xCE;
const DEC_ABSOLUTE_X: u8 = 0xDE;
const DEX: u8 = 0xCA;
const DEY: u8 = 0x88;

/// Opcodes of instruction set for 6502 processor
/// see: [6502 docs](http://www.6502.org/tutorials/6502opcodes.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// Add with Carry
    Adc(u8, AddressingMode),
    /// Subtract with Carry
    Sbc(u8, AddressingMode),
    /// Compare with Accumulator
    Cmp(u8, AddressingMode),
    // Compare with X Register
    Cpx(u8, AddressingMode),
    /// Compare with Y Register
    Cpy(u8, AddressingMode),
    /// Bitwise And with accumulator
    And(u8, AddressingMode),
    /// Bitwise Or with accumulator
    Ora(u8, AddressingMode),
    /// Bitwise Exclusive Or with accumulator
    Eor(u8, AddressingMode),
    /// Bit Test
    Bit(u8, AddressingMode),
    /// Load Accumulator
    Lda(u8, AddressingMode),
    /// Load X Register
    Ldx(u8, AddressingMode),
    /// Load Y Register
    Ldy(u8, AddressingMode),
    /// Store Accumulator
    Sta(u8, AddressingMode),
    /// Store X Register
    Stx(u8, AddressingMode),
    /// Store Y Register
    Sty(u8, AddressingMode),
    /// Clear Carry Flag
    Clc,
    /// Clear Decimal Mode
    Cld,
    /// Clear Interrupt Disable
    Cli,
    /// Clear Overflow Flag
    Clv,
    /// Set Carry Flag
    Sec,
    /// Set Decimal Mode
    Sed,
    /// Set Interrupt Disable
    Sei,
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
    /// Transfer Accumulator to X
    Tax(u8),
    /// Transfer Accumulator to Y
    Tay(u8),
    /// Transfer X to Accumulator
    Txa(u8),
    /// Transfer Y to Accumulator
    Tya(u8),
    /// Transfer Stack Pointer to X
    Tsx(u8),
    /// Transfer X to Stack Pointer
    Txs(u8),
    /// Increment Memory
    Inc(u8, AddressingMode),
    /// Increment X Register
    Inx(u8),
    /// Increment Y Register
    Iny(u8),
    /// Decrement Memory
    Dec(u8, AddressingMode),
    /// Decrement X Register
    Dex(u8),
    /// Decrement Y Register
    Dey(u8),
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
            CPX_IMMEDIATE => Some(Opcode::Cpx(value, AddressingMode::Immediate)),
            CPX_ZERO_PAGE => Some(Opcode::Cpx(value, AddressingMode::ZeroPage)),
            CPX_ABSOLUTE => Some(Opcode::Cpx(value, AddressingMode::Absolute)),
            CPY_IMMEDIATE => Some(Opcode::Cpy(value, AddressingMode::Immediate)),
            CPY_ZERO_PAGE => Some(Opcode::Cpy(value, AddressingMode::ZeroPage)),
            CPY_ABSOLUTE => Some(Opcode::Cpy(value, AddressingMode::Absolute)),
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
            LDA_IMMEDIATE => Some(Opcode::Lda(value, AddressingMode::Immediate)),
            LDA_ZERO_PAGE => Some(Opcode::Lda(value, AddressingMode::ZeroPage)),
            LDA_ZERO_PAGE_X => Some(Opcode::Lda(value, AddressingMode::ZeroPageX)),
            LDA_ABSOLUTE => Some(Opcode::Lda(value, AddressingMode::Absolute)),
            LDA_ABSOLUTE_X => Some(Opcode::Lda(value, AddressingMode::AbsoluteX)),
            LDA_ABSOLUTE_Y => Some(Opcode::Lda(value, AddressingMode::AbsoluteY)),
            LDA_INDIRECT_X => Some(Opcode::Lda(value, AddressingMode::IndirectX)),
            LDA_INDIRECT_Y => Some(Opcode::Lda(value, AddressingMode::IndirectY)),
            LDX_IMMEDIATE => Some(Opcode::Ldx(value, AddressingMode::Immediate)),
            LDX_ZERO_PAGE => Some(Opcode::Ldx(value, AddressingMode::ZeroPage)),
            LDX_ZERO_PAGE_Y => Some(Opcode::Ldx(value, AddressingMode::ZeroPageY)),
            LDX_ABSOLUTE => Some(Opcode::Ldx(value, AddressingMode::Absolute)),
            LDX_ABSOLUTE_Y => Some(Opcode::Ldx(value, AddressingMode::AbsoluteY)),
            LDY_IMMEDIATE => Some(Opcode::Ldy(value, AddressingMode::Immediate)),
            LDY_ZERO_PAGE => Some(Opcode::Ldy(value, AddressingMode::ZeroPage)),
            LDY_ZERO_PAGE_X => Some(Opcode::Ldy(value, AddressingMode::ZeroPageX)),
            LDY_ABSOLUTE => Some(Opcode::Ldy(value, AddressingMode::Absolute)),
            LDY_ABSOLUTE_X => Some(Opcode::Ldy(value, AddressingMode::AbsoluteX)),
            STA_ZERO_PAGE => Some(Opcode::Sta(value, AddressingMode::ZeroPage)),
            STA_ZERO_PAGE_X => Some(Opcode::Sta(value, AddressingMode::ZeroPageX)),
            STA_ABSOLUTE => Some(Opcode::Sta(value, AddressingMode::Absolute)),
            STA_ABSOLUTE_X => Some(Opcode::Sta(value, AddressingMode::AbsoluteX)),
            STA_ABSOLUTE_Y => Some(Opcode::Sta(value, AddressingMode::AbsoluteY)),
            STA_INDIRECT_X => Some(Opcode::Sta(value, AddressingMode::IndirectX)),
            STA_INDIRECT_Y => Some(Opcode::Sta(value, AddressingMode::IndirectY)),
            STX_ZERO_PAGE => Some(Opcode::Stx(value, AddressingMode::ZeroPage)),
            STX_ZERO_PAGE_Y => Some(Opcode::Stx(value, AddressingMode::ZeroPageY)),
            STX_ABSOLUTE => Some(Opcode::Stx(value, AddressingMode::Absolute)),
            STY_ZERO_PAGE => Some(Opcode::Sty(value, AddressingMode::ZeroPage)),
            STY_ZERO_PAGE_X => Some(Opcode::Sty(value, AddressingMode::ZeroPageX)),
            STY_ABSOLUTE => Some(Opcode::Sty(value, AddressingMode::Absolute)),
            CLC => Some(Opcode::Clc),
            CLD => Some(Opcode::Cld),
            CLI => Some(Opcode::Cli),
            CLV => Some(Opcode::Clv),
            SEC => Some(Opcode::Sec),
            SED => Some(Opcode::Sed),
            SEI => Some(Opcode::Sei),
            BCC => Some(Opcode::Bcc(value)),
            BCS => Some(Opcode::Bcs(value)),
            BEQ => Some(Opcode::Beq(value)),
            BMI => Some(Opcode::Bmi(value)),
            BNE => Some(Opcode::Bne(value)),
            BPL => Some(Opcode::Bpl(value)),
            BVC => Some(Opcode::Bvc(value)),
            BVS => Some(Opcode::Bvs(value)),
            TAX => Some(Opcode::Tax(value)),
            TAY => Some(Opcode::Tay(value)),
            TXA => Some(Opcode::Txa(value)),
            TYA => Some(Opcode::Tya(value)),
            TSX => Some(Opcode::Tsx(value)),
            TXS => Some(Opcode::Txs(value)),
            INC_ZERO_PAGE => Some(Opcode::Inc(value, AddressingMode::ZeroPage)),
            INC_ZERO_PAGE_X => Some(Opcode::Inc(value, AddressingMode::ZeroPageX)),
            INC_ABSOLUTE => Some(Opcode::Inc(value, AddressingMode::Absolute)),
            INC_ABSOLUTE_X => Some(Opcode::Inc(value, AddressingMode::AbsoluteX)),
            INX => Some(Opcode::Inx(value)),
            INY => Some(Opcode::Iny(value)),
            DEC_ZERO_PAGE => Some(Opcode::Dec(value, AddressingMode::ZeroPage)),
            DEC_ZERO_PAGE_X => Some(Opcode::Dec(value, AddressingMode::ZeroPageX)),
            DEC_ABSOLUTE => Some(Opcode::Dec(value, AddressingMode::Absolute)),
            DEC_ABSOLUTE_X => Some(Opcode::Dec(value, AddressingMode::AbsoluteX)),
            DEX => Some(Opcode::Dex(value)),
            DEY => Some(Opcode::Dey(value)),
            _ => None,
        }
    }
}
