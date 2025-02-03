use bitflags::bitflags;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum InstructionError {
    InvalidOpcode(u8),
    InvalidInstruction(String),
    NotImplementedOpcode(u8),
    NotImplementedInstruction(String),
    InvalidIMnemonicAndAddressingModePair(String, String),
    AddressingModeNotRecognized(String),
    FatalError(String)
}

impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionError::InvalidOpcode(opcode) => {
                write!(f, "Invalid opcode: 0x{:02X}", opcode)
            }
            InstructionError::NotImplementedOpcode(opcode) => {
                write!(f, "Not implemented opcode: 0x{:02X}", opcode)
            },
            InstructionError::NotImplementedInstruction(mnemonic) => {
                write!(f, "Not implemented instruction: {}", mnemonic)
            },
            InstructionError::InvalidIMnemonicAndAddressingModePair(mnemonic, addressing_mode) => {
                write!(f, "The mnemonic/addressing mode pair [{}/{}] is invalid", mnemonic, addressing_mode)
            },
            InstructionError::InvalidInstruction(ref mnemonic) => {
                write!(f, "Invalid instruction with mnemonic: {}", mnemonic)
            },
            InstructionError::AddressingModeNotRecognized(ref data) => {
                write!(f, "Addressing mode not recognized for data: {}", data)
            },
            InstructionError::FatalError(ref message) => {
                write!(f, "Fatal error! Message: {}", message)
            }
        }
    }
}

bitflags! {
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const UNUSED            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}
