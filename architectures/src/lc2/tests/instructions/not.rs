use super::*;

#[test]
fn not() {
    let mut cpu = Lc2::new(0x3000);

    // Set the memory to negate R1 into R0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b1001_000_001_111111);
    cpu.set_register(&Register::Gpr(Gpr::R1), 42);

    cpu.step_instruction();
    assert_eq!(cpu.get_register(&Register::Gpr(Gpr::R0)), !42);
}
