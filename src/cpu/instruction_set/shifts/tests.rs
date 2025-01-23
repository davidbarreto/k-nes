use super::*;
#[test]
fn test_asl() {
    test_shift(
        0x02,
        0x10,
        0x4,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.asl(address),
        CpuFlags::empty());
}

#[test]
fn test_asl_flag_carry() {
    test_shift(
        0x81,
        0x10,
        0x02,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.asl(address),
        CpuFlags::CARRY);
}

#[test]
fn test_asl_flag_zero() {
    test_shift(
        0x00,
        0x10,
        0x00,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.asl(address),
        CpuFlags::ZERO);
}

#[test]
fn test_asl_flag_negative() {
    test_shift(
        0x40,
        0x10,
        0x80,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.asl(address),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_asl_accumulator() {
    test_shift(
        0x02,
        0x10,
        0x04,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.asl_accumulator(),
        CpuFlags::empty());
}

#[test]
fn test_asl_accumulator_flag_carry() {
    test_shift(
        0x81,
        0x10,
        0x02,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.asl_accumulator(),
        CpuFlags::CARRY);
}

#[test]
fn test_asl_accumulator_flag_zero() {
    test_shift(
        0x00,
        0x10,
        0x00,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.asl_accumulator(),
        CpuFlags::ZERO);
}

#[test]
fn test_asl_accumulator_flag_negative() {
    test_shift(
        0x40,
        0x10,
        0x80,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.asl_accumulator(),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_lsr() {
    test_shift(
        0x02,
        0x10,
        0x01,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.lsr(address),
        CpuFlags::empty());
}

#[test]
fn test_lsr_flag_carry() {
    test_shift(
        0x03,
        0x10,
        0x01,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.lsr(address),
        CpuFlags::CARRY);
}

#[test]
fn test_lsr_flag_zero() {
    test_shift(
        0x00,
        0x10,
        0x00,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.lsr(address),
        CpuFlags::ZERO);
}

#[test]
fn test_lsr_accumulator() {
    test_shift(
        0x02,
        0x10,
        0x01,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.lsr_accumulator(),
        CpuFlags::empty());
}

#[test]
fn test_lsr_accumulator_flag_carry() {
    test_shift(
        0x03,
        0x10,
        0x01,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.lsr_accumulator(),
        CpuFlags::CARRY);
}

#[test]
fn test_lsr_accumulator_flag_zero() {
    test_shift(
        0x00,
        0x10,
        0x00,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.lsr_accumulator(),
        CpuFlags::ZERO);
}

#[test]
fn test_rol() {
    test_shift(
        0x01,
        0x10,
        0x02,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.rol(address),
        CpuFlags::empty());
}

#[test]
fn test_rol_with_carry() {
    test_shift(
        0x01,
        0x10,
        0x03,
        CpuFlags::CARRY,
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.rol(address),
        CpuFlags::empty());
}

#[test]
fn test_rol_flag_carry() {
    test_shift(
        0x81,
        0x10,
        0x02,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.rol(address),
        CpuFlags::CARRY);
}

#[test]
fn test_rol_flag_zero() {
    test_shift(
        0x00,
        0x10,
        0x00,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.rol(address),
        CpuFlags::ZERO);
}

#[test]
fn test_rol_flag_negative() {
    test_shift(
        0x40,
        0x10,
        0x80,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.rol(address),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_rol_accumulator() {
    test_shift(
        0x01,
        0x10,
        0x02,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.rol_accumulator(),
        CpuFlags::empty());
}

#[test]
fn test_rol_accumulator_flag_carry() {
    test_shift(
        0x81,
        0x10,
        0x02,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.rol_accumulator(),
        CpuFlags::CARRY);
}

#[test]
fn test_rol_accumulator_flag_zero() {
    test_shift(
        0x00,
        0x10,
        0x00,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.rol_accumulator(),
        CpuFlags::ZERO);
}

#[test]
fn test_rol_accumulator_flag_negative() {
    test_shift(
        0x40,
        0x10,
        0x80,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.rol_accumulator(),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_ror() {
    test_shift(
        0x02,
        0x10,
        0x01,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.ror(address),
        CpuFlags::empty());
}

#[test]
fn test_ror_with_carry() {
    test_shift(
        0x00,
        0x10,
        0x80,
        CpuFlags::CARRY,
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.ror(address),
        CpuFlags::NEGATIVE);
}

#[test]
fn test_ror_flag_carry() {
    test_shift(
        0x05,
        0x10,
        0x02,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.ror(address),
        CpuFlags::CARRY);
}

#[test]
fn test_ror_flag_zero() {
    test_shift(
        0x00,
        0x10,
        0x00,
        CpuFlags::empty(),
        |cpu, value, address| cpu.memory.write(value, address as u16),
        |cpu, address| cpu.memory.read(address as u16),
        |cpu, address| cpu.ror(address),
        CpuFlags::ZERO);
}

#[test]
fn test_ror_accumulator() {
    test_shift(
        0x4,
        0x10,
        0x02,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.ror_accumulator(),
        CpuFlags::empty());
}

#[test]
fn test_ror_accumulator_flag_carry() {
    test_shift(
        0x3,
        0x10,
        0x01,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.ror_accumulator(),
        CpuFlags::CARRY);
}

#[test]
fn test_ror_accumulator_flag_zero() {
    test_shift(
        0x0,
        0x10,
        0x00,
        CpuFlags::empty(),
        |cpu, value, _| cpu.registers.accumulator = value,
        |cpu, _| cpu.registers.accumulator,
        |cpu, _| cpu.ror_accumulator(),
        CpuFlags::ZERO);
}

fn test_shift<F, G, H>(initial_value: u8, address: u8, expected_value: u8, initial_status: CpuFlags, set_value: F, get_value: G, operation: H, expected_status: CpuFlags) 
where 
    F: Fn(&mut Cpu, u8, u8),
    G: Fn(&mut Cpu, u8) -> u8,
    H: Fn(&mut Cpu, u8)
{
    // Given
    let mut cpu = Cpu::new();
    cpu.registers.status = initial_status;
    set_value(&mut cpu, initial_value, address);

    // When
    operation(&mut cpu, address);

    // Then
    assert_eq!(get_value(&mut cpu, address), expected_value);
    assert_eq!(cpu.registers.status, expected_status);
}