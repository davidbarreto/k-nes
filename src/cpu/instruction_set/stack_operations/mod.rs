use crate::cpu::register_bank::STACK_POINTER_BASE_ADDRESS;
use crate::cpu::types::CpuFlags;
use crate::cpu::Cpu;

pub trait StackOperations {
    fn pha(&mut self);
    fn php(&mut self);
    fn pla(&mut self);
    fn plp(&mut self);
}

impl StackOperations for Cpu {

    fn pha(&mut self) {
        self.memory.write(self.registers.accumulator,
            STACK_POINTER_BASE_ADDRESS + self.registers.stack_pointer as u16);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    fn php(&mut self) {
        self.memory.write(self.registers.status.bits(),
            STACK_POINTER_BASE_ADDRESS + self.registers.stack_pointer as u16);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    fn pla(&mut self) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        self.registers.accumulator = self.memory.read(STACK_POINTER_BASE_ADDRESS + self.registers.stack_pointer as u16);
        raise_flags(self, self.registers.accumulator);
    }

    fn plp(&mut self) {
        self.registers.status = CpuFlags::from_bits_truncate(self.memory.read(STACK_POINTER_BASE_ADDRESS + self.registers.stack_pointer as u16));
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
    }
}

fn raise_flags(cpu: &mut Cpu, value: u8) {
    cpu.registers.status.set(CpuFlags::ZERO, value == 0);
    cpu.registers.status.set(CpuFlags::NEGATIVE, value & 0x80 == 0x80);
}

#[cfg(test)]
mod tests;