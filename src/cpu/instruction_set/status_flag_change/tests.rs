use super::*;

#[test]
fn test_clear_carry_flag() {
    let mut cpu = Cpu::new();
    cpu.clc();
    assert!(!cpu.registers.status.contains(CpuFlags::CARRY));
}

#[test]
fn test_clear_decimal_mode_flag() {
    let mut cpu = Cpu::new();
    cpu.cld();
    assert!(!cpu.registers.status.contains(CpuFlags::DECIMAL_MODE));
}

#[test]
fn test_clear_interrupt_disable_flag() {
    let mut cpu = Cpu::new();
    cpu.cli();
    assert!(!cpu.registers.status.contains(CpuFlags::INTERRUPT_DISABLE));
}

#[test]
fn test_clear_overflow_flag() {
    let mut cpu = Cpu::new();
    cpu.clv();
    assert!(!cpu.registers.status.contains(CpuFlags::OVERFLOW));
}

#[test]
fn test_set_carry_flag() {
    let mut cpu = Cpu::new();
    cpu.sec();
    assert!(cpu.registers.status.contains(CpuFlags::CARRY));
}

#[test]
fn test_set_decimal_mode_flag() {
    let mut cpu = Cpu::new();
    cpu.sed();
    assert!(cpu.registers.status.contains(CpuFlags::DECIMAL_MODE));
}

#[test]
fn test_set_interrupt_disable_flag() {
    let mut cpu = Cpu::new();
    cpu.sei();
    assert!(cpu.registers.status.contains(CpuFlags::INTERRUPT_DISABLE));
}