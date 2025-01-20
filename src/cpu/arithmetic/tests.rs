use super::Arithmetic;
use super::Cpu;
use super::CpuFlags;

#[test]
fn test_adc_no_carry() {
    run_adc_test(
        0x02,
        0x01,
        CpuFlags::empty(),
        0x03,
        CpuFlags::empty());
}

#[test]
fn test_adc_with_carry() {
    run_adc_test(
        0x02,
        0x01,
        CpuFlags::CARRY,
        0x04,
        CpuFlags::empty());
}

#[test]
fn test_adc_flag_carry() {
    run_adc_test(
        0xc8,
        0x64,
        CpuFlags::empty(),
        0x2c,
        CpuFlags::CARRY);
}

#[test]
fn test_adc_flag_overflow_and_negative() {
    run_adc_test(
        0x50,
        0x50,
        CpuFlags::empty(),
        0xa0,
        CpuFlags::OVERFLOW | CpuFlags::NEGATIVE);
}

#[test]
fn test_adc_flag_overflow_and_carry() {
    run_adc_test(
        0xd0,
        0x90,
        CpuFlags::empty(),
        0x60,
        CpuFlags::OVERFLOW | CpuFlags::CARRY);
}

#[test]
fn test_adc_flag_negative() {
    run_adc_test(
        0x80,
        0x1,
        CpuFlags::empty(),
        0x81,
        CpuFlags::NEGATIVE);
}

#[test]
fn test_adc_flag_zero() {
    run_adc_test(
        0x00,
        0x00,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO);
}

#[test]
fn test_adc_flag_zero_and_carry() {
    run_adc_test(
        0x01,
        0xff,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO | CpuFlags::CARRY);
}

fn run_adc_test(operand_1: u8, operand_2: u8, initial_status: CpuFlags, expected_result: u8, expected_status: CpuFlags) {
    // Given
    let mut cpu = Cpu::new();
    cpu.registers.accumulator = operand_1;
    cpu.registers.status = initial_status;

    // When
    cpu.adc(operand_2);

    // Then
    assert_eq!(cpu.registers.accumulator, expected_result);
    assert_eq!(cpu.registers.status, expected_status);
}
