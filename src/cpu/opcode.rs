use crate::memory::types::AddressingMode;

pub const BRK: u8 = 0x00;
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
const ASL_ACCUMULATOR: u8 = 0x0A;
const ASL_ZERO_PAGE: u8 = 0x06;
const ASL_ZERO_PAGE_X: u8 = 0x16;
const ASL_ABSOLUTE: u8 = 0x0E;
const ASL_ABSOLUTE_X: u8 = 0x1E;
const LSR_ACCUMULATOR: u8 = 0x4A;
const LSR_ZERO_PAGE: u8 = 0x46;
const LSR_ZERO_PAGE_X: u8 = 0x56;
const LSR_ABSOLUTE: u8 = 0x4E;
const LSR_ABSOLUTE_X: u8 = 0x5E;
const ROL_ACCUMULATOR: u8 = 0x2A;
const ROL_ZERO_PAGE: u8 = 0x26;
const ROL_ZERO_PAGE_X: u8 = 0x36;
const ROL_ABSOLUTE: u8 = 0x2E;
const ROL_ABSOLUTE_X: u8 = 0x3E;
const ROR_ACCUMULATOR: u8 = 0x6A;
const ROR_ZERO_PAGE: u8 = 0x66;
const ROR_ZERO_PAGE_X: u8 = 0x76;
const ROR_ABSOLUTE: u8 = 0x6E;
const ROR_ABSOLUTE_X: u8 = 0x7E;
const JMP_ABSOLUTE: u8 = 0x4C;
const JMP_INDIRECT: u8 = 0x6C;
const JSR_ABSOLUTE: u8 = 0x20;
const RTS: u8 = 0x60;
const NOP: u8 = 0xEA;
const PHA: u8 = 0x48;
const PHP: u8 = 0x08;
const PLA: u8 = 0x68;
const PLP: u8 = 0x28;
const RTI: u8 = 0x40;


