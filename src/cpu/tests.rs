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
    let indirect_address: u16 = 0x0044;
    let operand_address_lsb: u8 = 0x09;
    let operand_address_msb: u8 = 0x07;
    let operand_address: u16 = 0x0709;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.x_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_address_lsb, indirect_address);
    cpu.memory.write(operand_address_msb, indirect_address+1);
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
    let expected_result: u8 = 0xFF;
    let zero_page_address: u8 = 0x80;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0xff;
    // 0x80 + 0xff = 0x007f instead of 0x017f because of zero page addressing (wrap around)
    let indirect_address: u16 = 0x007F;
    let operand_address_lsb: u8 = 0x09;
    let operand_address_msb: u8 = 0x07;
    let operand_address: u16 = 0x0709;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.x_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_address_lsb, indirect_address);
    cpu.memory.write(operand_address_msb, indirect_address+1);
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
    let expected_result: u8 = 0xFF;
    let zero_page_address: u8 = 0x42;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0x02;
    let indirect_address: u16 = 0x0042;
    let operand_address_lsb: u8 = 0x09;
    let operand_address_msb: u8 = 0x07;
    let operand_address: u16 = 0x070B;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.y_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_address_lsb, indirect_address);
    cpu.memory.write(operand_address_msb, indirect_address+1);
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
    let expected_result: u8 = 0xFF;
    let zero_page_address: u8 = 0x80;
    let initial_address: u16 = 0x8000;
    let increment_address: u8 = 0xFF;
    let indirect_address: u16 = 0x0080;
    let operand_address_lsb: u8 = 0x09;
    let operand_address_msb: u8 = 0x07;
    let operand_address: u16 = 0x0808;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.y_register = increment_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(zero_page_address, initial_address+1);
    cpu.memory.write(operand_address_lsb, indirect_address);
    cpu.memory.write(operand_address_msb, indirect_address+1);
    cpu.memory.write(operand_2, operand_address);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_INDIRECT_Y));
    assert_eq!(cpu.registers.accumulator, expected_result);
}

#[test]
fn test_calculate_address_absolute() {
    // Given
    let expected_value: u16 = 0x1020;
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x8000;
    cpu.memory.write(0x20, 0x8000);
    cpu.memory.write(0x10, 0x8001);

    // When
    let result = cpu.calculate_address(AddressingMode::Absolute);

    // Then
    assert_eq!(expected_value, result);
}

#[test]
fn test_calculate_address_absolute_x() {
    // Given
    let expected_value: u16 = 0x102A;
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x0A;
    cpu.memory.write(0x20, 0x8000);
    cpu.memory.write(0x10, 0x8001);

    // When
    let result = cpu.calculate_address(AddressingMode::AbsoluteX);

    // Then
    assert_eq!(expected_value, result);
}

#[test]
fn test_calculate_address_absolute_y() {
    // Given
    let expected_value: u16 = 0x102A;
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x0A;
    cpu.memory.write(0x20, 0x8000);
    cpu.memory.write(0x10, 0x8001);

    // When
    let result = cpu.calculate_address(AddressingMode::AbsoluteY);

    // Then
    assert_eq!(expected_value, result);
}

#[test]
fn test_calculate_address_immediate() {
    // Given
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x8000;
    
    // When
    let result = cpu.calculate_address(AddressingMode::Immediate);

    // Then
    assert_eq!(cpu.registers.program_counter, result);
}

#[test]
fn test_calculate_address_indirect() {
    // Given
    let expected_value = 0x01FF;
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x8000;
    
    cpu.memory.write(0x20, 0x8000);
    cpu.memory.write(0x10, 0x8001);

    cpu.memory.write(0xFF, 0x1020);
    cpu.memory.write(0x01, 0x1021);
    
    // When
    let result = cpu.calculate_address(AddressingMode::Indirect);

    // Then
    assert_eq!(expected_value, result);
}

#[test]
fn test_calculate_address_indirect_x() {
    // Given
    let expected_value = 0x10;
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x8000;
    cpu.registers.x_register = 0x0A;
    
    cpu.memory.write(0x20, 0x8000);
    cpu.memory.write(0x10, 0x002A);
    
    // When
    let result = cpu.calculate_address(AddressingMode::IndirectX);

    // Then
    assert_eq!(expected_value, result);
}

#[test]
fn test_calculate_address_indirect_y() {
    // Given
    let expected_value = 0x301A;
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x8000;
    cpu.registers.y_register = 0x0A;
    
    cpu.memory.write(0x20, 0x8000);
    cpu.memory.write(0x10, 0x0020);
    cpu.memory.write(0x30, 0x0021);
    
    // When
    let result = cpu.calculate_address(AddressingMode::IndirectY);

    // Then
    assert_eq!(expected_value, result);
}

