use super::*;

#[test]
fn test_jump() {
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x1000;
    cpu.jmp(0x2000);
    assert_eq!(cpu.registers.program_counter, 0x2000);
}

#[test]
fn test_call() {
    let mut cpu = Cpu::new();
    cpu.registers.program_counter = 0x1000;
    cpu.registers.stack_pointer = 0xFF;
    cpu.jsr(0x2000, 3);
    assert_eq!(cpu.registers.program_counter, 0x2000);
    assert_eq!(cpu.registers.stack_pointer, 0xFD);
    assert_eq!(cpu.memory.read(0x01FF), 0x10);
    assert_eq!(cpu.memory.read(0x01FE), 0x01);
}

#[test]
fn test_ret() {
    let mut cpu = Cpu::new();
    cpu.registers.stack_pointer = 0xFD;
    cpu.memory.write(0x02,0x01FE);
    cpu.memory.write(0x10,0x01FF);
    cpu.rts();
    assert_eq!(cpu.registers.program_counter, 0x1002);
    assert_eq!(cpu.registers.stack_pointer, 0xFF);
}
