use crate::cpu::types::CpuFlags;
use crate::cpu::Cpu;

pub trait Logical {
    fn and(&mut self, data: u8);
    fn ora(&mut self, data: u8);
    fn eor(&mut self, data: u8);
    fn bit(&mut self, data: u8);
}

impl Logical for Cpu {
    /// Implementation of AND (Logical AND) instruction
    /// Bitwise AND of the accumulator with the operand
    fn and(&mut self, data: u8) {

        let result = self.registers.accumulator & data;

        self.registers
            .status
            .set(CpuFlags::ZERO, result == 0);
        self.registers
            .status
            .set(CpuFlags::NEGATIVE, result & 0x80 == 0x80);

        self.registers.accumulator = result;
    }

    /// Implementation of ORA (Logical OR) instruction
    /// Bitwise OR of the accumulator with the operand
    fn ora(&mut self, data: u8) {

        let result = self.registers.accumulator | data;

        self.registers
            .status
            .set(CpuFlags::ZERO, result == 0);
        self.registers
            .status
            .set(CpuFlags::NEGATIVE, result & 0x80 == 0x80);

        self.registers.accumulator = result;
    }

    /// Implementation of EOR (Logical XOR) instruction
    /// Bitwise XOR of the accumulator with the operand
    fn eor(&mut self, data: u8) {

        let result = self.registers.accumulator ^ data;

        self.registers
            .status
            .set(CpuFlags::ZERO, result == 0);
        self.registers
            .status
            .set(CpuFlags::NEGATIVE, result & 0x80 == 0x80);

        self.registers.accumulator = result;
    }

    /// Implementation of BIT (Bit Test) instruction
    /// Tests bits in memory with the accumulator
    fn bit(&mut self, data: u8) {
        let result = self.registers.accumulator & data;

        // Update Zero flag
        self.registers
            .status
            .set(CpuFlags::ZERO, result == 0);

        // Update Negative flag (bit 7 of the operand)
        self.registers
            .status
            .set(CpuFlags::NEGATIVE, data & 0x80 == 0x80);

        // Update Overflow flag (bit 6 of the operand)
        self.registers
            .status
            .set(CpuFlags::OVERFLOW, data & 0x40 == 0x40);
    }
}

#[cfg(test)]
mod tests;
