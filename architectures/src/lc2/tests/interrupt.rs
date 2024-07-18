use super::*;

#[test]
fn r#return() {
    let mut cpu = Lc2::new(0x3000);

    // Setup the interrupt routine, the stack pointer and the condition code
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x6000, 0b1000_000000000000);
    cpu.set_register(&Register::Gpr(Gpr::R6), 0x5000);
    cpu.set_condition_code(&ConditionCode::Negative);

    // Initialize the interrupt
    cpu.interrupt(0x6000);

    // Assert that the routine returns correctly
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x3000);
    assert_eq!(cpu.get_condition_code(), ConditionCode::Negative);
}
