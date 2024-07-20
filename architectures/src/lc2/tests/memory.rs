use super::*;

use std::rc::Rc;
use std::sync::atomic::{AtomicU16, Ordering};

#[test]
fn get_and_set() {
    let mut cpu = Lc2::new(0x3000);

    // For every address...
    for address in 0..2_usize.pow(16) {
        #[allow(clippy::cast_possible_truncation)]
        let address = address as u16;

        // Set the memory address
        cpu.set_memory(address, !address);

        // Check if the memory has been set
        assert_eq!(cpu.get_memory(address), !address);
    }
}

#[test]
fn load_bytes() {
    let mut cpu = Lc2::new(0x3000);
    cpu.load_bytes(0x6000, &[1, 2, 3, 4, 5]).unwrap();

    assert_eq!(cpu.get_memory(0x6000), 0x0102);
    assert_eq!(cpu.get_memory(0x6001), 0x0304);
    assert_eq!(cpu.get_memory(0x6002), 0x0500);
}

#[test]
fn load_bytes_fails() {
    let mut cpu = Lc2::new(0x3000);

    assert_eq!(
        cpu.load_bytes(0xffff, &[1, 2]),
        Err("The array of byte is too big")
    );
}

#[test]
fn watchers() {
    // Create a new LC2 and an atomic u16 to store the watcher results
    let mut cpu = Lc2::new(0x3000);
    let value = Rc::new(AtomicU16::new(0));

    // For every address...
    for address in 0..2_usize.pow(16) {
        #[allow(clippy::cast_possible_truncation)]
        let address = address as u16;

        // Create a watcher that negates the value in memory
        let value_watcher = value.clone();
        cpu.add_memory_watcher(address, move |new_value| {
            value_watcher.store(!new_value, Ordering::Relaxed);
        });

        // Set the memory address
        cpu.set_memory(address, address);

        // Check if the watcher has been called
        assert_eq!(value.load(Ordering::Relaxed), !address);

        // Remove the watcher and assert that nothing changes
        cpu.remove_memory_watcher(address);
        cpu.set_memory(address, u16::from(address == 0));
        assert_eq!(value.load(Ordering::Relaxed), !address);
    }
}
