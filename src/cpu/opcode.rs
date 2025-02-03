use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

use crate::memory::types::AddressingMode;

use super::types::InstructionError;

lazy_static! {
    static ref OPCODE_MAP: HashMap<(&'static str, AddressingMode), u8> = {
        let mut map = HashMap::new();
        map.insert(("ADC", AddressingMode::Immediate), ADC_IMMEDIATE);
        map.insert(("ADC", AddressingMode::ZeroPage), ADC_ZERO_PAGE);
        map.insert(("ADC", AddressingMode::ZeroPageX), ADC_ZERO_PAGE_X);
        map.insert(("ADC", AddressingMode::Absolute), ADC_ABSOLUTE);
        map.insert(("ADC", AddressingMode::AbsoluteX), ADC_ABSOLUTE_X);
        map.insert(("ADC", AddressingMode::AbsoluteY), ADC_ABSOLUTE_Y);
        map.insert(("ADC", AddressingMode::IndirectX), ADC_INDIRECT_X);
        map.insert(("ADC", AddressingMode::IndirectY), ADC_INDIRECT_Y);
        
        map.insert(("AND", AddressingMode::Immediate), AND_IMMEDIATE);
        map.insert(("AND", AddressingMode::ZeroPage), AND_ZERO_PAGE);
        map.insert(("AND", AddressingMode::ZeroPageX), AND_ZERO_PAGE_X);
        map.insert(("AND", AddressingMode::Absolute), AND_ABSOLUTE);
        map.insert(("AND", AddressingMode::AbsoluteX), AND_ABSOLUTE_X);
        map.insert(("AND", AddressingMode::AbsoluteY), AND_ABSOLUTE_Y);
        map.insert(("AND", AddressingMode::IndirectX), AND_INDIRECT_X);
        map.insert(("AND", AddressingMode::IndirectY), AND_INDIRECT_Y);

        map.insert(("ASL", AddressingMode::Accumulator), ASL_ACCUMULATOR);
        map.insert(("ASL", AddressingMode::ZeroPage), ASL_ZERO_PAGE);
        map.insert(("ASL", AddressingMode::ZeroPageX), ASL_ZERO_PAGE_X);
        map.insert(("ASL", AddressingMode::Absolute), ASL_ABSOLUTE);
        map.insert(("ASL", AddressingMode::AbsoluteX), ASL_ABSOLUTE_X);

        map.insert(("BCC", AddressingMode::Relative), BCC);

        map.insert(("BCS", AddressingMode::Relative), BCS);

        map.insert(("BEQ", AddressingMode::Relative), BEQ);

        map.insert(("BIT", AddressingMode::ZeroPage), BIT_ZERO_PAGE);
        map.insert(("BIT", AddressingMode::Absolute), BIT_ABSOLUTE);

        map.insert(("BMI", AddressingMode::Relative), BMI);

        map.insert(("BNE", AddressingMode::Relative), BNE);

        map.insert(("BPL", AddressingMode::Relative), BPL);

        map.insert(("BRK", AddressingMode::Relative), BRK);

        map.insert(("BVC", AddressingMode::Relative), BVC);

        map.insert(("BNS", AddressingMode::Relative), BVS);

        map.insert(("CLC", AddressingMode::Implicit), CLC);

        map.insert(("CLD", AddressingMode::Implicit), CLD);

        map.insert(("CLI", AddressingMode::Implicit), CLI);

        map.insert(("CLV", AddressingMode::Implicit), CLV);

        map.insert(("CMP", AddressingMode::Immediate), CMP_IMMEDIATE);
        map.insert(("CMP", AddressingMode::ZeroPage), CMP_ZERO_PAGE);
        map.insert(("CMP", AddressingMode::ZeroPageX), CMP_ZERO_PAGE_X);
        map.insert(("CMP", AddressingMode::Absolute), CMP_ABSOLUTE);
        map.insert(("CMP", AddressingMode::AbsoluteX), CMP_ABSOLUTE_X);
        map.insert(("CMP", AddressingMode::AbsoluteY), CMP_ABSOLUTE_Y);
        map.insert(("CMP", AddressingMode::IndirectX), CMP_INDIRECT_X);
        map.insert(("CMP", AddressingMode::IndirectY), CMP_INDIRECT_Y);

        map.insert(("CPX", AddressingMode::Immediate), CPX_IMMEDIATE);
        map.insert(("CPX", AddressingMode::ZeroPage), CPX_ZERO_PAGE);
        map.insert(("CPX", AddressingMode::Absolute), CPX_ABSOLUTE);

        map.insert(("CPY", AddressingMode::Immediate), CPY_IMMEDIATE);
        map.insert(("CPY", AddressingMode::ZeroPage), CPY_ZERO_PAGE);
        map.insert(("CPY", AddressingMode::Absolute), CPY_ABSOLUTE);

        map.insert(("DEC", AddressingMode::ZeroPage), DEC_ZERO_PAGE);
        map.insert(("DEC", AddressingMode::ZeroPageX), DEC_ZERO_PAGE_X);
        map.insert(("DEC", AddressingMode::Absolute), DEC_ABSOLUTE);
        map.insert(("DEC", AddressingMode::AbsoluteX), DEC_ABSOLUTE_X);

        map.insert(("DEX", AddressingMode::Implicit), DEX);

        map.insert(("DEY", AddressingMode::Implicit), DEY);

        map.insert(("EOR", AddressingMode::Immediate), EOR_IMMEDIATE);
        map.insert(("EOR", AddressingMode::ZeroPage), EOR_ZERO_PAGE);
        map.insert(("EOR", AddressingMode::ZeroPageX), EOR_ZERO_PAGE_X);
        map.insert(("EOR", AddressingMode::Absolute), EOR_ABSOLUTE);
        map.insert(("EOR", AddressingMode::AbsoluteX), EOR_ABSOLUTE_X);
        map.insert(("EOR", AddressingMode::AbsoluteY), EOR_ABSOLUTE_Y);
        map.insert(("EOR", AddressingMode::IndirectX), EOR_INDIRECT_X);
        map.insert(("EOR", AddressingMode::IndirectY), EOR_INDIRECT_Y);

        map.insert(("INC", AddressingMode::ZeroPage), INC_ZERO_PAGE);
        map.insert(("INC", AddressingMode::ZeroPageX), INC_ZERO_PAGE_X);
        map.insert(("INC", AddressingMode::Absolute), INC_ABSOLUTE);
        map.insert(("INC", AddressingMode::AbsoluteX), INC_ABSOLUTE_X);

        map.insert(("INX", AddressingMode::Implicit), INX);

        map.insert(("INY", AddressingMode::Implicit), INY);

        map.insert(("JMP", AddressingMode::Absolute), JMP_ABSOLUTE);
        map.insert(("JMP", AddressingMode::Indirect), JMP_INDIRECT);

        map.insert(("JSR", AddressingMode::Absolute), JSR_ABSOLUTE);

        map.insert(("LDA", AddressingMode::Immediate), LDA_IMMEDIATE);
        map.insert(("LDA", AddressingMode::ZeroPage), LDA_ZERO_PAGE);
        map.insert(("LDA", AddressingMode::ZeroPageX), LDA_ZERO_PAGE_X);
        map.insert(("LDA", AddressingMode::Absolute), LDA_ABSOLUTE);
        map.insert(("LDA", AddressingMode::AbsoluteX), LDA_ABSOLUTE_X);
        map.insert(("LDA", AddressingMode::AbsoluteY), LDA_ABSOLUTE_Y);
        map.insert(("LDA", AddressingMode::IndirectX), LDA_INDIRECT_X);
        map.insert(("LDA", AddressingMode::IndirectY), LDA_INDIRECT_Y);

        map.insert(("LDX", AddressingMode::Immediate), LDX_IMMEDIATE);
        map.insert(("LDX", AddressingMode::ZeroPage), LDX_ZERO_PAGE);
        map.insert(("LDX", AddressingMode::ZeroPageY), LDX_ZERO_PAGE_Y);
        map.insert(("LDX", AddressingMode::Absolute), LDX_ABSOLUTE);
        map.insert(("LDX", AddressingMode::AbsoluteY), LDX_ABSOLUTE_Y);

        map.insert(("LDY", AddressingMode::Immediate), LDY_IMMEDIATE);
        map.insert(("LDY", AddressingMode::ZeroPage), LDY_ZERO_PAGE);
        map.insert(("LDY", AddressingMode::ZeroPageX), LDY_ZERO_PAGE_X);
        map.insert(("LDY", AddressingMode::Absolute), LDY_ABSOLUTE);
        map.insert(("LDY", AddressingMode::AbsoluteX), LDY_ABSOLUTE_X);

        map.insert(("LSR", AddressingMode::Accumulator), LSR_ACCUMULATOR);
        map.insert(("LSR", AddressingMode::ZeroPage), LSR_ZERO_PAGE);
        map.insert(("LSR", AddressingMode::ZeroPageX), LSR_ZERO_PAGE_X);
        map.insert(("LSR", AddressingMode::Absolute), LSR_ABSOLUTE);
        map.insert(("LSR", AddressingMode::AbsoluteX), LSR_ABSOLUTE_X);

        map.insert(("NOP", AddressingMode::Implicit), NOP);

        map.insert(("ORA", AddressingMode::Immediate), ORA_IMMEDIATE);
        map.insert(("ORA", AddressingMode::ZeroPage), ORA_ZERO_PAGE);
        map.insert(("ORA", AddressingMode::ZeroPageX), ORA_ZERO_PAGE_X);
        map.insert(("ORA", AddressingMode::Absolute), ORA_ABSOLUTE);
        map.insert(("ORA", AddressingMode::AbsoluteX), ORA_ABSOLUTE_X);
        map.insert(("ORA", AddressingMode::AbsoluteY), ORA_ABSOLUTE_Y);
        map.insert(("ORA", AddressingMode::IndirectX), ORA_INDIRECT_X);
        map.insert(("ORA", AddressingMode::IndirectY), ORA_INDIRECT_Y);

        map.insert(("PHA", AddressingMode::Implicit), PHA);

        map.insert(("PHP", AddressingMode::Implicit), PHP);

        map.insert(("PLA", AddressingMode::Implicit), PLA);

        map.insert(("PLP", AddressingMode::Implicit), PLP);

        map.insert(("ROL", AddressingMode::Accumulator), ROL_ACCUMULATOR);
        map.insert(("ROL", AddressingMode::ZeroPage), ROL_ZERO_PAGE);
        map.insert(("ROL", AddressingMode::ZeroPageX), ROL_ZERO_PAGE_X);
        map.insert(("ROL", AddressingMode::Absolute), ROL_ABSOLUTE);
        map.insert(("ROL", AddressingMode::AbsoluteX), ROL_ABSOLUTE_X);

        map.insert(("ROR", AddressingMode::Accumulator), ROR_ACCUMULATOR);
        map.insert(("ROR", AddressingMode::ZeroPage), ROR_ZERO_PAGE);
        map.insert(("ROR", AddressingMode::ZeroPageX), ROR_ZERO_PAGE_X);
        map.insert(("ROR", AddressingMode::Absolute), ROR_ABSOLUTE);
        map.insert(("ROR", AddressingMode::AbsoluteX), ROR_ABSOLUTE_X);

        map.insert(("RTI", AddressingMode::Implicit), RTI);

        map.insert(("RTS", AddressingMode::Implicit), RTS);

        map.insert(("SBC", AddressingMode::Immediate), SBC_IMMEDIATE);
        map.insert(("SBC", AddressingMode::ZeroPage), SBC_ZERO_PAGE);
        map.insert(("SBC", AddressingMode::ZeroPageX), SBC_ZERO_PAGE_X);
        map.insert(("SBC", AddressingMode::Absolute), SBC_ABSOLUTE);
        map.insert(("SBC", AddressingMode::AbsoluteX), SBC_ABSOLUTE_X);
        map.insert(("SBC", AddressingMode::AbsoluteY), SBC_ABSOLUTE_Y);
        map.insert(("SBC", AddressingMode::IndirectX), SBC_INDIRECT_X);
        map.insert(("SBC", AddressingMode::IndirectY), SBC_INDIRECT_Y);

        map.insert(("SEC", AddressingMode::Implicit), SEC);

        map.insert(("SED", AddressingMode::Implicit), SED);

        map.insert(("SEI", AddressingMode::Implicit), SEI);

        map.insert(("STA", AddressingMode::ZeroPage), STA_ZERO_PAGE);
        map.insert(("STA", AddressingMode::ZeroPageX), STA_ZERO_PAGE_X);
        map.insert(("STA", AddressingMode::Absolute), STA_ABSOLUTE);
        map.insert(("STA", AddressingMode::AbsoluteX), STA_ABSOLUTE_X);
        map.insert(("STA", AddressingMode::AbsoluteY), STA_ABSOLUTE_Y);
        map.insert(("STA", AddressingMode::IndirectX), STA_INDIRECT_X);
        map.insert(("STA", AddressingMode::IndirectY), STA_INDIRECT_Y);

        map.insert(("STX", AddressingMode::ZeroPage), STX_ZERO_PAGE);
        map.insert(("STX", AddressingMode::ZeroPageY), STX_ZERO_PAGE_Y);
        map.insert(("STX", AddressingMode::Absolute), STX_ABSOLUTE);

        map.insert(("STY", AddressingMode::ZeroPage), STY_ZERO_PAGE);
        map.insert(("STY", AddressingMode::ZeroPageX), STY_ZERO_PAGE_X);
        map.insert(("STY", AddressingMode::Absolute), STY_ABSOLUTE);

        map.insert(("TAX", AddressingMode::Implicit), TAX);

        map.insert(("TAY", AddressingMode::Implicit), TAY);

        map.insert(("TSX", AddressingMode::Implicit), TSX);

        map.insert(("TXA", AddressingMode::Implicit), TXA);

        map.insert(("TXS", AddressingMode::Implicit), TXS);

        map.insert(("TYA", AddressingMode::Implicit), TYA);

        map
    };

    static ref OPCODE_SET: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("ADC");
        set.insert("AND");
        set.insert("ASL");
        set.insert("BCC");
        set.insert("BCS");
        set.insert("BEQ");
        set.insert("BIT");
        set.insert("BMI");
        set.insert("BNE");
        set.insert("BPL");
        set.insert("BRK");
        set.insert("BVC");
        set.insert("BVS");
        set.insert("CLC");
        set.insert("CLD");
        set.insert("CLI");
        set.insert("CLV");
        set.insert("CMP");
        set.insert("CPX");
        set.insert("CPY");
        set.insert("DEC");
        set.insert("DEX");
        set.insert("DEY");
        set.insert("EOR");
        set.insert("INC");
        set.insert("INX");
        set.insert("INY");
        set.insert("JMP");
        set.insert("JSR");
        set.insert("LDA");
        set.insert("LDX");
        set.insert("LDY");
        set.insert("LSR");
        set.insert("NOP");
        set.insert("ORA");
        set.insert("PHA");
        set.insert("PHP");
        set.insert("PLA");
        set.insert("PLP");
        set.insert("ROL");
        set.insert("ROR");
        set.insert("RTI");
        set.insert("RTS");
        set.insert("SBC");
        set.insert("SEC");
        set.insert("SED");
        set.insert("STA");
        set.insert("STX");
        set.insert("STY");
        set.insert("TAX");
        set.insert("TAY");
        set.insert("TSX");
        set.insert("TXA");
        set.insert("TXS");
        set.insert("TYA");

        set
    };
}

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

