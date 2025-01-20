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
mod tests {
    use super::*;

    #[test]
    fn read_write() {
        // Given
        let address: u16 = 0x8000;
        let expected_data: u8 = 0x69;

        // When
        let mut memory = Memory::new();
        memory.write(expected_data, address);

        // Then
        assert_eq!(expected_data, memory.read(address));
    }

    #[test]
    fn read_u16() {
        // Given
        let address: u16 = 0x8000;
        let least_significant_byte: u8 = 0x69;
        let most_significant_byte: u8 = 0x96;
        let expected_data: u16 = 0x9669;

        // When
        let mut memory = Memory::new();
        memory.write(least_significant_byte, address);
        memory.write(most_significant_byte, address+1);

        // Then
        assert_eq!(expected_data, memory.read_u16(address));
    }
}
