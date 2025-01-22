use super::*;

#[test]
fn test_inc() {
    test_increment_decrement(
        0x42,
        0x10,
        0x43,
        |cpu, address, value| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.inc(address),
        CpuFlags::empty());
}

#[test]
fn test_inx() {
    test_increment_decrement(
        0x42,
        0x10,
        0x43,
        |cpu, _, value| cpu.registers.x_register = value,
        |cpu, _| cpu.registers.x_register,
        |cpu, _| cpu.inx(),
        CpuFlags::empty());
}

#[test]
fn test_iny() {
    test_increment_decrement(
        0x42,
        0x10,
        0x43,
        |cpu, _, value| cpu.registers.y_register = value,
        |cpu, _| cpu.registers.y_register,
        |cpu, _| cpu.iny(),
        CpuFlags::empty());
}

#[test]
fn test_dec() {
    test_increment_decrement(
        0x42,
        0x10,
        0x41,
        |cpu, address, value| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.dec(address),
        CpuFlags::empty());
}

#[test]
fn test_dex() {
    test_increment_decrement(
        0x42,
        0x10,
        0x41,
        |cpu, _, value| cpu.registers.x_register = value,
        |cpu, _| cpu.registers.x_register,
        |cpu, _| cpu.dex(),
        CpuFlags::empty());
}

#[test]
fn test_dey() {
    test_increment_decrement(
        0x42,
        0x10,
        0x41,
        |cpu, _, value| cpu.registers.y_register = value,
        |cpu, _| cpu.registers.y_register,
        |cpu, _| cpu.dey(),
        CpuFlags::empty());
}

#[test]
fn test_inc_zero_flag() {
    test_increment_decrement(
        0xFF,
        0x10,
        0x00,
        |cpu, address, value| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.inc(address),
        CpuFlags::ZERO);
}

#[test]
fn test_inc_negative_flag() {
    test_increment_decrement(
        0x7F,
        0x10,
        0x80,
        |cpu, address, value| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.inc(address),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_dec_zero_flag() {
    test_increment_decrement(
        0x01,
        0x10,
        0x00,
        |cpu, address, value| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.dec(address),
        CpuFlags::ZERO);
}

#[test]
fn test_dec_negative_flag() {
    test_increment_decrement(
        0x00,
        0x10,
        0xFF,
        |cpu, address, value| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.dec(address),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_inx_zero_flag() {
    test_increment_decrement(
        0xFF,
        0x10,
        0x00,
        |cpu, _, value| cpu.registers.x_register = value,
        |cpu, _| cpu.registers.x_register,
        |cpu, _| cpu.inx(),
        CpuFlags::ZERO);
}

#[test]
fn test_inx_negative_flag() {
    test_increment_decrement(
        0x7F,
        0x10,
        0x80,
        |cpu, _, value| cpu.registers.x_register = value,
        |cpu, _| cpu.registers.x_register,
        |cpu, _| cpu.inx(),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_iny_zero_flag() {
    test_increment_decrement(
        0xFF,
        0x10,
        0x00,
        |cpu, _, value| cpu.registers.y_register = value,
        |cpu, _| cpu.registers.y_register,
        |cpu, _| cpu.iny(),
        CpuFlags::ZERO);
}

#[test]
fn test_iny_negative_flag() {
    test_increment_decrement(
        0x7F,
        0x10,
        0x80,
        |cpu, _, value| cpu.registers.y_register = value,
        |cpu, _| cpu.registers.y_register,
        |cpu, _| cpu.iny(),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_dex_zero_flag() {
    test_increment_decrement(
        0x01,
        0x10,
        0x00,
        |cpu, _, value| cpu.registers.x_register = value,
        |cpu, _| cpu.registers.x_register,
        |cpu, _| cpu.dex(),
        CpuFlags::ZERO);
}

#[test]
fn test_dex_negative_flag() {
    test_increment_decrement(
        0x00,
        0x10,
        0xFF,
        |cpu, _, value| cpu.registers.x_register = value,
        |cpu, _| cpu.registers.x_register,
        |cpu, _| cpu.dex(),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_dey_zero_flag() {
    test_increment_decrement(
        0x01,
        0x10,
        0x00,
        |cpu, _, value| cpu.registers.y_register = value,
        |cpu, _| cpu.registers.y_register,
        |cpu, _| cpu.dey(),
        CpuFlags::ZERO);
}

#[test]
fn test_dey_negative_flag() {
    test_increment_decrement(
        0x00,
        0x10,
        0xFF,
        |cpu, _, value| cpu.registers.y_register = value,
        |cpu, _| cpu.registers.y_register,
        |cpu, _| cpu.dey(),
        CpuFlags::NEGATIVE);
}

fn test_increment_decrement<F, G, H>(initial_value: u8, address: u8, expected_value: u8, set_value: F, get_value: G, operation: H, expected_status: CpuFlags)
where
    F: Fn(&mut Cpu, u8, u8),
    G: Fn(&Cpu, u8) -> u8,
    H: Fn(&mut Cpu, u8),
{
    // Given
    let mut cpu = Cpu::new();
    set_value(&mut cpu, address, initial_value);

    // When
    operation(&mut cpu, address);

    // Then
    assert_eq!(get_value(&cpu, address), expected_value);
    assert_eq!(cpu.registers.status, expected_status);
}