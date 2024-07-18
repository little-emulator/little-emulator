use super::*;

#[test]
fn jump() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to jump to 0x300A without saving the Program
    // Counter to R7
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0100_0_00_000001010);

    // Run the instruction and check if the jump has happened
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x300A);
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R7)), 0x0000);
}

#[test]
fn jump_to_subroutine() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to jump to 0x300A saving the Program Counter to R7
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0100_1_00_000001010);

    // Run the instruction and check if the jump has happened
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x300A);
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R7)), 0x3001);
}

#[test]
fn jump_through_register() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to jump to the value of R0 + 0xB (0x300A) without
    // saving the Program Counter to R7
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b1100_0_00_000_111111);
    cpu.set_register(&Register::Gpr(Gpr::R0), 0x2fcb);

    // Run the instruction and check if the jump has happened
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x300A);
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R7)), 0x0000);
}

#[test]
fn jump_to_subroutine_through_register() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to jump to the value of R0 + 0xB (0x300A) without
    // saving the Program Counter to R7
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b1100_1_00_000_111111);
    cpu.set_register(&Register::Gpr(Gpr::R0), 0x2fcb);

    // Run the instruction and check if the jump has happened
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x300A);
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R7)), 0x3001);
}
