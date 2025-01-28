use crate::cpu::types::CpuFlags;
use crate::cpu::Cpu;

pub trait IncrementsDecrements {
    fn inc(&mut self, address: u16);
    fn inx(&mut self);
    fn iny(&mut self);
    fn dec(&mut self, address: u16);
    fn dex(&mut self);
    fn dey(&mut self);
}

impl IncrementsDecrements for Cpu {
    /// Implementation of INC (Increment Memory) instruction
    fn inc(&mut self, address: u16) {
        let val = self.memory.read(address).wrapping_add(0x01);
        self.memory.write(val, address);
        update_flag(self, val);
    }

    /// Implementation of INX (Increment X Register) instruction
    fn inx(&mut self) {
        self.registers.x_register = self.registers.x_register.wrapping_add(0x01);
        update_flag(self, self.registers.x_register);
    }

    /// Implementation of INY (Increment Y Register) instruction
    fn iny(&mut self) {
        self.registers.y_register = self.registers.y_register.wrapping_add(0x01);
        update_flag(self, self.registers.y_register);
    }

    /// Implementation of DEC (Decrement Memory) instruction
    fn dec(&mut self, address: u16) {
        let val = self.memory.read(address).wrapping_add(0xff);
        self.memory.write(val, address);
        update_flag(self, val);
    }

    /// Implementation of DEX (Decrement X Register) instruction
    fn dex(&mut self) {
        self.registers.x_register = self.registers.x_register.wrapping_add(0xff);
        update_flag(self, self.registers.x_register);
    }

    /// Implementation of DEY (Decrement Y Register) instruction
    fn dey(&mut self) {
        self.registers.y_register = self.registers.y_register.wrapping_add(0xff);
        update_flag(self, self.registers.y_register);
    }
}

fn update_flag(cpu: &mut Cpu, val: u8) {
    cpu.registers.status.set(CpuFlags::ZERO, val == 0);
    cpu.registers.status.set(CpuFlags::NEGATIVE, val & 0x80 == 0x80);
}

#[cfg(test)]
mod tests;
