use super::*;

#[test]
fn nop() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to never branch
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0000_000_000001010);

    // Run the test cases
    run_test_cases(
        &mut cpu,
        &[
            (ConditionCode::Negative, 0x3001),
            (ConditionCode::Zero, 0x3001),
            (ConditionCode::Positive, 0x3001),
        ],
    );
}

#[test]
fn branch_n() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to branch to 0x300a if the `negative` condition
    // code is set
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0000_100_000001010);

    // Run the test cases
    run_test_cases(
        &mut cpu,
        &[
            (ConditionCode::Negative, 0x300A),
            (ConditionCode::Zero, 0x3001),
            (ConditionCode::Positive, 0x3001),
        ],
    );
}

#[test]
fn branch_z() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to branch to 0x300a if the `zero` condition code
    // is set
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0000_010_000001010);

    // Run the test cases
    run_test_cases(
        &mut cpu,
        &[
            (ConditionCode::Negative, 0x3001),
            (ConditionCode::Zero, 0x300A),
            (ConditionCode::Positive, 0x3001),
        ],
    );
}

#[test]
fn branch_p() {
    let mut cpu = Lc2::new(0x3000);

    // Set the 0x3000 address to branch to 0x300a if the `positive` condition
    // code is set
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0000_001_000001010);

    // Run the test cases
    run_test_cases(
        &mut cpu,
        &[
            (ConditionCode::Negative, 0x3001),
            (ConditionCode::Zero, 0x3001),
            (ConditionCode::Positive, 0x300A),
        ],
    );
}

fn run_test_cases(cpu: &mut Lc2, test_cases: &[(ConditionCode, u16)]) {
    // For each test case...
    for (condition_code, address) in test_cases {
        // Set the Program Counter back to 0x3000
        cpu.set_register(&Register::ProgramCounter, 0x3000);

        // Setup the Condition Code
        cpu.set_condition_code(condition_code);

        // Run the instruction and check if the jump has happened
        cpu.step_instruction();
        assert_eq!(cpu.get_register(&Register::ProgramCounter), *address);
    }
}
