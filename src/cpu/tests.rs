use super::*;

#[test]
fn execute_instruction() {

    // Given
    let operand_1: u8 = 0x69;
    let operand_2: u8 = 0x96;
    let expected_result: u8 = operand_1 + operand_2;
    let initial_address: u16 = 0x8000;

    // When
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = initial_address;
    cpu.registers.accumulator = operand_1;
    cpu.memory.write(operand_2, initial_address+1);

    // Then
    assert_eq!(Ok(()), cpu.execute_instruction(opcode::ADC_WITH_IMMEDIATE));
    assert_eq!(cpu.registers.accumulator, expected_result);
}