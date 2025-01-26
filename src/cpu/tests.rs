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

#[test]
fn test_subroutines() {
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

#[test]
/// This tests uses a program that calculates week day
/// Reference: (Day of Week)[http://www.6502.org/source/misc/dow.htm]
/// How to compute the day of the week in 6502 assembly.
/// By Paul Guertin (pg@sff.net), 18 August 2000.
fn test_day_of_week() {
    
    // This routine works for any date from 1900-03-01 to 2155-12-31.
    // No range checking is done, so validate input before calling.
    //
    // I use the formula
    //     Weekday = (day + offset[month] + year + year/4 + fudge) mod 7
    // where the value of fudge depends on the century.
    //
    // Input: Y = year (0=1900, 1=1901, ..., 255=2155)
    //        X = month (1=Jan, 2=Feb, ..., 12=Dec)
    //        A = day (1 to 31)
    //
    // Output: Weekday in A (0=Sunday, 1=Monday, ..., 6=Saturday)
    //
    // Given input: 1988-07-28
    // Expected output: Thursday (4)
    //
    // PROGRAM:
    // TMP      EQU $6          ; Temporary storage

    // WEEKDAY:
    //          CPX #3          ; Year starts in March to bypass
    //          BCS MARCH       ; leap year problem
    //          DEY             ; If Jan or Feb, decrement year
    // MARCH    EOR #$7F        ; Invert A so carry works right
    //          CPY #200        ; Carry will be 1 if 22nd century
    //          ADC MTAB-1,X    ; A is now day+month offset
    //          STA TMP
    //          TYA             ; Get the year
    //          JSR MOD7        ; Do a modulo to prevent overflow
    //          SBC TMP         ; Combine with day+month
    //          STA TMP
    //          TYA             ; Get the year again
    //          LSR             ; Divide it by 4
    //          LSR
    //          CLC             ; Add it to y+m+d and fall through
    //          ADC TMP
    // MOD7     ADC #7          ; Returns (A+3) modulo 7
    //          BCC MOD7        ; for A in 0..255
    //          RTS
    // MTAB     DB 1,5,6,3,1,5,3,0,4,2,6,4   	; Month offsets

    let program: [u8; 91] = 
    [
        // Write MTAB values into memory starting at $0600
        0xA9, 0x01,       // LDA #$01 (First MTAB value)
        0x85, 0x20,       // STA $0020
        0xA9, 0x05,       // LDA #$05
        0x85, 0x21,       // STA $0021
        0xA9, 0x06,       // LDA #$06
        0x85, 0x22,       // STA $0022
        0xA9, 0x03,       // LDA #$03
        0x85, 0x23,       // STA $0023
        0xA9, 0x01,       // LDA #$01
        0x85, 0x24,       // STA $0024
        0xA9, 0x05,       // LDA #$05
        0x85, 0x25,       // STA $0025
        0xA9, 0x03,       // LDA #$03
        0x85, 0x26,       // STA $0026
        0xA9, 0x00,       // LDA #$00
        0x85, 0x27,       // STA $0027
        0xA9, 0x04,       // LDA #$04
        0x85, 0x28,       // STA $0028
        0xA9, 0x02,       // LDA #$02
        0x85, 0x29,       // STA $0029
        0xA9, 0x06,       // LDA #$06
        0x85, 0x2A,       // STA $002A
        0xA9, 0x04,       // LDA #$04
        0x85, 0x2B,       // STA $002B

        // Write TMP
        0xA9, 0x06,       // LDA #$06
        0x85, 0x06,       // STA $0006

        // Write INPUT
        0xA0, 0xBC,       // LDY #$BC (Year = 1988 - 1900 = 88)
        0xA2, 0x07,       // LDX #$07 (Month = July = 7)
        0xA9, 0x1C,       // LDA #$1C (Day = 28)

        // Main program
        0xE0, 0x03,       // CPX #$03
        0xB0, 0x01,       // BCS MARCH
        0x88,             // DEY
        0x49, 0x7F,       // EOR #$7F -- MARCH
        0xC0, 0xC8,       // CPY #$C8
        0x75, 0x20,       // ADC MTAB-1,X
        0x85, 0x06,       // STA $06
        0x98,             // TYA
        0x20, 0x26, 0x06, // JSR MOD7
        0xE5, 0x06,       // SBC $06
        0x85, 0x06,       // STA $06
        0x98,             // TYA
        0x4A,             // LSR
        0x4A,             // LSR
        0x18,             // CLC
        0x65, 0x06,       // ADC $06
        00,               // BRK - End of program

        // MOD7 Subroutine
        0x69, 0x07,       // ADC #$07 - MOD7
        0x90, 0xFD,       // BCC MOD7
        0x60              // RTS
    ];

    let cpu = execute_program(&program);

    assert_eq!(cpu.registers.program_counter, 0x060E);
    assert_eq!(cpu.registers.accumulator, 0x04); // Answer should be Thursday

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

// TODO Add tests for other addressing modes