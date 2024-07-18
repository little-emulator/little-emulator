use super::*;

#[test]
fn load() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to store 42 from R0 into memory
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0011_000_111111111);
    cpu.set_register(&Register::Gpr(Gpr::R0), 42);

    // Assert that the memory cell is stored correctly
    cpu.step_instruction();
    assert_eq!(cpu.get_memory(0x31FF), 42);
}

#[test]
fn store_indirect() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to store 42 from R0 into memory loading the address
    // indirectly
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b1011_000_111111111);
    cpu.set_memory(0x31FF, 0x6000);
    cpu.set_register(&Register::Gpr(Gpr::R0), 42);

    // Assert that the memory cell is stored correctly
    cpu.step_instruction();
    assert_eq!(cpu.get_memory(0x6000), 42);
}

#[test]
fn store_through_register() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to store 42 from R0 into memory loading the address
    // through R1
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0111_000_001_111111);
    cpu.set_register(&Register::Gpr(Gpr::R0), 42);
    cpu.set_register(&Register::Gpr(Gpr::R1), 0x6000);

    // Assert that the memory cell is stored correctly
    cpu.step_instruction();
    assert_eq!(cpu.get_memory(0x603F), 42);
}
