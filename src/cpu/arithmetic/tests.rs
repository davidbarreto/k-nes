use super::Arithmetic;
use super::Cpu;
use super::CpuFlags;

#[test]
fn test_adc_no_carry() {
    run_arithmetic_test(
        0x02,
        0x01,
        CpuFlags::empty(),
        0x03,
        CpuFlags::empty(),
        Cpu::adc);
}

#[test]
fn test_adc_with_carry() {
    run_arithmetic_test(
        0x02,
        0x01,
        CpuFlags::CARRY,
        0x04,
        CpuFlags::empty(),
        Cpu::adc);
}

#[test]
fn test_adc_flag_carry() {
    run_arithmetic_test(
        0xc8,
        0x64,
        CpuFlags::empty(),
        0x2c,
        CpuFlags::CARRY,
        Cpu::adc);
}

#[test]
fn test_adc_flag_overflow_and_negative() {
    run_arithmetic_test(
        0x50,
        0x50,
        CpuFlags::empty(),
        0xa0,
        CpuFlags::OVERFLOW | CpuFlags::NEGATIVE,
        Cpu::adc);
}

#[test]
fn test_adc_flag_overflow_and_carry() {
    run_arithmetic_test(
        0xd0,
        0x90,
        CpuFlags::empty(),
        0x60,
        CpuFlags::OVERFLOW | CpuFlags::CARRY,
        Cpu::adc);
}

#[test]
fn test_adc_flag_negative() {
    run_arithmetic_test(
        0x80,
        0x1,
        CpuFlags::empty(),
        0x81,
        CpuFlags::NEGATIVE,
        Cpu::adc);
}

#[test]
fn test_adc_flag_zero() {
    run_arithmetic_test(
        0x00,
        0x00,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO,
        Cpu::adc);
}

#[test]
fn test_adc_flag_zero_and_carry() {
    run_arithmetic_test(
        0x01,
        0xff,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO | CpuFlags::CARRY,
        Cpu::adc);
}

#[test]
fn test_sbc_with_carry() {
    run_arithmetic_test(
        0x02,
        0x01,
        CpuFlags::CARRY,
        0x01,
        CpuFlags::CARRY,
        Cpu::sbc);
}

#[test]
fn test_sbc_with_carry_clean_flag_carry_set_flag_negative() {
    run_arithmetic_test(
        0x01,
        0x02,
        CpuFlags::CARRY,
        0xff,
        CpuFlags::NEGATIVE,
        Cpu::sbc);
}

fn run_arithmetic_test<F>(operand_1: u8, operand_2: u8, initial_status: CpuFlags, expected_result: u8, expected_status: CpuFlags, operation: F)
where F: Fn(&mut Cpu, u8)
{
    // Given
    let mut cpu = Cpu::new();
    cpu.registers.accumulator = operand_1;
    cpu.registers.status = initial_status;

    // When
    operation(&mut cpu, operand_2);

    // Then
    assert_eq!(cpu.registers.accumulator, expected_result);
    assert_eq!(cpu.registers.status, expected_status);
}
