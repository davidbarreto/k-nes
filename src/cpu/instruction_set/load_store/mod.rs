use crate::cpu::types::CpuFlags;
use crate::cpu::Cpu;

pub trait LoadStore {
    fn lda(&mut self, data: u8);
    fn ldx(&mut self, data: u8);
    fn ldy(&mut self, data: u8);
    fn sta(&mut self, address: u8);
    fn stx(&mut self, address: u8);
    fn sty(&mut self, address: u8);
}

impl LoadStore for Cpu {

    fn lda(&mut self, data: u8) {
        self.registers.accumulator = data;
        self.registers.status.set(CpuFlags::ZERO, data == 0);
        self.registers.status.set(CpuFlags::NEGATIVE, data & 0x80 == 0x80);
    }

    fn ldx(&mut self, data: u8) {
        self.registers.x_register = data;
        self.registers.status.set(CpuFlags::ZERO, data == 0);
        self.registers.status.set(CpuFlags::NEGATIVE, data & 0x80 == 0x80);
    }

    fn ldy(&mut self, data: u8) {
        self.registers.y_register = data;
        self.registers.status.set(CpuFlags::ZERO, data == 0);
        self.registers.status.set(CpuFlags::NEGATIVE, data & 0x80 == 0x80);
    }

    fn sta(&mut self, address: u8) {
        self.memory.write(self.registers.accumulator, address as u16);
    }

    fn stx(&mut self, address: u8) {
        self.memory.write(self.registers.x_register, address as u16);
    }

    fn sty(&mut self, address: u8) {
        self.memory.write(self.registers.y_register, address as u16);
    }
}

#[cfg(test)]
mod tests;