pub fn translate_instruction_to_opcode(mnemonic: &str, addressing_mode: AddressingMode) -> Result<u8, InstructionError> {
    OPCODE_MAP.get(&(mnemonic, addressing_mode))
        .copied()
        .ok_or_else(|| InstructionError::InvalidIMnemonicAndAddressingModePair(mnemonic.to_string(), addressing_mode.to_string()))
}

pub fn is_valid_mnemonic(mnemonic: &str) -> bool {
    OPCODE_SET.contains(mnemonic)
}

pub fn addressing_modes_from_mnemonic(mnemonic: &str) -> Result<Vec<AddressingMode>, InstructionError> {
    match mnemonic {
        "ADC" | "SBC" | "CMP" | "AND" | "EOR" | "ORA" | "LDA" => Ok(vec![
            AddressingMode::Immediate,
            AddressingMode::ZeroPage,
            AddressingMode::ZeroPageX,
            AddressingMode::Absolute,
            AddressingMode::AbsoluteX,
            AddressingMode::AbsoluteY,
            AddressingMode::IndirectX,
            AddressingMode::IndirectY,
        ]),
        "ASL" | "LSR" | "ROL" | "ROR" => Ok(vec![
            AddressingMode::Accumulator,
            AddressingMode::ZeroPage,
            AddressingMode::ZeroPageX,
            AddressingMode::Absolute,
            AddressingMode::AbsoluteX,
        ]),
        "BCC" | "BSC" | "BEQ" | "BMI" | "BNE" | "BPL" | "BVC" | "BVS" => Ok(vec![AddressingMode::Relative]),
        "BIT" => Ok(vec![AddressingMode::ZeroPage, AddressingMode::Absolute]),
        "BRK" | "CLC" | "CLD" | "CLI" | "CLV" | "DEX" | "DEY" | "INX" | "INY" | "NOP" | "PHA" | "PHP" | "PLA" | "PLP" | "RTI" | "RTS" | "SEC" | "SED" | "SEI" | "TAX" | "TAY" | "TSX" | "TXA" | "TXS" | "TYA" => Ok(vec![AddressingMode::Implicit]),
        "CPX" | "CPY" => Ok(vec![
            AddressingMode::Immediate,
            AddressingMode::ZeroPage,
            AddressingMode::Absolute,
        ]),
        "DEC" | "INC" => Ok(vec![
            AddressingMode::ZeroPage,
            AddressingMode::ZeroPageX,
            AddressingMode::Absolute,
            AddressingMode::AbsoluteX,
        ]),
        "JMP" => Ok(vec![AddressingMode::Absolute, AddressingMode::Indirect]),
        "JSR" => Ok(vec![AddressingMode::Absolute]),
        "LDX" => Ok(vec![
            AddressingMode::Immediate,
            AddressingMode::ZeroPage,
            AddressingMode::ZeroPageY,
            AddressingMode::Absolute,
            AddressingMode::AbsoluteY,
        ]),
        "LDY" => Ok(vec![
            AddressingMode::Immediate,
            AddressingMode::ZeroPage,
            AddressingMode::ZeroPageX,
            AddressingMode::Absolute,
            AddressingMode::AbsoluteX,
        ]),
        "STA" => Ok(vec![
            AddressingMode::ZeroPage,
            AddressingMode::ZeroPageX,
            AddressingMode::Absolute,
            AddressingMode::AbsoluteX,
            AddressingMode::AbsoluteY,
            AddressingMode::IndirectX,
            AddressingMode::IndirectY,
        ]),
        "STX" => Ok(vec![
            AddressingMode::ZeroPage,
            AddressingMode::ZeroPageY,
            AddressingMode::Absolute,
            AddressingMode::AbsoluteY,
        ]),
        "STY" => Ok(vec![
            AddressingMode::ZeroPage,
            AddressingMode::ZeroPageX,
            AddressingMode::Absolute,
            AddressingMode::AbsoluteX,
        ]),
        _ => Err(InstructionError::InvalidInstruction(mnemonic.to_string())),
    }
}
