use crate::cpu::types::CpuFlags;
use crate::cpu::Cpu;

pub trait StatusFlagChange {
    fn clc(&mut self);
    fn cld(&mut self);
    fn cli(&mut self);
    fn clv(&mut self);
    fn sec(&mut self);
    fn sed(&mut self);
    fn sei(&mut self);
}

impl StatusFlagChange for Cpu {
    fn clc(&mut self) {
        self.registers.status.remove(CpuFlags::CARRY);
    }

    fn cld(&mut self) {
        self.registers.status.remove(CpuFlags::DECIMAL_MODE);
    }

    fn cli(&mut self) {
        self.registers.status.remove(CpuFlags::INTERRUPT_DISABLE);
    }

    fn clv(&mut self) {
        self.registers.status.remove(CpuFlags::OVERFLOW);
    }

    fn sec(&mut self) {
        self.registers.status.insert(CpuFlags::CARRY);
    }

    fn sed(&mut self) {
        self.registers.status.insert(CpuFlags::DECIMAL_MODE);
    }

    fn sei(&mut self) {
        self.registers.status.insert(CpuFlags::INTERRUPT_DISABLE);
    }
}

#[cfg(test)]
mod tests;