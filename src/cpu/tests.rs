use super::*;

#[test]
fn execute_adc_immediate() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let initial_address: u16 = 0x8000;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(operand_2, initial_address+1);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_IMMEDIATE));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_zero_page() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let zero_page_address: u8 = 0x42;
    let initial_address: u16 = 0x8000;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_2, zero_page_address as u16);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_ZERO_PAGE));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_zero_page_x() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let zero_page_address: u8 = 0x42;
    let increment_address: u8 = 0x02;
    let operand_address: u16 = 0x0044;
    let initial_address: u16 = 0x8000;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.accumulator = operand_1;
    cpu.registers.x_register = increment_address;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_ZERO_PAGE_X));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_zero_page_x_with_high_x_value() {
    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let zero_page_address: u8 = 0x80;
    let increment_address: u8 = 0xff;
    // 0x80 + 0xff = 0x007f instead of 0x017f because of zero page addressing (wrap around)
    let operand_address: u16 = 0x007f;
    let initial_address: u16 = 0x8000;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.accumulator = operand_1;
    cpu.registers.x_register = increment_address;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_ZERO_PAGE_X));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_absolute() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let absolute_addr_least_significant_byte: u8 = 0x42;
    let absolute_addr_most_significant_byte: u8 = 0x80;
    let absolute_address: u16 = 0x8042;
    let initial_address: u16 = 0x8000;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(absolute_addr_least_significant_byte, initial_address+1);
    cpu.memory.write(absolute_addr_most_significant_byte, initial_address+2);
    cpu.memory.write(operand_2, absolute_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_ABSOLUTE));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_absolute_x() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let absolute_addr_least_significant_byte: u8 = 0x42;
    let absolute_addr_most_significant_byte: u8 = 0x80;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0x02;
    let operand_address: u16 = 0x8044;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.x_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(absolute_addr_least_significant_byte, initial_address+1);
    cpu.memory.write(absolute_addr_most_significant_byte, initial_address+2);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_ABSOLUTE_X));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_absolute_y() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let absolute_addr_least_significant_byte: u8 = 0x42;
    let absolute_addr_most_significant_byte: u8 = 0x80;
    let absolute_address: u16 = 0x8042;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0x02;
    let operand_address: u16 = absolute_address + increment_address as u16;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.y_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(absolute_addr_least_significant_byte, initial_address+1);
    cpu.memory.write(absolute_addr_most_significant_byte, initial_address+2);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_ABSOLUTE_Y));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_indirect_x() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let zero_page_address: u8 = 0x42;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0x02;
    let operand_address: u16 = 0x0044;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.x_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_INDIRECT_X));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_indirect_x_with_high_x_value() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let zero_page_address: u8 = 0x80;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0xff;
    // 0x80 + 0xff = 0x007f instead of 0x017f because of zero page addressing (wrap around)
    let operand_address: u16 = 0x007f;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.x_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_INDIRECT_X));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_indirect_y() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let zero_page_address: u8 = 0x42;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0x02;
    let operand_address: u16 = 0x0044;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.y_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_INDIRECT_Y));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn execute_adc_indirect_y_with_high_y_value() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = 0xff;
    let zero_page_address: u8 = 0x80;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0xff;
    // 0x80 + 0xff = 0x017f normally (zero page is not applied to the sum)
    let operand_address: u16 = 0x017f;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.y_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_INDIRECT_Y));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn test_example_program() {
    
    // Program test:
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a9 c0     LDA #$c0
    // $0602    aa        TAX 
    // $0603    e8        INX 
    // $0604    69 c4     ADC #$c4
    // $0606    00        BRK 
    let program: [u8; 7] = [0xA9, 0xC0, 0xAA, 0xE8, 0x69, 0xC4, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x0607);
    assert_eq!(cpu.registers.accumulator, 0x84);
    assert_eq!(cpu.registers.x_register, 0xC1);
}

#[test]
// TODO Configure test for the following program
// FIXME PC increment after each instruction should depend on instruction data size
fn test_example_program_2() {
    
    // Program test:
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a2 08     LDX #$08
    // $0602    ca        DEX 
    // $0603    8e 00 02  STX $0200
    // $0606    e0 03     CPX #$03
    // $0608    d0 f8     BNE $0602
    // $060a    8e 01 02  STX $0201
    // $060d    00        BRK
    let program: [u8; 14] = [0xA2, 0x08, 0xCA, 0x8E, 0x00, 0x02, 0xE0, 0x03, 0xD0, 0xF8, 0x8E, 0x01, 0x02, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x060E);
    assert_eq!(cpu.registers.accumulator, 0x00);
    assert_eq!(cpu.registers.x_register, 0x03);
    assert_eq!(cpu.memory.read(0x0000), 0x00);
    assert_eq!(cpu.memory.read(0x0200), 0x03);
    assert_eq!(cpu.memory.read(0x0201), 0x03);
}

fn execute_program(program: &[u8]) -> Cpu {
    let program_address: u16 = 0x0600;
    let mut cpu = Cpu::new();

    // Load program into memory, starting at 'program_address'
    cpu.memory.write_array(&program, program_address);
    cpu.registers.program_counter = program_address;
    
    if let Err(e) = cpu.execute_program() {
        println!("Got error: {}", e);
        panic!();
    }
    cpu
}

// TODO Add tests for other addressing modes