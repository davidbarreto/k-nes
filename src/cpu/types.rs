use bitflags::bitflags;

#[derive(Debug, PartialEq)]
pub enum InstructionError {
    InvalidOpcode(u8),
}

bitflags! {
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const UNUSED            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

impl CpuFlags {
    pub fn update_flag(&mut self, flag: CpuFlags, condition: bool) {
        if condition {
            self.insert(flag);
        } else {
            self.remove(flag);
        }
    }
}
