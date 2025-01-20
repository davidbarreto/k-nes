use super::types::CpuFlags;
use super::Cpu;

pub trait Arithmetic {
    fn adc(&mut self, data: u8);
}

impl Arithmetic for Cpu {
    /// Implementation of ADC (Add with Carry) instruction
    fn adc(&mut self, data: u8) {
        let carry: u8 = if self.registers.status.contains(CpuFlags::CARRY) {
            1
        } else {
            0
        };

        let (result, carry_1) = self.registers.accumulator.overflowing_add(data);
        let (result, carry_2) = result.overflowing_add(carry);

        self.registers
            .status
            .update_flag(CpuFlags::CARRY, carry_1 || carry_2);
        self.registers
            .status
            .update_flag(CpuFlags::ZERO, result == 0);
        self.registers
            .status
            .update_flag(CpuFlags::NEGATIVE, result & 0x80 == 0x80);
        self.registers.status.update_flag(
            CpuFlags::OVERFLOW,
            (self.registers.accumulator ^ result) & (data ^ result) & 0x80 != 0,
        );

        self.registers.accumulator = result;
    }
}

#[cfg(test)]
mod tests;
