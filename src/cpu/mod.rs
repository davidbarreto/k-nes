pub mod opcode;
pub mod types;

mod instruction_set;
mod register_bank;

use crate::memory::Memory;
use crate::memory::types::AddressingMode;

use instruction_set::arithmetic::Arithmetic;
use instruction_set::branches::Branches;
use instruction_set::increments_decrements::IncrementsDecrements;
use instruction_set::jumps_calls::JumpsCalls;
use instruction_set::load_store::LoadStore;
use instruction_set::logical::Logical;
use instruction_set::register_transfers::RegisterTransfers;
use instruction_set::shifts::Shifts;
use instruction_set::stack_operations::StackOperations;
use instruction_set::status_flag_change::StatusFlagChange;

use register_bank::RegisterBank;
use opcode::Opcode;
use types::InstructionError;

pub struct Cpu {
    registers: RegisterBank,
    memory: Memory
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: RegisterBank::new(),
            memory: Memory::new()
        }
    }

    pub fn new_with_parameters(memory: Memory, registers: RegisterBank) -> Self {
        Self {
            registers,
            memory
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
                // Arithmetic
                Opcode::Adc(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.adc(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Sbc(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.sbc(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Cmp(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.cmp(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Cpx(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.cpx(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Cpy(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.cpy(data);
                    current_addressing_mode = addressing_mode;
                },
                // Branches
                Opcode::Bcc(_, addressing_mode) => {
                    let address: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bcc(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Bcs(_, addressing_mode) => {
                    let address: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bcs(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Beq(_, addressing_mode) => {
                    let address: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.beq(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Bmi(_, addressing_mode) => {
                    let address: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bmi(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Bne(_, addressing_mode) => {
                    let address: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bne(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Bpl(_, addressing_mode) => {
                    let address: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bpl(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Bvc(_, addressing_mode) => {
                    let address: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bvc(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Bvs(_, addressing_mode) => {
                    let address: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bvs(address);
                    current_addressing_mode = addressing_mode;
                },
                // IncrementsDecrements
                Opcode::Inc(_, addressing_mode) => {
                    let address: u16 = self.calculate_address(addressing_mode);
                    self.inc(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Inx(_, addressing_mode) => {
                    self.inx();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Iny(_, addressing_mode) => {
                    self.iny();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Dec(_, addressing_mode) => {
                    let address: u16 = self.calculate_address(addressing_mode);
                    self.dec(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Dex(_, addressing_mode) => {
                    self.dex();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Dey(_, addressing_mode) => {
                    self.dey();
                    current_addressing_mode = addressing_mode;
                },
                // JumpsCalls
                Opcode::Jmp(_, addressing_mode) => {
                    let address: u16 = self.calculate_address(addressing_mode);
                    self.jmp(address);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Jsr(_, addressing_mode) => {
                    let address: u16 = self.calculate_address(addressing_mode);
                    self.jsr(address, addressing_mode.byte_size());
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Rts(_, addressing_mode) => {
                    self.rts();
                    current_addressing_mode = addressing_mode;
                    // According to specs, we are supposed to save next instruction - 1,
                    // to PC. In order adjust PC this program logic, we have to increment it
                    // so it will point to next instruction.
                    self.registers.program_counter += 1;
                },
                // LoadStore
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
                Opcode::Ldy(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.ldy(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Sta(_, addressing_mode) => {
                    let data: u16 = self.calculate_address(addressing_mode);
                    self.sta(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Stx(_, addressing_mode) => {
                    let data: u16 = self.calculate_address(addressing_mode);
                    self.stx(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Sty(_, addressing_mode) => {
                    let data: u16 = self.calculate_address(addressing_mode);
                    self.sty(data);
                    current_addressing_mode = addressing_mode;
                },
                // Logical
                Opcode::And(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.and(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Ora(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.ora(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Eor(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.eor(data);
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Bit(_, addressing_mode) => {
                    let data: u8 = self.memory.read(self.calculate_address(addressing_mode));
                    self.bit(data);
                    current_addressing_mode = addressing_mode;
                },
                // RegisterTransfers
                Opcode::Tax(_, addressing_mode) => {
                    self.tax();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Tay(_, addressing_mode) => {
                    self.tay();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Txa(_, addressing_mode) => {
                    self.txa();
                    current_addressing_mode = addressing_mode;
                }
                Opcode::Tya(_, addressing_mode) => {
                    self.tya();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Tsx(_, addressing_mode) => {
                    self.tsx();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Txs(_, addressing_mode) => {
                    self.txs();
                    current_addressing_mode = addressing_mode;
                },
                // Shifts
                Opcode::Asl(_, addressing_mode) => {
                    if addressing_mode == AddressingMode::Accumulator {
                        self.asl_accumulator();    
                    } else {
                        let address: u16 = self.calculate_address(addressing_mode);
                        self.asl(address);
                    }
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Lsr(_, addressing_mode) => {
                    if addressing_mode == AddressingMode::Accumulator {
                        self.lsr_accumulator();    
                    } else {
                        let address: u16 = self.calculate_address(addressing_mode);
                        self.lsr(address);
                    }
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Rol(_, addressing_mode) => {
                    if addressing_mode == AddressingMode::Accumulator {
                        self.rol_accumulator();    
                    } else {
                        let address: u16 = self.calculate_address(addressing_mode);
                        self.rol(address);
                    }
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Ror(_, addressing_mode) => {
                    if addressing_mode == AddressingMode::Accumulator {
                        self.ror_accumulator();    
                    } else {
                        let address: u16 = self.calculate_address(addressing_mode);
                        self.ror(address);
                    }
                    current_addressing_mode = addressing_mode;
                },
                // Stack Operations
                Opcode::Pha(_, addressing_mode) => {
                    self.pha();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Php(_, addressing_mode) => {
                    self.php();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Pla(_, addressing_mode) => {
                    self.pla();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Plp(_, addressing_mode) => {
                    self.plp();
                    current_addressing_mode = addressing_mode;
                },
                // StatusFlagChange
                Opcode::Clc(_, addressing_mode) => {
                    self.clc();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Cld(_, addressing_mode) => {
                    self.cld();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Cli(_, addressing_mode) => {
                    self.cli();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Clv(_, addressing_mode) => {
                    self.clv();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Sec(_, addressing_mode) => {
                    self.sec();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Sed(_, addressing_mode) => {
                    self.sed();
                    current_addressing_mode = addressing_mode;
                },
                Opcode::Sei(_, addressing_mode) => {
                    self.sei();
                    current_addressing_mode = addressing_mode;
                },
                // No Operation
                Opcode::Nop(_, addressing_mode) => {
                    current_addressing_mode = addressing_mode;
                },
                // Handling opcode not implemented
                _ => {
                    return Err(InstructionError::NotImplementedInstruction(op));
                },
            }

            // Jump instructions will set PC direct to next instruction address
            // So, no need to increment PC in these cases
            if !opcode.is_jump_instruction() {
                self.registers.program_counter += current_addressing_mode.byte_size() as u16 - 1;
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

            AddressingMode::Relative => {
                self.registers.program_counter
            }

            _ => panic!("Addressing mode {:?} not available", mode),
        }
    }
}

#[cfg(test)]
mod tests;
