use crate::cpu::Cpu;
use crate::cpu::register_bank::STACK_POINTER_BASE_ADDRESS;

pub trait JumpsCalls {
    fn jmp(&mut self, address: u16);
    fn jsr(&mut self, address: u16, bytes_to_skip: u8);
    fn rts(&mut self);
}

impl JumpsCalls for Cpu {
    fn jmp(&mut self, address: u16) {
        self.registers.program_counter = address;
    }

    fn jsr(&mut self, address: u16, bytes_to_skip: u8) {
        // The address_to_push is equal the current PC + size of instruction (bytes to skip) - 2
        // The idea is to discount the instruction byte (by this time, instruction was already read and PC incremented)
        // and also one more byte to point to next instruction -1 (according to specs)
        // So, in practice, we are going to point to the last byte of the current instruction
        let address_to_push: u16 = self.registers.program_counter + bytes_to_skip as u16 - 2;
        let address_stack_top = STACK_POINTER_BASE_ADDRESS + (self.registers.stack_pointer as u16);
        let address_stack_top_minus_1 = STACK_POINTER_BASE_ADDRESS + (self.registers.stack_pointer.wrapping_sub(1) as u16);
        self.memory.write(((address_to_push & 0xFF00) >> 8) as u8, address_stack_top);
        self.memory.write((address_to_push & 0x00FF) as u8, address_stack_top_minus_1);
        // Update SP (we already stored 2 bytes onto the stack)
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(2);
        self.registers.program_counter = address;
    }

    fn rts(&mut self) {
        let address = STACK_POINTER_BASE_ADDRESS + (self.registers.stack_pointer.wrapping_add(1) as u16);
        self.registers.program_counter = self.memory.read_u16(address);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(2);
    }
}

#[cfg(test)]
mod tests;