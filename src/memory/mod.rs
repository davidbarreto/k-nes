pub mod types;

const MEMORY_SIZE: usize = 0x0800;

pub struct Memory {
    mem: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mem[self.map_address(address)]
    }

    /// Uses Little Endian (LE) approach to retrieve 2 bytes from memory as a value
    pub fn read_u16(&self, address: u16) -> u16 {
        let lsb = self.read(address) as u16;
        let msb = self.read(address + 1) as u16;

        msb << 8 | lsb
    }
    
    pub fn write(&mut self, data: u8, address: u16) {
        self.mem[self.map_address(address)] = data;
    }

    fn map_address(&self, address: u16) -> usize {
        match address {
            0x0000..=0x1FFF => self.mirror_down(address, 0x0800),
            0x2000..=0x3FFF => 0, // TODO Handle PPU registers
            0x4000..=0x4017 => 0, // TODO Handle APU IO Registers
            0x4020..=0xFFFF => 0, // TODO Handle ROM
            _ => 0
        }
    }

    fn mirror_down(&self, address: u16, size: u16) -> usize {
        (address % size) as usize
    }

    // Following functions are used only in tests, so no need to use address mappings
    // The idea is just to facilitate memory manipulation in tests

    #[cfg(test)]
    pub fn read_as_array<const N: usize>(&self, address: usize) -> [u8; N] {
        let mut array = [0u8; N];
        array.copy_from_slice(&self.mem[address..address + N]);
        array
    }

    #[cfg(test)]
    pub fn write_array(&mut self, data: &[u8], address: u16) {
        for (i, &byte) in data.iter().enumerate() {
            self.mem[address as usize + i] = byte;
        }
    }
}

#[cfg(test)]
mod tests;