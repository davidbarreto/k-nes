pub mod types;

const MEMORY_SIZE: usize = 0x10000;

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
        self.mem[address as usize]
    }

    /// Uses Little Endian (LE) approach to retrieve 2 bytes from memory as a value
    pub fn read_u16(&self, address: u16) -> u16 {
        let lsb = self.read(address) as u16;
        let msb = self.read(address + 1) as u16;

        msb << 8 | lsb
    }

    pub fn write(&mut self, data: u8, address: u16) {
        self.mem[address as usize] = data;
    }

    pub fn write_array(&mut self, data: &[u8], address: u16) {
        for (i, &byte) in data.iter().enumerate() {
            self.write(byte, address + i as u16);
        }
    }
}

#[cfg(test)]
mod tests;