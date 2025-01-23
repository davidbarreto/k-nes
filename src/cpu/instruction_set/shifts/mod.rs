use crate::cpu::types::CpuFlags;
use crate::cpu::Cpu;

pub trait Shifts {
    fn asl(&mut self, address: u8);
    fn asl_accumulator(&mut self);
    fn lsr(&mut self, address: u8);
    fn lsr_accumulator(&mut self);
    fn rol(&mut self, address: u8);
    fn rol_accumulator(&mut self);
    fn ror(&mut self, address: u8);
    fn ror_accumulator(&mut self);
}

impl Shifts for Cpu {
    fn asl(&mut self, address: u8) {
        let val = self.memory.read(address as u16);
        let has_carry = val & 0x80 != 0;
        let result = val << 1;
        self.memory.write(result, address as u16);
        update_flags(result, has_carry, self);
    }

    fn asl_accumulator(&mut self) {
        let has_carry = self.registers.accumulator & 0x80 != 0;
        let result = self.registers.accumulator << 1;
        self.registers.accumulator = result;
        update_flags(result, has_carry, self);
    }

    fn lsr(&mut self, address: u8) {
        let val = self.memory.read(address as u16);
        let has_carry = val & 0x01 != 0;
        let result = val >> 1;
        self.memory.write(result, address as u16);
        update_flags(result, has_carry, self);
    }

    fn lsr_accumulator(&mut self) {
        let has_carry = self.registers.accumulator & 0x01 != 0;
        let result = self.registers.accumulator >> 1;
        self.registers.accumulator = result;
        update_flags(result, has_carry, self);
    }

    fn rol(&mut self, address: u8) {
        let val = self.memory.read(address as u16);
        let has_carry = val & 0x80 != 0;
        let shifted = val << 1;
        let result = if self.registers.status.contains(CpuFlags::CARRY) { shifted | 0x01 } else { shifted };
        self.memory.write(result, address as u16);
        update_flags(result, has_carry, self);
    }

    fn rol_accumulator(&mut self) {
        let has_carry = self.registers.accumulator & 0x80 != 0;
        let shifted = self.registers.accumulator << 1;
        let result = if self.registers.status.contains(CpuFlags::CARRY) { shifted | 0x01 } else { shifted };
        self.registers.accumulator = result;
        update_flags(result, has_carry, self);
    }

    fn ror(&mut self, address: u8) {
        let val = self.memory.read(address as u16);
        let has_carry = val & 0x01 != 0;
        let shifted = val >> 1;
        let result = if self.registers.status.contains(CpuFlags::CARRY) { shifted | 0x80 } else { shifted };
        self.memory.write(result, address as u16);
        update_flags(result, has_carry, self);
    }

    fn ror_accumulator(&mut self) {
        let has_carry = self.registers.accumulator & 0x01 != 0;
        let shifted = self.registers.accumulator >> 1;
        let result = if self.registers.status.contains(CpuFlags::CARRY) { shifted | 0x80 } else { shifted };
        self.registers.accumulator = result;
        update_flags(result, has_carry, self);
    }
}

fn update_flags(val: u8, has_carry: bool, cpu: &mut Cpu) {
    cpu.registers.status.set(CpuFlags::CARRY, has_carry);
    cpu.registers.status.set(CpuFlags::ZERO, val == 0);
    cpu.registers.status.set(CpuFlags::NEGATIVE, val & 0x80 != 0);
}

#[cfg(test)]
mod tests;
