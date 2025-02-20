use super::*;

#[test]
fn test_pha() {
    // Given
    let mut cpu = Cpu::new();
    cpu.registers.accumulator = 0x80;

    // When
    cpu.pha();

    // Then
    assert_eq!(cpu.memory.read(0x01FF), 0x80);
    assert_eq!(cpu.registers.stack_pointer, 0xFE);
}

#[test]
fn test_php() {
    // Given
    let mut cpu = Cpu::new();
    cpu.registers.status = CpuFlags::BREAK | CpuFlags::CARRY;

    // When
    cpu.php();

    // Then
    assert_eq!(cpu.memory.read(0x01FF), cpu.registers.status.bits());
    assert_eq!(cpu.registers.stack_pointer, 0xFE);
}

#[test]
fn test_pla() {
    // Given
    let mut cpu = Cpu::new();
    let expected_accumulator: u8 = 0x81;
    cpu.memory.write(expected_accumulator, 0x01FF);
    cpu.registers.stack_pointer = 0xFE;

    // When
    cpu.pla();

    // Then
    assert_eq!(expected_accumulator, cpu.registers.accumulator);
    assert_eq!(cpu.registers.stack_pointer, 0xFF);
    assert!(cpu.registers.status.contains(CpuFlags::NEGATIVE));
    assert!(!cpu.registers.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_plp() {
    // Given
    let mut cpu = Cpu::new();
    let expected_status = CpuFlags::BREAK | CpuFlags::CARRY;
    cpu.memory.write(expected_status.bits(), 0x01FF);
    cpu.registers.stack_pointer = 0xFE;

    // When
    cpu.plp();

    // Then
    assert_eq!(cpu.memory.read(0x01FF), cpu.registers.status.bits());
    assert_eq!(cpu.registers.stack_pointer, 0xFF);
}