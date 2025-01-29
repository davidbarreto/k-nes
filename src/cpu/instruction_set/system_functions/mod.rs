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
        
    }
}

#[cfg(test)]
mod tests;