#[test]
fn test_calculate_address_zero_page() {
    // Given
    let expected_value = 0x10;
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x8000;
    
    cpu.memory.write(0x20, 0x8000);
    cpu.memory.write(0x10, 0x0020);
    
    // When
    let result = cpu.calculate_address(AddressingMode::IndirectY);

    // Then
    assert_eq!(expected_value, result);
}

#[test]
fn test_easy_6502_our_first_program() {

    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a9 01     LDA #$01
    // $0602    8d 00 02  STA $0200
    // $0605    a9 05     LDA #$05
    // $0607    8d 01 02  STA $0201
    // $060a    a9 08     LDA #$08
    // $060c    8d 02 02  STA $0202

    let program: [u8; 16] = [0xA9, 0x01, 0x8D, 0x00, 0x02, 0xA9, 0x05, 0x8D, 0x01, 0x02, 0xA9, 0x08, 0x8D, 0x02, 0x02, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x0610);
    assert_eq!(cpu.registers.accumulator, 0x08);
    assert_eq!(cpu.memory.read(0x0200), 0x01);
    assert_eq!(cpu.memory.read(0x0201), 0x05);
    assert_eq!(cpu.memory.read(0x0202), 0x08);
}

#[test]
fn test_easy_6502_instructions() {
    
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
fn test_easy_6502_instructions_2() {
    
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a9 80     LDA #$80
    // $0602    85 01     STA $01
    // $0604    65 01     ADC $01
    let program: [u8; 7] = [0xA9, 0x80, 0x85, 0x01, 0x65, 0x01, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x0607);
    // $80 + $80 = 100 => wrapped = 00
    assert_eq!(cpu.registers.accumulator, 0x00);
}

#[test]
fn test_easy_6502_branching() {
    
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

#[test]
fn test_easy_6502_addressing_modes_relative() {
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a9 01     LDA #$01
    // $0602    c9 02     CMP #$02
    // $0604    d0 02     BNE $0608
    // $0606    85 22     STA $22
    // $0608    00        BRK 

    let program: [u8; 9] = [0xA9, 0x01, 0xC9, 0x02, 0xD0, 0x02, 0x85, 0x22, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x0609);
    assert_eq!(cpu.registers.accumulator, 0x01);
    assert_eq!(cpu.memory.read(0x0022), 0x00);
}

#[test]
fn test_easy_6502_addressing_modes_indirect() {
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a9 01     LDA #$01
    // $0602    85 f0     STA $f0
    // $0604    a9 cc     LDA #$cc
    // $0606    85 f1     STA $f1
    // $0608    6c f0 00  JMP ($00f0)

    // As memory is initialized with 0's, address $CC01 has 00, which is interpreted as BRK
    let program: [u8; 11] = [0xA9, 0x01, 0x85, 0xF0, 0xA9, 0xCC, 0x85, 0xF1, 0x6C, 0xF0, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0xCC02);
    assert_eq!(cpu.registers.accumulator, 0xCC);
    assert_eq!(cpu.memory.read_u16(0x00F0), 0xCC01);
}

#[test]
fn test_easy_6502_addressing_modes_indexed_indirect() {
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a2 01     LDX #$01
    // $0602    a9 05     LDA #$05
    // $0604    85 01     STA $01
    // $0606    a9 07     LDA #$07
    // $0608    85 02     STA $02
    // $060a    a0 0a     LDY #$0a
    // $060c    8c 05 07  STY $0705
    // $060f    a1 00     LDA ($00,X)
    let program: [u8; 18] = [0xA2, 0x01, 0xA9, 0x05, 0x85, 0x01, 0xA9, 0x07, 0x85, 0x02, 0xA0, 0x0A, 0x8C, 0x05, 0x07, 0xA1, 0x00, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x0612);
    assert_eq!(cpu.registers.accumulator, 0x0A);
    assert_eq!(cpu.registers.x_register, 0x01);
    assert_eq!(cpu.registers.y_register, 0x0A);
    assert_eq!(cpu.memory.read(0x0000), 0x00);
    assert_eq!(cpu.memory.read(0x0001), 0x05);
    assert_eq!(cpu.memory.read(0x0002), 0x07);
    assert_eq!(cpu.memory.read(0x0705), 0x0A);
}

#[test]
fn test_easy_6502_addressing_modes_indirect_indexed() {
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a0 01     LDY #$01
    // $0602    a9 03     LDA #$03
    // $0604    85 01     STA $01
    // $0606    a9 07     LDA #$07
    // $0608    85 02     STA $02
    // $060a    a2 0a     LDX #$0a
    // $060c    8e 04 07  STX $0704
    // $060f    b1 01     LDA ($01),Y
    let program: [u8; 18] = [0xA0, 0x01, 0xA9, 0x03, 0x85, 0x01, 0xA9, 0x07, 0x85, 0x02, 0xA2, 0x0A, 0x8E, 0x04, 0x07, 0xB1, 0x01, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x0612);
    assert_eq!(cpu.registers.accumulator, 0x0A);
    assert_eq!(cpu.registers.x_register, 0x0A);
    assert_eq!(cpu.registers.y_register, 0x01);
    assert_eq!(cpu.memory.read(0x0000), 0x00);
    assert_eq!(cpu.memory.read(0x0001), 0x03);
    assert_eq!(cpu.memory.read(0x0002), 0x07);
    assert_eq!(cpu.memory.read(0x0704), 0x0A);
}

#[test]
fn test_easy_6502_the_stack() {
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a2 00     LDX #$00
    // $0602    a0 00     LDY #$00
    // $0604    8a        TXA 
    // $0605    99 00 02  STA $0200,Y
    // $0608    48        PHA 
    // $0609    e8        INX 
    // $060a    c8        INY 
    // $060b    c0 10     CPY #$10
    // $060d    d0 f5     BNE $0604
    // $060f    68        PLA 
    // $0610    99 00 02  STA $0200,Y
    // $0613    c8        INY 
    // $0614    c0 20     CPY #$20
    // $0616    d0 f7     BNE $060f
    let program: [u8; 25] = [0xA2, 0x00, 0xA0, 0x00, 0x8A, 0x99, 0x00, 0x02, 0x48, 0xE8, 0xC8, 0xC0, 0x10, 0xD0, 0xF5, 0x68, 0x99, 0x00, 0x02, 0xC8, 0xC0, 0x20, 0xD0, 0xF7, 0x00];
    let cpu = execute_program(&program);
    let mut expected_array: [u8; 16] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xA, 0xB, 0xC, 0xD, 0x0E, 0x0F];

    assert_eq!(cpu.registers.program_counter, 0x0619);
    assert_eq!(cpu.registers.accumulator, 0x00);
    assert_eq!(cpu.registers.x_register, 0x10);
    assert_eq!(cpu.registers.y_register, 0x20);
    assert_eq!(cpu.registers.stack_pointer, 0xFF);
    assert_eq!(cpu.memory.read_as_array(0x0200), expected_array);
    expected_array.reverse();
    assert_eq!(cpu.memory.read_as_array(0x01F0), expected_array);
    assert_eq!(cpu.memory.read_as_array(0x0210), expected_array);
}

#[test]
fn test_easy_6502_jumping_jmp() {
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    a9 03     LDA #$03
    // $0602    4c 08 06  JMP $0608
    // $0605    00        BRK 
    // $0606    00        BRK 
    // $0607    00        BRK 
    // $0608    8d 00 02  STA $0200
    let program: [u8; 12] = [0xA9, 0x03, 0x4C, 0x08, 0x06, 0x00, 0x00, 0x00, 0x8D, 0x00, 0x02, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x060C);
    assert_eq!(cpu.registers.accumulator, 0x03);
    assert_eq!(cpu.memory.read(0x0200), 0x03);
}

#[test]
fn test_easy_6502_jumping_jsr_rts() {
    // Address  Hexdump   Dissassembly
    // -------------------------------
    // $0600    20 09 06  JSR $0609
    // $0603    20 0c 06  JSR $060c
    // $0606    20 12 06  JSR $0612
    // $0609    a2 00     LDX #$03
    // $060b    60        RTS 
    // $060c    e8        INX 
    // $060d    e0 05     CPX #$05
    // $060f    d0 fb     BNE $060c
    // $0611    60        RTS 
    // $0612    00        BRK 
    let program: [u8; 19] = [0x20, 0x09, 0x06, 0x20, 0x0C, 0x06, 0x20, 0x12, 0x06, 0xA2, 0x03, 0x60, 0xE8, 0xE0, 0x05, 0xD0, 0xFB, 0x60, 0x00];
    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x0613);
    assert_eq!(cpu.registers.accumulator, 0x00);
    assert_eq!(cpu.registers.x_register, 0x05);
    assert_eq!(cpu.registers.y_register, 0x00);
    assert_eq!(cpu.registers.stack_pointer, 0xFD);
    assert_eq!(cpu.memory.read(0x0000), 0x00);
    assert_eq!(cpu.memory.read(0x01FE), 0x08);
    assert_eq!(cpu.memory.read(0x01FF), 0x06);
}

fn execute_program(program: &[u8]) -> Cpu {
    let program_address: u16 = 0x0600;
    let mut cpu = Cpu::new();

    // Load program into memory, starting at 'program_address'
    cpu.memory.write_array(&program, program_address);
    cpu.registers.program_counter = program_address;
    
    if let Err(e) = cpu.execute_program() {
        println!("Got error: {} in address {:#06x}", e, cpu.registers.program_counter);
        panic!();
    }
    cpu
}