pub mod opcode;
pub mod types;

mod instruction_set;
mod register_bank;

use crate::memory::Memory;
use crate::memory::types::AddressingMode;

use instruction_set::arithmetic::Arithmetic;
use instruction_set::load_store::LoadStore;
use instruction_set::register_transfers::RegisterTransfers;
use instruction_set::increments_decrements::IncrementsDecrements;
use instruction_set::branches::Branches;

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
        loop {
            let opcode = self.memory.read(self.registers.program_counter);
            if opcode == opcode::BRK {
                self.registers.program_counter += 1;
                break;
            }
            match self.execute_instruction(opcode) {
                Err(instruction_error) => return Err(instruction_error),
                Ok(_) => ()
            }
        }
        Ok(())
    }

    fn execute_instruction(&mut self, op: u8) -> Result<(), InstructionError> {
        
        let current_addressing_mode: AddressingMode;
        if let Some(opcode) = Opcode::from_u8(op) {
            
            self.registers.program_counter += 1;

            match opcode {
                Opcode::Adc(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.adc(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Lda(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.lda(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Ldx(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.ldx(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Dex(_, addressing_mode) => {
                    self.dex();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Tax(_, addressing_mode) => {
                    self.tax();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Inx(_, addressing_mode) => {
                    self.inx();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Stx(_, addressing_mode) => {
                    let data: u16 = self.calculate_address(addressing_mode);
                    self.stx(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Cpx(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.cpx(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Bne(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bne(data);
                    current_addressing_mode = addressing_mode;
                },
                _ => {
                    return Err(InstructionError::NotImplementedInstruction(op));
                },
            }
        } else {
            return Err(InstructionError::InvalidOpcode(op));
        }

        self.registers.program_counter += current_addressing_mode.byte_size() as u16 - 1;
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

            AddressingMode::Relative => {
                self.registers.program_counter
            }

            _ => panic!("Addressing mode {:?} not available", mode),
        }
    }
}

#[cfg(test)]
mod tests;
