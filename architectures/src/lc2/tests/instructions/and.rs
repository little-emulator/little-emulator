use super::*;

#[test]
fn register() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to and R1 and R2 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0101_000_001_0_00_010);

    // Set R1 and R2
    cpu.set_register(&Register::Gpr(Gpr::R1), 0b0000_0000_1111_1111);
    cpu.set_register(&Register::Gpr(Gpr::R2), 0b0000_1111_0000_1111);

    // Check if the result is correct and if the Condition Code are updated
    cpu.step_instruction();
    assert_eq!(
        cpu.get_register(&Register::Gpr(Gpr::R0)),
        0b0000_0000_0000_1111
    );
    assert_eq!(cpu.get_condition_code(), ConditionCode::Positive);
}

#[test]
fn immediate() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to and R1 and 0b01111 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0101_000_001_1_01111);

    // Set R1 to 5
    cpu.set_register(&Register::Gpr(Gpr::R1), 0b0000_1111_0000_1111);

    // Check if the result is correct and if the Condition Code are updated
    cpu.step_instruction();
    assert_eq!(
        cpu.get_register(&Register::Gpr(Gpr::R0)),
        0b0000_0000_0000_1111
    );
    assert_eq!(cpu.get_condition_code(), ConditionCode::Positive);
}
