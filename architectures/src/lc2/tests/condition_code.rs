use super::*;

use crate::WatcherType;
use std::rc::Rc;
use std::sync::atomic::{AtomicU16, Ordering};

#[test]
fn get_and_set() {
    let mut cpu = Lc2::new(0x3000);

    cpu.set_condition_code(&ConditionCode::from(!5 + 1));
    assert_eq!(cpu.get_condition_code(), ConditionCode::Negative);

    cpu.set_condition_code(&ConditionCode::from(0));
    assert_eq!(cpu.get_condition_code(), ConditionCode::Zero);

    cpu.set_condition_code(&ConditionCode::from(5));
    assert_eq!(cpu.get_condition_code(), ConditionCode::Positive);
}

#[test]
fn set_through_gpr() {
    let mut cpu = Lc2::new(0x3000);

    // In order:
    //  - Add 1 to R0 to make the Condition Code positive
    //  - And R0 with 0 to make the Condition Code zero
    //  - Add -1 to R0 to make the Condition Code negative
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_000_1_00001); // ADD R0, R0, #1
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3001, 0b0101_000_000_1_00000); // AND R0, R0, #0
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3002, 0b0001_000_000_1_11111); // ADD R0, R0, #-1

    cpu.step_instruction();
    assert_eq!(cpu.get_condition_code(), ConditionCode::Positive);

    cpu.step_instruction();
    assert_eq!(cpu.get_condition_code(), ConditionCode::Zero);

    cpu.step_instruction();
    assert_eq!(cpu.get_condition_code(), ConditionCode::Negative);
}

#[test]
fn watcher_on_write() {
    // Create a new LC2 and an atomic u16 to store the watcher results
    let mut cpu = Lc2::new(0x3000);
    let value = Rc::new(AtomicU16::new(0));

    // Create a watcher that transforms the Condition Code into a u16
    let value_watcher = value.clone();
    cpu.add_condition_code_watcher(WatcherType::OnWrite, move |new_value| {
        value_watcher.store(u16::from(new_value), Ordering::Relaxed);
    });

    // Change the state of the Condition Code
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_000_1_00001); // ADD R0, R0, #1

    // Check if the watcher has been called
    cpu.step_instruction();
    assert_eq!(value.load(Ordering::Relaxed), 1);

    // Remove the watcher and assert that nothing changes
    cpu.remove_condition_code_watcher(WatcherType::OnWrite);
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3001, 0b0001_000_000_1_11110); // ADD R0, R0, #-2
    cpu.step_instruction();
    assert_eq!(value.load(Ordering::Relaxed), 1);
}

#[test]
fn watcher_on_read() {
    // Create a new LC2 and an atomic u16 to store the watcher results
    let mut cpu = Lc2::new(0x3000);
    let value = Rc::new(AtomicU16::new(0));

    // Create a watcher that transforms the Condition Code into a u16
    let value_watcher = value.clone();
    cpu.add_condition_code_watcher(WatcherType::OnRead, move |new_value| {
        value_watcher.store(u16::from(new_value), Ordering::Relaxed);
    });

    // Set the Condition Code
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3000, 0b0001_000_000_1_00001); // ADD R0, R0, #1

    // Check that the watcher has NOT been called
    cpu.step_instruction();
    assert_eq!(value.load(Ordering::Relaxed), 0);

    // Get the Condition Code
    let _ = cpu.get_condition_code();

    // Check that the watcher has been called
    assert_eq!(value.load(Ordering::Relaxed), 1);

    // Remove the watcher and assert that nothing changes
    cpu.remove_condition_code_watcher(WatcherType::OnRead);
    #[allow(clippy::unusual_byte_groupings)]
    cpu.set_memory(0x3001, 0b0001_000_000_1_11110); // ADD R0, R0, #-2
    cpu.step_instruction();
    let _ = cpu.get_condition_code();
    assert_eq!(value.load(Ordering::Relaxed), 1);
}
