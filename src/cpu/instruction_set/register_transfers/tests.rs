use super::*;

#[test]
fn test_tax() {
    test_transfer(
        0x42,
        CpuFlags::empty(),
        |cpu, value| cpu.registers.accumulator = value,
        |cpu| cpu.registers.x_register,
        Cpu::tax);
}

#[test]
fn test_tax_zero_flag() {
    test_transfer(
        0x00,
        CpuFlags::ZERO,
        |cpu, value| cpu.registers.accumulator = value,
        |cpu| cpu.registers.x_register,
        Cpu::tax);
}

#[test]
fn test_tax_negative_flag() {
    test_transfer(
        0x80,
        CpuFlags::NEGATIVE,
        |cpu, value| cpu.registers.accumulator = value,
        |cpu| cpu.registers.x_register,
        Cpu::tax);
}

#[test]
fn test_tay() {
    test_transfer(
        0x42,
        CpuFlags::empty(),
        |cpu, value| cpu.registers.accumulator = value,
        |cpu| cpu.registers.y_register,
        Cpu::tay);
}

#[test]
fn test_tay_zero_flag() {
    test_transfer(
        0x00,
        CpuFlags::ZERO,
        |cpu, value| cpu.registers.accumulator = value,
        |cpu| cpu.registers.y_register,
        Cpu::tay);
}

#[test]
fn test_tay_negative_flag() {
    test_transfer(
        0x80,
        CpuFlags::NEGATIVE,
        |cpu, value| cpu.registers.accumulator = value,
        |cpu| cpu.registers.y_register,
        Cpu::tay);
}

#[test]
fn test_txa() {
    test_transfer(
        0x42,
        CpuFlags::empty(),
        |cpu, value| cpu.registers.x_register = value,
        |cpu| cpu.registers.accumulator,
        Cpu::txa);
}

#[test]
fn test_txa_zero_flag() {
    test_transfer(
        0x00,
        CpuFlags::ZERO,
        |cpu, value| cpu.registers.x_register = value,
        |cpu| cpu.registers.accumulator,
        Cpu::txa);
}

#[test]
fn test_txa_negative_flag() {
    test_transfer(
        0x80,
        CpuFlags::NEGATIVE,
        |cpu, value| cpu.registers.x_register = value,
        |cpu| cpu.registers.accumulator,
        Cpu::txa);
}

#[test]
fn test_tya() {
    test_transfer(
        0x42,
        CpuFlags::empty(),
        |cpu, value| cpu.registers.y_register = value,
        |cpu| cpu.registers.accumulator,
        Cpu::tya);
}

#[test]
fn test_tya_zero_flag() {
    test_transfer(
        0x00,
        CpuFlags::ZERO,
        |cpu, value| cpu.registers.y_register = value,
        |cpu| cpu.registers.accumulator,
        Cpu::tya);
}

#[test]
fn test_tya_negative_flag() {
    test_transfer(
        0x80,
        CpuFlags::NEGATIVE,
        |cpu, value| cpu.registers.y_register = value,
        |cpu| cpu.registers.accumulator,
        Cpu::tya);
}

#[test]
fn test_tsx() {
    test_transfer(
        0x42,
        CpuFlags::empty(),
        |cpu, value| cpu.registers.stack_pointer = value,
        |cpu| cpu.registers.x_register,
        Cpu::tsx);
}

#[test]
fn test_tsx_zero_flag() {
    test_transfer(
        0x00,
        CpuFlags::ZERO,
        |cpu, value| cpu.registers.stack_pointer = value,
        |cpu| cpu.registers.x_register,
        Cpu::tsx);
}

#[test]
fn test_tsx_negative_flag() {
    test_transfer(
        0x80,
        CpuFlags::NEGATIVE,
        |cpu, value| cpu.registers.stack_pointer = value,
        |cpu| cpu.registers.x_register,
        Cpu::tsx);
}

#[test]
fn test_txs() {
    test_transfer(
        0x42,
        CpuFlags::empty(),
        |cpu, value| cpu.registers.x_register = value,
        |cpu| cpu.registers.stack_pointer,
        Cpu::txs);
}

#[test]
fn test_txs_zero_flag() {
    test_transfer(
        0x00,
        CpuFlags::ZERO,
        |cpu, value| cpu.registers.x_register = value,
        |cpu| cpu.registers.stack_pointer,
        Cpu::txs);
}

#[test]
fn test_txs_negative_flag() {
    test_transfer(
        0x80,
        CpuFlags::NEGATIVE,
        |cpu, value| cpu.registers.x_register = value,
        |cpu| cpu.registers.stack_pointer,
        Cpu::txs);
}

fn test_transfer<F, G>(data: u8, status: CpuFlags, set_source_register: F, get_destination_register: G, operation: fn(&mut Cpu)) 
where
    F: Fn(&mut Cpu, u8),
    G: Fn(&Cpu) -> u8
{
    // Given
    let mut cpu = Cpu::new();
    set_source_register(&mut cpu, data);

    // When
    operation(&mut cpu);

    // Then
    assert_eq!(get_destination_register(&mut cpu), data);
    assert_eq!(cpu.registers.status, status);
}
