use super::*;

#[test]
fn register_positive() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to sum R1 and R2 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_001_0_00_010);

    // Set R1 and R2 to 5
    cpu.set_register(&Register::Gpr(Gpr::R1), 5);
    cpu.set_register(&Register::Gpr(Gpr::R2), 5);

    // Check if the result is correct and if the Condition Code are updated
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), 10);
    assert_eq!(cpu.get_condition_code(), ConditionCode::Positive);
}

#[test]
fn register_negative() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to sum R1 and R2 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_001_0_00_010);

    // Set R1 and R2 to -5
    cpu.set_register(&Register::Gpr(Gpr::R1), !5 + 1);
    cpu.set_register(&Register::Gpr(Gpr::R2), !5 + 1);

    // Check if the result is correct and if the Condition Code are updated
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), !10 + 1);
    assert_eq!(cpu.get_condition_code(), ConditionCode::Negative);
}

#[test]
fn register_zero() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to sum R1 and R2 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_001_0_00_010);

    // Set R1 to 5 and R2 to -5
    cpu.set_register(&Register::Gpr(Gpr::R1), 5);
    cpu.set_register(&Register::Gpr(Gpr::R2), !5 + 1);

    // Check if the result is correct and if the Condition Code are updated
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), 0);
    assert_eq!(cpu.get_condition_code(), ConditionCode::Zero);
}

#[test]
fn register_overflow() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to sum R1 and R2 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_001_0_00_010);

    // Set R1 and R2 to 65535
    cpu.set_register(&Register::Gpr(Gpr::R1), u16::MAX);
    cpu.set_register(&Register::Gpr(Gpr::R2), u16::MAX);

    // Check if the result is correct and if the Condition Code are updated
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), u16::MAX - 1);
    assert_eq!(cpu.get_condition_code(), ConditionCode::Negative);
}

#[test]
fn immediate_positive() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to sum R1 and 5 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_001_1_00101);

    // Set R1 to 5
    cpu.set_register(&Register::Gpr(Gpr::R1), 5);

    // Check if the result is correct and if the Condition Code are updated
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), 10);
    assert_eq!(cpu.get_condition_code(), ConditionCode::Positive);
}

#[test]
fn immediate_negative() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to sum R1 and -5 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_001_1_11011);

    // Set R1 to -5
    cpu.set_register(&Register::Gpr(Gpr::R1), !5 + 1);

    // Check if the result is correct and if the Condition Code are updated
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), !10 + 1);
    assert_eq!(cpu.get_condition_code(), ConditionCode::Negative);
}
