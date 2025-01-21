use super::Logical;
use super::Cpu;
use super::CpuFlags;

#[test]
fn test_and_flag_zero() {
    run_logical_test(
        0x01,
        0x00,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO,
        Cpu::and,
    );
}

#[test]
fn test_and_flag_negative() {
    run_logical_test(
        0x81,
        0xfe,
        CpuFlags::empty(),
        0x80,
        CpuFlags::NEGATIVE,
        Cpu::and,
    );
}

#[test]
fn test_eor_flag_zero() {
    run_logical_test(
        0xff,
        0xff,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO,
        Cpu::eor,
    );
}

#[test]
fn test_eor_flag_negative() {
    run_logical_test(
        0x80,
        0x01,
        CpuFlags::empty(),
        0x81,
        CpuFlags::NEGATIVE,
        Cpu::eor,
    );
}

#[test]
fn test_bit_flag_zero() {
    run_logical_test(
        0x00,
        0x00,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO,
        Cpu::bit,
    );
}

#[test]
fn test_bit_flag_negative() {
    run_logical_test(
        0x80,
        0x80,
        CpuFlags::empty(),
        0x80,
        CpuFlags::NEGATIVE,
        Cpu::bit,
    );
}

#[test]
fn test_bit_flag_overflow() {
    run_logical_test(
        0x41,
        0x40,
        CpuFlags::empty(),
        0x41,
        CpuFlags::OVERFLOW,
        Cpu::bit,
    );
}

#[test]
fn test_ora_flag_zero() {
    run_logical_test(
        0x00,
        0x00,
        CpuFlags::empty(),
        0x00,
        CpuFlags::ZERO,
        Cpu::ora,
    );
}

#[test]
fn test_ora_flag_negative() {
    run_logical_test(
        0x80,
        0x01,
        CpuFlags::empty(),
        0x81,
        CpuFlags::NEGATIVE,
        Cpu::ora,
    );
}

fn run_logical_test<F>(operand_1: u8, operand_2: u8, initial_status: CpuFlags, expected_result: u8, expected_status: CpuFlags, operation: F)
where
    F: Fn(&mut Cpu, u8),
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

// TODO - Implement tests for ORA, EOR, and BIT