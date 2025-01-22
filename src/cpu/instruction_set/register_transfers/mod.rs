use crate::cpu::types::CpuFlags;
use crate::cpu::Cpu;

pub trait RegisterTransfers {
    fn tax(&mut self);
    fn tay(&mut self);
    fn txa(&mut self);
    fn tya(&mut self);
    fn tsx(&mut self);
    fn txs(&mut self);
}

impl RegisterTransfers for Cpu {

    fn tax(&mut self) {
        self.registers.x_register = self.registers.accumulator;
        raise_flags(self, self.registers.x_register);
    }

    fn tay(&mut self) {
        self.registers.y_register = self.registers.accumulator;
        raise_flags(self, self.registers.y_register);
    }

    fn txa(&mut self) {
        self.registers.accumulator = self.registers.x_register;
        raise_flags(self, self.registers.accumulator);
    }

    fn tya(&mut self) {
        self.registers.accumulator = self.registers.y_register;
        raise_flags(self, self.registers.accumulator);
    }

    fn tsx(&mut self) {
        self.registers.x_register = self.registers.stack_pointer;
        raise_flags(self, self.registers.x_register);
    }

    fn txs(&mut self) {
        self.registers.stack_pointer = self.registers.x_register;
        raise_flags(self, self.registers.stack_pointer);
    }
}

fn raise_flags(cpu: &mut Cpu, value: u8) {
    cpu.registers.status.set(CpuFlags::ZERO, value == 0);
    cpu.registers.status.set(CpuFlags::NEGATIVE, value & 0x80 == 0x80);
}

#[cfg(test)]
mod tests;