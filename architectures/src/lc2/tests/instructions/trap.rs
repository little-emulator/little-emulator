use super::*;

#[test]
fn trap() {
    let mut cpu = Lc2::new(0x3000);

    // Setup the Trap Vector
    for i in 0..0xff {
        cpu.set_memory(i, !i);
    }

    // For every addres in the Trap Vector...
    for i in 0..0xff {
        // Setup the CPU to trap vector element
        cpu.set_register(&Register::ProgramCounter, 0x3000);
        cpu.set_memory(0x3000, 0xf000 + i);

        // Assert that the jump to the trap address has happened
        cpu.step_instruction();
        assert_eq!(cpu.get_register(&Register::ProgramCounter), !i);
    }
}

#[test]
fn return_from_trap() {
    let mut cpu = Lc2::new(0x3000);

    // Setup the memory to trap to address 0x6000 and return immediatly
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b1111_0000_00101010);
    cpu.set_memory(42, 0x6000);
    cpu.set_memory(0x6000, 0b1101_000000000000);

    // Assert that the Program Counter is updated accordingly
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x6000);
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::ProgramCounter), 0x3001);
}
