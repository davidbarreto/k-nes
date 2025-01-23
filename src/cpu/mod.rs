pub mod opcode;
pub mod types;

mod instruction_set;
mod register_bank;

use crate::memory::Memory;
use crate::memory::types::AddressingMode;

use instruction_set::arithmetic::Arithmetic;
use register_bank::RegisterBank;
use opcode::Opcode;
use types::InstructionError;

pub struct Cpu {
    registers: RegisterBank,
    memory: Memory,
    cycles: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: RegisterBank::new(),
            memory: Memory::new(),
            cycles: 0,
        }
    }

    pub fn new_with_parameters(memory: Memory, registers: RegisterBank, cycles: u64) -> Self {
        Self {
            registers,
            memory,
            cycles,
        }
    }

    pub fn execute_program(&mut self) -> Result<(), InstructionError> {
        //loop {
            let opcode = self.memory.read(self.registers.program_counter);
            self.execute_instruction(opcode)
        //}
    }

    fn execute_instruction(&mut self, op: u8) -> Result<(), InstructionError> {
        if let Some(opcode) = Opcode::from_u8(op) {
            self.registers.program_counter += 1;

            match opcode {
                Opcode::Adc(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.adc(data);
                }
                _ => (),
            }
        } else {
            return Err(InstructionError::InvalidOpcode(op));
        }

        Ok(())
    }

    /// Returns the address required by the AddressingMode of the current instruction
    /// It assumes that program_counter (PC) was already incremented after opcode parsing
    /// Some addressing modes may not have a memory address to return. E.g. AddressingMode::Accumulator
    fn calculate_address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Absolute => self.memory.read_u16(self.registers.program_counter),

            AddressingMode::AbsoluteX => {
                let address = self.memory.read_u16(self.registers.program_counter);
                address.wrapping_add(self.registers.x_register as u16)
            }

            AddressingMode::AbsoluteY => {
                let address = self.memory.read_u16(self.registers.program_counter);
                address.wrapping_add(self.registers.y_register as u16)
            }

            AddressingMode::Immediate => self.registers.program_counter,

            AddressingMode::Indirect => {
                self.memory.read_u16(
                    self.memory.read_u16(self.registers.program_counter))
            },

            AddressingMode::IndirectX => {
                let base = self.memory.read(self.registers.program_counter);
                base.wrapping_add(self.registers.x_register) as u16
            }

            AddressingMode::IndirectY => {
                let address = self.memory.read(self.registers.program_counter) as u16;
                address + (self.registers.y_register as u16)
            }

            AddressingMode::ZeroPage => self.memory.read(self.registers.program_counter) as u16,

            AddressingMode::ZeroPageX => {
                let address = self.memory.read(self.registers.program_counter);
                address.wrapping_add(self.registers.x_register) as u16
            }

            AddressingMode::ZeroPageY => {
                let address = self.memory.read(self.registers.program_counter);
                address.wrapping_add(self.registers.y_register) as u16
            }

            _ => panic!("Addressing mode {:?} not available", mode),
        }
    }
}

#[cfg(test)]
mod tests;
