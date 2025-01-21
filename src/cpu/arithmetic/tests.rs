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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
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
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
        Cpu::sbc);
}

#[test]
fn test_cmp_flag_carry() {
    run_arithmetic_test(
        0x02,
        0x01,
        CpuFlags::empty(),
        0x02,
        CpuFlags::CARRY,
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
        Cpu::cmp);
}

#[test]
fn test_cmp_flag_carry_and_zero() {
    run_arithmetic_test(
        0x02,
        0x02,
        CpuFlags::empty(),
        0x02,
        CpuFlags::ZERO | CpuFlags::CARRY,
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
        Cpu::cmp);
}

#[test]
fn test_cmp_flag_carry_and_negative() {
    run_arithmetic_test(
        0xff,
        0x00,
        CpuFlags::empty(),
        0xff,
        CpuFlags::NEGATIVE | CpuFlags::CARRY,
        |cpu| cpu.registers.accumulator,
        |cpu, value| cpu.registers.accumulator = value,
        Cpu::cmp);
}

#[test]
fn test_cpx_flag_carry() {
    run_arithmetic_test(
        0x02,
        0x01,
        CpuFlags::empty(),
        0x02,
        CpuFlags::CARRY,
        |cpu| cpu.registers.x_register,
        |cpu, value| cpu.registers.x_register = value,
        Cpu::cpx);
}

#[test]
fn test_cpx_flag_carry_and_zero() {
    run_arithmetic_test(
        0x02,
        0x02,
        CpuFlags::empty(),
        0x02,
        CpuFlags::ZERO | CpuFlags::CARRY,
        |cpu| cpu.registers.x_register,
        |cpu, value| cpu.registers.x_register = value,
        Cpu::cpx);
}

#[test]
fn test_cpx_flag_carry_and_negative() {
    run_arithmetic_test(
        0xff,
        0x00,
        CpuFlags::empty(),
        0xff,
        CpuFlags::NEGATIVE | CpuFlags::CARRY,
        |cpu| cpu.registers.x_register,
        |cpu, value| cpu.registers.x_register = value,
        Cpu::cpx);
}

#[test]
fn test_cpy_flag_carry() {
    run_arithmetic_test(
        0x02,
        0x01,
        CpuFlags::empty(),
        0x02,
        CpuFlags::CARRY,
        |cpu| cpu.registers.y_register,
        |cpu, value| cpu.registers.y_register = value,
        Cpu::cpy);
}

#[test]
fn test_cpy_flag_carry_and_zero() {
    run_arithmetic_test(
        0x02,
        0x02,
        CpuFlags::empty(),
        0x02,
        CpuFlags::ZERO | CpuFlags::CARRY,
        |cpu| cpu.registers.y_register,
        |cpu, value| cpu.registers.y_register = value,
        Cpu::cpy);
}

#[test]
fn test_cpy_flag_carry_and_negative() {
    run_arithmetic_test(
        0xff,
        0x00,
        CpuFlags::empty(),
        0xff,
        CpuFlags::NEGATIVE | CpuFlags::CARRY,
        |cpu| cpu.registers.y_register,
        |cpu, value| cpu.registers.y_register = value,
        Cpu::cpy);
}

fn run_arithmetic_test<F, G, H>(
    operand_1: u8,
    operand_2: u8,
    initial_status: CpuFlags,
    expected_result: u8,
    expected_status: CpuFlags,
    get_register: F,
    set_register: G,
    operation: H,
) where
    F: Fn(&Cpu) -> u8,
    G: Fn(&mut Cpu, u8),
    H: Fn(&mut Cpu, u8),
{
    // Given
    let mut cpu = Cpu::new();
    set_register(&mut cpu, operand_1);
    cpu.registers.status = initial_status;

    // When
    operation(&mut cpu, operand_2);

    // Then
    assert_eq!(get_register(&cpu), expected_result);
    assert_eq!(cpu.registers.status, expected_status);
}
