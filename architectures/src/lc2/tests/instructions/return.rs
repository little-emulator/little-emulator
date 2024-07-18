use super::*;

#[test]
fn r#return() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to jump to 0x300A and immediatly return
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0100_1_00_000001010); // JSR 0x300A
    cpu.set_memory(0x300A, 0b1101_000000000000); // RET

    // Assert that the Program Counter is back on the main routine
    cpu.step_instruction();
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x3001);
}

#[test]
fn return_from_interrupt() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to return from an interrupt and set the stack to have a
    // negative Condition Code and 0x6000 as the return address
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b1000_000000000000);
    cpu.set_register(&Register::Gpr(Gpr::R6), 0x300B);
    cpu.set_memory(0x300B, 0x8000);
    cpu.set_memory(0x300A, 0x6000);

    // Asserts that the code returns succesfully from an interrupt
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x6000);
    assert_eq!(cpu.get_condition_code(), ConditionCode::Negative);
}
