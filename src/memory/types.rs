#[derive(Debug, PartialEq)]
pub enum MemoryError {
    AccessViolation(usize),
}

/// Addressing mode for 6502 processor
/// Each addressing mode is a way in which memory locations can be addressed.
/// see: [NESDEV Obelisk 6502 guide - Addressing Modes](https://www.nesdev.org/obelisk-6502-guide/addressing.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// Absolute addressing - 16-bit address direct into the instruction
    /// Instructions will work with the value at memory address that is writen in the instruction
    /// operand directly.
    /// Example: LDA $0x8000 will load the value at memory address 0x8000 into the accumulator.
    Absolute,
    /// Absolute addressing with X register offset. The same as Absolute, but the X register is added to the address
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Indirect,
    /// Indexed Indirect or Indirect X
    IndirectX,
    /// Indirect Indexed or Indirect Y
    IndirectY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}