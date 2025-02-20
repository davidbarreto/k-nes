use crate::cpu::register_bank::STACK_POINTER_BASE_ADDRESS;
use crate::cpu::Cpu;
use crate::cpu::types::CpuFlags;

pub trait SystemFunctions {
    fn brk(&mut self);
    fn rti(&mut self);
    // As NOP doesn't do anything, I'll not add it
}

impl SystemFunctions for Cpu {
    fn brk(&mut self) {
        self.registers.status.insert(CpuFlags::BREAK);
    }

    fn rti(&mut self) {
        
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        self.registers.status = CpuFlags::from_bits_truncate(self.memory.read(STACK_POINTER_BASE_ADDRESS + self.registers.stack_pointer as u16));
        
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        let low_byte = self.memory.read(STACK_POINTER_BASE_ADDRESS + self.registers.stack_pointer as u16) as u16;
        
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        let high_byte = self.memory.read(STACK_POINTER_BASE_ADDRESS + self.registers.stack_pointer as u16) as u16;

        self.registers.program_counter = high_byte << 8 | low_byte;
    }
}

#[cfg(test)]
mod tests;