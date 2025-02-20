use super::*;

#[test]
fn test_brk() {
    // Given
    let mut cpu = Cpu::new();

    // When
    cpu.brk();

    // Then
    assert!(cpu.registers.status.contains(CpuFlags::BREAK));
}

#[test]
fn test_rti() {
    // Given
    let mut cpu = Cpu::new();
    let expected_status = CpuFlags::CARRY | CpuFlags::ZERO | CpuFlags::OVERFLOW;
    let expected_pc: u16 = 0x810F;
    cpu.memory.write(0x81, 0x01FF);
    cpu.memory.write(0x0F, 0x01FE);
    cpu.memory.write(expected_status.bits(), 0x01FD);
    cpu.registers.stack_pointer = 0xFC;
    
    // When
    cpu.rti();

    // Then
    assert_eq!(expected_pc, cpu.registers.program_counter);
    assert_eq!(expected_status, cpu.registers.status);
}