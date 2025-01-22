use super::*;

#[test]
fn test_bcc_do_branch_with_positive_displacement() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::empty(),
        0x0043,
        Cpu::bcc);
}

#[test]
fn test_bcc_do_branch_with_negative_displacement() {
    test_branch(
        0x0042,
        0xff,
        CpuFlags::empty(),
        0x0041,
        Cpu::bcc);
}

#[test]
fn test_bcc_do_not_branch() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::CARRY,
        0x0042,
        Cpu::bcc);
}

#[test]
fn test_bcs_do_branch_with_positive_displacement() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::CARRY,
        0x0043,
        Cpu::bcs);
}

#[test]
fn test_bcs_do_branch_with_negative_displacement() {
    test_branch(
        0x0042,
        0xff,
        CpuFlags::CARRY,
        0x0041,
        Cpu::bcs);
}

#[test]
fn test_bcs_do_not_branch() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::empty(),
        0x0042,
        Cpu::bcs);
}

#[test]
fn test_beq_do_branch_with_positive_displacement() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::ZERO,
        0x0043,
        Cpu::beq);
}

#[test]
fn test_beq_do_branch_with_negative_displacement() {
    test_branch(
        0x0042,
        0xff,
        CpuFlags::ZERO,
        0x0041,
        Cpu::beq);
}

#[test]
fn test_beq_do_not_branch() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::empty(),
        0x0042,
        Cpu::beq);
}

#[test]
fn test_bmi_do_branch_with_positive_displacement() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::NEGATIVE,
        0x0043,
        Cpu::bmi);
}

#[test]
fn test_bmi_do_branch_with_negative_displacement() {
    test_branch(
        0x0042,
        0xff,
        CpuFlags::NEGATIVE,
        0x0041,
        Cpu::bmi);
}

#[test]
fn test_bmi_do_not_branch() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::empty(),
        0x0042,
        Cpu::bmi);
}

#[test]
fn test_bne_do_branch_with_positive_displacement() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::empty(),
        0x0043,
        Cpu::bne);
}

#[test]
fn test_bne_do_branch_with_negative_displacement() {
    test_branch(
        0x0042,
        0xff,
        CpuFlags::empty(),
        0x0041,
        Cpu::bne);
}

#[test]
fn test_bne_do_not_branch() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::ZERO,
        0x0042,
        Cpu::bne);
}

#[test]
fn test_bpl_do_branch_with_positive_displacement() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::empty(),
        0x0043,
        Cpu::bpl);
}

#[test]
fn test_bpl_do_branch_with_negative_displacement() {
    test_branch(
        0x0042,
        0xff,
        CpuFlags::empty(),
        0x0041,
        Cpu::bpl);
}

#[test]
fn test_bpl_do_not_branch() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::NEGATIVE,
        0x0042,
        Cpu::bpl);
}

#[test]
fn test_bvc_do_branch_with_positive_displacement() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::empty(),
        0x0043,
        Cpu::bvc);
}

#[test]
fn test_bvc_do_branch_with_negative_displacement() {
    test_branch(
        0x0042,
        0xff,
        CpuFlags::empty(),
        0x0041,
        Cpu::bvc);
}

#[test]
fn test_bvc_do_not_branch() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::OVERFLOW,
        0x0042,
        Cpu::bvc);
}

#[test]
fn test_bvs_do_branch_with_positive_displacement() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::OVERFLOW,
        0x0043,
        Cpu::bvs);
}

#[test]
fn test_bvs_do_branch_with_negative_displacement() {
    test_branch(
        0x0042,
        0xff,
        CpuFlags::OVERFLOW,
        0x0041,
        Cpu::bvs);
}

#[test]
fn test_bvs_do_not_branch() {
    test_branch(
        0x0042,
        0x01,
        CpuFlags::empty(),
        0x0042,
        Cpu::bvs);
}

fn test_branch(initial_pc: u16, displacement: u8, status: CpuFlags, expected_pc: u16, operation: fn(&mut Cpu, u8)) {
    // Given
    let mut cpu = Cpu::new();
    cpu.registers.status = status;
    cpu.registers.program_counter = initial_pc;

    // When
    operation(&mut cpu, displacement);

    // Then
    assert_eq!(cpu.registers.program_counter, expected_pc);
}