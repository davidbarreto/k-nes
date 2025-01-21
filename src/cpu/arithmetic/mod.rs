use super::types::CpuFlags;
use super::Cpu;

pub trait Arithmetic {
    fn adc(&mut self, data: u8);
    fn sbc(&mut self, data: u8);
    fn cmp(&mut self, data: u8);
    fn cpx(&mut self, data: u8);
    fn cpy(&mut self, data: u8);
}

impl Arithmetic for Cpu {
    /// Implementation of ADC (Add with Carry) instruction
    fn adc(&mut self, data: u8) {
        let carry: u8 = if self.registers.status.contains(CpuFlags::CARRY) {1} else {0};

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

        // For overflow, we need to check if the sign bit of the accumulator and the sign bit of the data
        // are different from the sign bit of the result
        // For detailed explanation, see https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.registers.status.update_flag(
            CpuFlags::OVERFLOW,
            (self.registers.accumulator ^ result) & (data ^ result) & 0x80 != 0,
        );

        self.registers.accumulator = result;
    }

    /// Implementation of SBC (Subtract with Carry) instruction
    /// According to the docs, there is no way to subtract without the carry which works as an inverse borrow
    /// i.e, to subtract you SET THE CARRY before the operation. If the carry is cleared by the operation, it indicates a borrow occurred.
    /// So, SBC operation is the same as ADC but with the data inverted.
    /// According to the docs, the operation must do: A-M-(1-C)
    /// This is the same as  A+(-M-1+C) => A+(-M-1)+C
    /// So, we need to also remove the carry from the result, because it is an inverse borrow
    /// See [SBC docs](https://www.nesdev.org/obelisk-6502-guide/reference.html#SBC)
    fn sbc(&mut self, data: u8) {
        // SBC is the same as ADC but with the data inverted,
        // A-M-(1-C) => A+(-M-1+C) => A+(-M-1)+C
        let negative_data = data.wrapping_neg().wrapping_sub(1);
        self.adc(negative_data);
    }

    /// Implementation of CMP (Compare with Accumulator) instruction
    fn cmp(&mut self, data: u8) {
        compare(self, self.registers.accumulator, data);
    }

    /// Implementation of CPX (Compare with X Register) instruction
    fn cpx(&mut self, data: u8) {
        compare(self, self.registers.x_register, data);
    }

    /// Implementation of CPY (Compare with Y Register) instruction
    fn cpy(&mut self, data: u8) {
        compare(self, self.registers.y_register, data);
    }
}

fn compare(cpu: &mut Cpu, register: u8, data: u8) {
    cpu.registers.status.update_flag(CpuFlags::CARRY, register >= data);
    cpu.registers.status.update_flag(CpuFlags::ZERO, register == data);
    cpu.registers.status.update_flag(CpuFlags::NEGATIVE, register & 0x80 == 0x80);  
}

#[cfg(test)]
mod tests;
