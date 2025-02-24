use super::*;

#[test]
fn write() {
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
    let address: u16 = 0x0200;
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

#[test]
fn write_array() {
    // Given
    let address: u16 = 0x8000;
    let array: [u8; 5] = [0x69, 0x96, 0x12, 0x80, 0x0f];

    // When
    let mut memory = Memory::new();
    memory.write_array(&array, address);

    // Then
    assert_eq!(array, memory.read_as_array(address as usize));
}