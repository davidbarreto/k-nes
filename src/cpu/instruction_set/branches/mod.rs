use crate::cpu::types::CpuFlags;
use crate::cpu::Cpu;

pub trait Branches {
    fn bcc(&mut self, data: u8);
    fn bcs(&mut self, data: u8);
    fn beq(&mut self, data: u8);
    fn bmi(&mut self, address: u8);
    fn bne(&mut self, address: u8);
    fn bpl(&mut self, address: u8);
    fn bvc(&mut self, address: u8);
    fn bvs(&mut self, address: u8);
}

impl Branches for Cpu {
    fn bcc(&mut self, data: u8) {
        branch_if_condition(self, data, !self.registers.status.contains(CpuFlags::CARRY));
    }

    fn bcs(&mut self, data: u8) {
        branch_if_condition(self, data, self.registers.status.contains(CpuFlags::CARRY));
    }

    fn beq(&mut self, data: u8) {
        branch_if_condition(self, data, self.registers.status.contains(CpuFlags::ZERO));
    }

    fn bmi(&mut self, data: u8) {
        branch_if_condition(self, data, self.registers.status.contains(CpuFlags::NEGATIVE));
    }

    fn bne(&mut self, data: u8) {
        branch_if_condition(self, data, !self.registers.status.contains(CpuFlags::ZERO));
    }

    fn bpl(&mut self, data: u8) {
        branch_if_condition(self, data, !self.registers.status.contains(CpuFlags::NEGATIVE));
    }

    fn bvc(&mut self, data: u8) {
        branch_if_condition(self, data, !self.registers.status.contains(CpuFlags::OVERFLOW));
    }

    fn bvs(&mut self, data: u8) {
        branch_if_condition(self, data, self.registers.status.contains(CpuFlags::OVERFLOW));
    }
}

fn branch_if_condition(cpu: &mut Cpu, data: u8, condition: bool) {
    let displacement = data as i8;
    if condition {
        cpu.registers.program_counter = cpu.registers.program_counter.wrapping_add(displacement as u16);
    }
}

#[cfg(test)]
mod tests;