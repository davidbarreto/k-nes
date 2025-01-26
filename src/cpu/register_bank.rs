use super::types::CpuFlags;

pub const STACK_POINTER_BASE_ADDRESS: u16 = 0x0100;

pub struct RegisterBank {
    pub status: CpuFlags,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
}

impl RegisterBank {
    pub fn new() -> Self {
        Self {
            status: CpuFlags::empty(),
            program_counter: 0,
            stack_pointer: 0xFF,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
        }
    }
}