/// Opcodes of instruction set for 6502 processor
///
/// Each variant of this enum represents a specific instruction in the 6502 instruction set.
/// - The `u8` parameter represents the opcode value;
/// - The `AddressingMode` parameter
/// represents the addressing mode used by the instruction
/// (how the instruction will extract it's data. E.g. from memory/registers, and how);
///
/// see: [6502 docs](http://www.6502.org/tutorials/6502opcodes.html)
/// 
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
    Clc(u8, AddressingMode),
    /// Clear Decimal Mode
    Cld(u8, AddressingMode),
    /// Clear Interrupt Disable
    Cli(u8, AddressingMode),
    /// Clear Overflow Flag
    Clv(u8, AddressingMode),
    /// Set Carry Flag
    Sec(u8, AddressingMode),
    /// Set Decimal Mode
    Sed(u8, AddressingMode),
    /// Set Interrupt Disable
    Sei(u8, AddressingMode),
    /// Branch on PLus
    Bpl(u8, AddressingMode),
    /// Branch on Minus
    Bmi(u8, AddressingMode),
    /// Branch on Overflow Clear
    Bvc(u8, AddressingMode),
    /// Branch on Overflow Set
    Bvs(u8, AddressingMode),
    /// Branch on Carry Clear
    Bcc(u8, AddressingMode),
    /// Branch on Carry Set
    Bcs(u8, AddressingMode),
    /// Branch on Not Equal
    Bne(u8, AddressingMode),
    /// Branch on Equal
    Beq(u8, AddressingMode),
    /// Transfer Accumulator to X
    Tax(u8, AddressingMode),
    /// Transfer Accumulator to Y
    Tay(u8, AddressingMode),
    /// Transfer X to Accumulator
    Txa(u8, AddressingMode),
    /// Transfer Y to Accumulator
    Tya(u8, AddressingMode),
    /// Transfer Stack Pointer to X
    Tsx(u8, AddressingMode),
    /// Transfer X to Stack Pointer
    Txs(u8, AddressingMode),
    /// Increment Memory
    Inc(u8, AddressingMode),
    /// Increment X Register
    Inx(u8, AddressingMode),
    /// Increment Y Register
    Iny(u8, AddressingMode),
    /// Decrement Memory
    Dec(u8, AddressingMode),
    /// Decrement X Register
    Dex(u8, AddressingMode),
    /// Decrement Y Register
    Dey(u8, AddressingMode),
    /// Arithmetic Shift Left
    Asl(u8, AddressingMode),
    /// Logical Shift Right
    Lsr(u8, AddressingMode),
    /// Rotate Left
    Rol(u8, AddressingMode),
    /// Rotate Right
    Ror(u8, AddressingMode),
    /// Jump to another location
    Jmp(u8, AddressingMode),
    /// Jump to Subroutine
    Jsr(u8, AddressingMode),
    /// Return from Subroutine
    Rts(u8, AddressingMode),
    /// Break - Force an Interrupt
    Brk(u8, AddressingMode),
    /// No Operation
    Nop(u8, AddressingMode),
    /// Push Accumulator on Stack
    Pha(u8, AddressingMode),
    /// Push processor status on Stack
    Php(u8, AddressingMode),
    /// Pull Accumulator from Stack
    Pla(u8, AddressingMode),
    /// Pull processor status from Stack
    Plp(u8, AddressingMode),
    /// Return from Interrupt
    Rti(u8, AddressingMode)
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
            CLC => Some(Opcode::Clc(value, AddressingMode::Implicit)),
            CLD => Some(Opcode::Cld(value, AddressingMode::Implicit)),
            CLI => Some(Opcode::Cli(value, AddressingMode::Implicit)),
            CLV => Some(Opcode::Clv(value, AddressingMode::Implicit)),
            SEC => Some(Opcode::Sec(value, AddressingMode::Implicit)),
            SED => Some(Opcode::Sed(value, AddressingMode::Implicit)),
            SEI => Some(Opcode::Sei(value, AddressingMode::Implicit)),
            BCC => Some(Opcode::Bcc(value, AddressingMode::Relative)),
            BCS => Some(Opcode::Bcs(value, AddressingMode::Relative)),
            BEQ => Some(Opcode::Beq(value, AddressingMode::Relative)),
            BMI => Some(Opcode::Bmi(value, AddressingMode::Relative)),
            BNE => Some(Opcode::Bne(value, AddressingMode::Relative)),
            BPL => Some(Opcode::Bpl(value, AddressingMode::Relative)),
            BVC => Some(Opcode::Bvc(value, AddressingMode::Relative)),
            BVS => Some(Opcode::Bvs(value, AddressingMode::Relative)),
            TAX => Some(Opcode::Tax(value, AddressingMode::Implicit)),
            TAY => Some(Opcode::Tay(value, AddressingMode::Implicit)),
            TXA => Some(Opcode::Txa(value, AddressingMode::Implicit)),
            TYA => Some(Opcode::Tya(value, AddressingMode::Implicit)),
            TSX => Some(Opcode::Tsx(value, AddressingMode::Implicit)),
            TXS => Some(Opcode::Txs(value, AddressingMode::Implicit)),
            INC_ZERO_PAGE => Some(Opcode::Inc(value, AddressingMode::ZeroPage)),
            INC_ZERO_PAGE_X => Some(Opcode::Inc(value, AddressingMode::ZeroPageX)),
            INC_ABSOLUTE => Some(Opcode::Inc(value, AddressingMode::Absolute)),
            INC_ABSOLUTE_X => Some(Opcode::Inc(value, AddressingMode::AbsoluteX)),
            INX => Some(Opcode::Inx(value, AddressingMode::Implicit)),
            INY => Some(Opcode::Iny(value, AddressingMode::Implicit)),
            DEC_ZERO_PAGE => Some(Opcode::Dec(value, AddressingMode::ZeroPage)),
            DEC_ZERO_PAGE_X => Some(Opcode::Dec(value, AddressingMode::ZeroPageX)),
            DEC_ABSOLUTE => Some(Opcode::Dec(value, AddressingMode::Absolute)),
            DEC_ABSOLUTE_X => Some(Opcode::Dec(value, AddressingMode::AbsoluteX)),
            DEX => Some(Opcode::Dex(value, AddressingMode::Implicit)),
            DEY => Some(Opcode::Dey(value, AddressingMode::Implicit)),
            ASL_ACCUMULATOR => Some(Opcode::Asl(value, AddressingMode::Accumulator)),
            ASL_ZERO_PAGE => Some(Opcode::Asl(value, AddressingMode::ZeroPage)),
            ASL_ZERO_PAGE_X => Some(Opcode::Asl(value, AddressingMode::ZeroPageX)),
            ASL_ABSOLUTE => Some(Opcode::Asl(value, AddressingMode::Absolute)),
            ASL_ABSOLUTE_X => Some(Opcode::Asl(value, AddressingMode::AbsoluteX)),
            LSR_ACCUMULATOR => Some(Opcode::Asl(value, AddressingMode::Accumulator)),
            LSR_ZERO_PAGE => Some(Opcode::Asl(value, AddressingMode::ZeroPage)),
            LSR_ZERO_PAGE_X => Some(Opcode::Asl(value, AddressingMode::ZeroPageX)),
            LSR_ABSOLUTE => Some(Opcode::Asl(value, AddressingMode::Absolute)),
            LSR_ABSOLUTE_X => Some(Opcode::Asl(value, AddressingMode::AbsoluteX)),
            ROL_ACCUMULATOR => Some(Opcode::Asl(value, AddressingMode::Accumulator)),
            ROL_ZERO_PAGE => Some(Opcode::Asl(value, AddressingMode::ZeroPage)),
            ROL_ZERO_PAGE_X => Some(Opcode::Asl(value, AddressingMode::ZeroPageX)),
            ROL_ABSOLUTE => Some(Opcode::Asl(value, AddressingMode::Absolute)),
            ROL_ABSOLUTE_X => Some(Opcode::Asl(value, AddressingMode::AbsoluteX)),
            ROR_ACCUMULATOR => Some(Opcode::Asl(value, AddressingMode::Accumulator)),
            ROR_ZERO_PAGE => Some(Opcode::Asl(value, AddressingMode::ZeroPage)),
            ROR_ZERO_PAGE_X => Some(Opcode::Asl(value, AddressingMode::ZeroPageX)),
            ROR_ABSOLUTE => Some(Opcode::Asl(value, AddressingMode::Absolute)),
            ROR_ABSOLUTE_X => Some(Opcode::Asl(value, AddressingMode::AbsoluteX)),
            JMP_ABSOLUTE => Some(Opcode::Jmp(value, AddressingMode::Absolute)),
            JMP_INDIRECT => Some(Opcode::Jmp(value, AddressingMode::Indirect)),
            JSR_ABSOLUTE => Some(Opcode::Jsr(value, AddressingMode::Absolute)),
            RTS => Some(Opcode::Rts(value, AddressingMode::Implicit)),
            PHA => Some(Opcode::Pha(value, AddressingMode::Implicit)),
            PHP => Some(Opcode::Php(value, AddressingMode::Implicit)),
            PLA => Some(Opcode::Pla(value, AddressingMode::Implicit)),
            PLP => Some(Opcode::Plp(value, AddressingMode::Implicit)),
            BRK => Some(Opcode::Brk(value, AddressingMode::Implicit)),
            RTI => Some(Opcode::Rti(value, AddressingMode::Implicit)),
            NOP => Some(Opcode::Nop(value, AddressingMode::Implicit)),
            _ => None,
        }
    }

    pub fn is_jump_instruction(&self) -> bool {
        match self {
            Opcode::Jmp(_, _) | Opcode::Jsr(_, _) => true,
            _ => false
        }
    }
}
