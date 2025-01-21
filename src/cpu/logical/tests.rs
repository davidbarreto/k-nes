use super::Logical;
use super::Cpu;
use super::CpuFlags;

#[test]
fn test_and() {
    run_and_test(
        0x03,
        0x01,
        CpuFlags::empty(),
        0x01,
        CpuFlags::empty());
}

#[test]
fn test_and_flag_zero() {
    run_and_test(
        0x02,
        0x01,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO);
}

#[test]
fn test_and_flag_negative() {
    run_and_test(
        0x81,
        0xfe,
        CpuFlags::empty(),
        0x80,
        CpuFlags::NEGATIVE);
}

fn run_and_test(operand_1: u8, operand_2: u8, initial_status: CpuFlags, expected_result: u8, expected_status: CpuFlags) {
    // Given
    let mut cpu = Cpu::new();
    cpu.registers.accumulator = operand_1;
    cpu.registers.status = initial_status;

    // When
    cpu.and(operand_2);

    // Then
    assert_eq!(cpu.registers.accumulator, expected_result);
    assert_eq!(cpu.registers.status, expected_status);
}

// TODO - Implement tests for ORA, EOR, and BIT