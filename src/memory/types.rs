#[derive(Debug, PartialEq)]
pub enum MemoryError {
    AccessViolation(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Indirect,
    IndirectX,
    IndirectY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}