use super::*;

#[test]
fn load() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to load 42 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0010_000_111111111);
    cpu.set_memory(0x31FF, 42);

    // Assert that the memory cell is loaded correctly
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), 42);
}

#[test]
fn load_indirect() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to load 42 into R0, loading the address through memory
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b1010_000_111111111);
    cpu.set_memory(0x31FF, 0x6000);
    cpu.set_memory(0x6000, 42);

    // Assert that the memory cell is loaded correctly
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), 42);
}

#[test]
fn load_through_register() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to load 42 into R0, loading the address through R1
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0110_000_001_111111);
    cpu.set_register(&Register::Gpr(Gpr::R1), 0x6000);
    cpu.set_memory(0x603F, 42);

    // Assert that the memory cell is loaded correctly
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), 42);
}

#[test]
fn load_effective_address() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to load the address 0x31FF into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b1110_000_111111111);

    // Assert that the address is loaded correctly
    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), 0x31FF);
}
