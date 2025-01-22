use super::*;

#[test]
fn test_load_accumulator() {
    run_load_test(
        0x01,
        |cpu| cpu.registers.accumulator,
        Cpu::lda,
        CpuFlags::empty());
}

#[test]
fn test_load_accumulator_flag_negative() {
    run_load_test(
        0xff,
        |cpu| cpu.registers.accumulator,
        Cpu::lda,
        CpuFlags::NEGATIVE);
}

#[test]
fn test_load_accumulator_flag_zero() {
    run_load_test(
        0x00,
        |cpu| cpu.registers.accumulator,
        Cpu::lda,
        CpuFlags::ZERO);
}

#[test]
fn test_load_x_register() {
    run_load_test(
        0x42,
        |cpu| cpu.registers.x_register,
        Cpu::ldx,
        CpuFlags::empty());
}

#[test]
fn test_load_x_register_flag_negative() {
    run_load_test(
        0xf0,
        |cpu| cpu.registers.x_register,
        Cpu::ldx,
        CpuFlags::NEGATIVE);
}

#[test]
fn test_load_x_register_flag_zero() {
    run_load_test(
        0x00,
        |cpu| cpu.registers.x_register,
        Cpu::ldx,
        CpuFlags::ZERO);
}

#[test]
fn test_load_y_register() {
    run_load_test(
        0x0D,
        |cpu| cpu.registers.y_register,
        Cpu::ldy,
        CpuFlags::empty());
}

#[test]
fn test_load_y_register_flag_negative() {
    run_load_test(
        0x80,
        |cpu| cpu.registers.y_register,
        Cpu::ldy,
        CpuFlags::NEGATIVE);
}

#[test]
fn test_load_y_register_flag_zero() {
    run_load_test(
        0x00,
        |cpu| cpu.registers.y_register,
        Cpu::ldy,
        CpuFlags::ZERO);
}

#[test]
fn test_store_accumulator() {
    run_store_test(
        0x42,
        0x80,
        |cpu, value| cpu.registers.accumulator = value,
        Cpu::sta);
}

#[test]
fn test_store_x_register() {
    run_store_test(
        0x42,
        0x80,
        |cpu, value| cpu.registers.x_register = value,
        Cpu::stx);
}

#[test]
fn test_store_y_register() {
    run_store_test(
        0x42,
        0x80,
        |cpu, value| cpu.registers.y_register = value,
        Cpu::sty);
}

fn run_store_test<F, G>(data: u8, address: u8, set_value: F, operation: G)
where
    F: Fn(&mut Cpu, u8),
    G: Fn(&mut Cpu, u8)
{
    // Given
    let mut cpu = Cpu::new();
    set_value(&mut cpu, data);

    // When
    operation(&mut cpu, address);

    // Then
    assert_eq!(cpu.memory.read(address as u16), data);
}

fn run_load_test<F, G>(data: u8, get_value: F, operation: G, expected_status: CpuFlags)
where
    F: Fn(&Cpu) -> u8,
    G: Fn(&mut Cpu, u8)
{
    // Given
    let mut cpu = Cpu::new();

    // When
    operation(&mut cpu, data);

    // Then
    assert_eq!(get_value(&cpu), data);
    assert_eq!(cpu.registers.status, expected_status);
}