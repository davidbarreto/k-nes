use regex::Regex;
use lazy_static::lazy_static;

const HEX_8_BIT: &str = r"(\$|0x)[0-9A-Fa-f]{2}";
const BIN_8_BIT: &str = r"(%|0b)[01]{8}";
const OCTAL_8_BIT:&str = r"(@|0o)[0-7]{1,3}";
const DEC_8_BIT: &str = r"25[0-5]|2[0-4][0-9]|1?[0-9]{1,2}";

const HEX_16_BIT: &str = r"(\$|0x)[0-9A-Fa-f]{4}";
const BIN_16_BIT: &str = r"(%|0b)[01]{16}";
const OCTAL_16_BIT: &str = r"(@|0o)[0-7]{1,5}";
const DEC_16_BIT: &str = r"6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{2}|[0-5]?[0-9]{1,4}";

lazy_static! {

    static ref NUM_8_BIT: String = format!("(?P<number>{}|{}|{}|{})", HEX_8_BIT, BIN_8_BIT, OCTAL_8_BIT, DEC_8_BIT);
    static ref NUM_16_BIT: String = format!("(?P<number>{}|{}|{}|{})", HEX_16_BIT, BIN_16_BIT, OCTAL_16_BIT, DEC_16_BIT);

    static ref ACCUMULATOR_REGEX: Regex = Regex::new(r"^A$").unwrap();
    static ref IMPLICIT_REGEX: Regex = Regex::new(r"^$").unwrap();
    static ref RELATIVE_REGEX: Regex = Regex::new(&format!(r"^{}$", NUM_8_BIT.as_str())).unwrap();
    static ref IMMEDIATE_REGEX: Regex = Regex::new(&format!(r"^#{}$", NUM_8_BIT.as_str())).unwrap();
    static ref ZERO_PAGE_REGEX: Regex = Regex::new(&format!(r"^{}$", NUM_8_BIT.as_str())).unwrap();
    static ref ZERO_PAGE_X_REGEX: Regex = Regex::new(&format!(r"^{}\s*,\s*X$", NUM_8_BIT.as_str())).unwrap();
    static ref ZERO_PAGE_Y_REGEX: Regex = Regex::new(&format!(r"^\${}\s*,\s*Y$", NUM_8_BIT.as_str())).unwrap();
    static ref ABSOLUTE_REGEX: Regex = Regex::new(&format!(r"^\${}$", NUM_16_BIT.as_str())).unwrap();
    static ref ABSOLUTE_X_REGEX: Regex = Regex::new(&format!(r"^{}\s*,\s*X$", NUM_16_BIT.as_str())).unwrap();
    static ref ABSOLUTE_Y_REGEX: Regex = Regex::new(&format!(r"^{}\s*,\s*Y$", NUM_16_BIT.as_str())).unwrap();
    static ref INDIRECT_REGEX: Regex = Regex::new(&format!(r"^\({}\)$", NUM_16_BIT.as_str())).unwrap();
    static ref INDIRECT_X_REGEX: Regex = Regex::new(&format!(r"^\({}\s*,\s*X\)$", NUM_8_BIT.as_str())).unwrap();
    static ref INDIRECT_Y_REGEX: Regex = Regex::new(&format!(r"^\({}\)\s*,\s*Y$", NUM_8_BIT.as_str())).unwrap();
}

#[derive(Debug, PartialEq)]
pub enum MemoryError {
    AccessViolation(usize),
}

// Addressing modes for the 6502 processor
///
/// Each variant of this enum represents a specific addressing mode used by the 6502 instruction set.
/// Addressing modes determine how the instruction will access its operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    /// Absolute addressing - 16-bit address direct into the instruction
    ///
    /// Instructions will work with the value at the memory address that is written in the instruction
    /// operand directly.
    ///
    /// Example: `LDA $0x8000` will load the value at memory address `0x8000` into the accumulator.
    Absolute,

    /// Absolute addressing with X register offset
    ///
    /// The same as Absolute, but the X register is added to the address.
    ///
    /// Example: `LDA $0x8000,X` will load the value at memory address `0x8000 + X` into the accumulator.
    AbsoluteX,

    /// Absolute addressing with Y register offset
    ///
    /// The same as Absolute, but the Y register is added to the address.
    ///
    /// Example: `LDA $0x8000,Y` will load the value at memory address `0x8000 + Y` into the accumulator.
    AbsoluteY,

    /// Accumulator addressing
    ///
    /// The instruction operates directly on the accumulator.
    ///
    /// Example: `ASL A` will perform an arithmetic shift left on the accumulator.
    Accumulator,

    /// Immediate addressing
    ///
    /// The operand is given as an immediate value in the instruction.
    ///
    /// Example: `LDA #$10` will load the value `0x10` directly into the accumulator.
    Immediate,

    /// Implicit addressing
    ///
    /// The operand is implied by the instruction itself.
    ///
    /// Example: `CLC` will clear the carry flag.
    Implicit,

    /// Indirect addressing
    ///
    /// The instruction operand specifies a memory address that contains the effective address.
    ///
    /// Example: `JMP ($0x1234)` will jump to the address stored at memory location `0x1234`.
    Indirect,

    /// Indexed Indirect addressing (Indirect X)
    ///
    /// The operand specifies a zero-page address, which is added to the X register to get the effective address.
    ///
    /// Example: `LDA ($20,X)` will load the value at the address found by adding `X` to the zero-page address `$20`.
    IndirectX,

    /// Indirect Indexed addressing (Indirect Y)
    ///
    /// The operand specifies a zero-page address, which is used to get the base address. The Y register is then added to this base address to get the effective address.
    ///
    /// Example: `LDA ($20),Y` will load the value at the address found by adding `Y` to the base address stored at zero-page address `$20`.
    IndirectY,

    /// Relative addressing
    ///
    /// The operand is a signed 8-bit offset relative to the current program counter.
    /// This addressing mode is used for branch instructions.
    ///
    /// Example: `BEQ $10` will branch to the address `current_program_counter + 0x10` if the zero flag is set.
    Relative,

    /// Zero Page addressing
    ///
    /// The operand specifies an address in the zero page (first 256 bytes of memory).
    ///
    /// Example: `LDA $20` will load the value at zero-page address `$20` into the accumulator.
    ZeroPage,

    /// Zero Page addressing with X register offset
    ///
    /// The same as Zero Page, but the X register is added to the address.
    ///
    /// Example: `LDA $20,X` will load the value at zero-page address `$20 + X` into the accumulator.
    ZeroPageX,

    /// Zero Page addressing with Y register offset
    ///
    /// The same as Zero Page, but the Y register is added to the address.
    ///
    /// Example: `LDA $20,Y` will load the value at zero-page address `$20 + Y` into the accumulator.
    ZeroPageY
}

impl AddressingMode {
    pub fn byte_size(&self) -> u8 {
        match self {
            AddressingMode::Implicit => 1,
            AddressingMode::Immediate => 2,
            AddressingMode::ZeroPage => 2,
            AddressingMode::ZeroPageX => 2,
            AddressingMode::ZeroPageY => 2,
            AddressingMode::Absolute => 3,
            AddressingMode::AbsoluteX => 3,
            AddressingMode::AbsoluteY => 3,
            AddressingMode::Indirect => 3,
            AddressingMode::IndirectX => 2,
            AddressingMode::IndirectY => 2,
            AddressingMode::Accumulator => 1,
            AddressingMode::Relative => 2
        }
    }

    pub fn regex(&self) -> &'static Regex {
        match self {
            AddressingMode::Implicit => &IMPLICIT_REGEX,
            AddressingMode::Immediate => &IMMEDIATE_REGEX,
            AddressingMode::ZeroPage => &ZERO_PAGE_REGEX,
            AddressingMode::ZeroPageX => &ZERO_PAGE_X_REGEX,
            AddressingMode::ZeroPageY => &ZERO_PAGE_Y_REGEX,
            AddressingMode::Absolute => &ABSOLUTE_REGEX,
            AddressingMode::AbsoluteX => &ABSOLUTE_X_REGEX,
            AddressingMode::AbsoluteY => &ABSOLUTE_Y_REGEX,
            AddressingMode::Indirect => &INDIRECT_REGEX,
            AddressingMode::IndirectX => &INDIRECT_X_REGEX,
            AddressingMode::IndirectY => &INDIRECT_Y_REGEX,
            AddressingMode::Accumulator => &ACCUMULATOR_REGEX,
            AddressingMode::Relative => &RELATIVE_REGEX
        }
    }
}