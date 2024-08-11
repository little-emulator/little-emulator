use super::*;

use crate::WatcherType;
use std::rc::Rc;
use std::sync::atomic::{AtomicU16, Ordering};

#[test]
fn get_and_set() {
    let mut cpu = Lc3::new(0x3000);

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
    let mut cpu = Lc3::new(0x3000);
    cpu.load_bytes(0x6000, &[1, 2, 3, 4, 5]).unwrap();

    assert_eq!(cpu.get_memory(0x6000), 0x0102);
    assert_eq!(cpu.get_memory(0x6001), 0x0304);
    assert_eq!(cpu.get_memory(0x6002), 0x0500);
}

#[test]
fn load_bytes_fails() {
    let mut cpu = Lc3::new(0x3000);

    assert_eq!(
        cpu.load_bytes(0xffff, &[1, 2]),
        Err("The array of byte is too big")
    );
}

#[test]
fn watchers_on_write() {
    // Create a new LC3 and an atomic u16 to store the watcher results
    let mut cpu = Lc3::new(0x3000);
    let value = Rc::new(AtomicU16::new(0));

    // For every address...
    for address in 0..2_usize.pow(16) {
        #[allow(clippy::cast_possible_truncation)]
        let address = address as u16;

        // Reset the watcher results
        value.store(0, Ordering::Relaxed);

        // Create a watcher that negates the value in memory
        let value_watcher = value.clone();
        cpu.add_memory_watcher(address, WatcherType::OnWrite, move |new_value| {
            value_watcher.store(!new_value, Ordering::Relaxed);
        });

        // Get the memory address
        let _ = cpu.get_memory(address);

        // Check that the watcher has NOT been called
        assert_eq!(value.load(Ordering::Relaxed), 0);

        // Set the memory address
        cpu.set_memory(address, address);

        // Check that the watcher has been called
        assert_eq!(value.load(Ordering::Relaxed), !address);

        // Remove the watcher and assert that nothing changes
        cpu.remove_memory_watcher(address, WatcherType::OnWrite);
        cpu.set_memory(address, u16::from(address == 0));
        assert_eq!(value.load(Ordering::Relaxed), !address);
    }
}

#[test]
fn watchers_on_read() {
    // Create a new LC3 and an atomic u16 to store the watcher results
    let mut cpu = Lc3::new(0x3000);
    let value = Rc::new(AtomicU16::new(0));

    // For every address...
    for address in 0..2_usize.pow(16) {
        #[allow(clippy::cast_possible_truncation)]
        let address = address as u16;

        // Reset the watcher results
        value.store(0, Ordering::Relaxed);

        // Create a watcher that negates the value in memory
        let value_watcher = value.clone();
        cpu.add_memory_watcher(address, WatcherType::OnRead, move |new_value| {
            value_watcher.store(!new_value, Ordering::Relaxed);
        });

        // Set the memory address
        cpu.set_memory(address, address);

        // Check that the watcher has NOT been called
        assert_eq!(value.load(Ordering::Relaxed), 0);

        // Set the memory address
        let _ = cpu.get_memory(address);

        // Check if the watcher has been called
        assert_eq!(value.load(Ordering::Relaxed), !address);

        // Remove the watcher and assert that nothing changes
        cpu.remove_memory_watcher(address, WatcherType::OnRead);
        cpu.set_memory(address, u16::from(address == 0));
        let _ = cpu.get_memory(address);
        assert_eq!(value.load(Ordering::Relaxed), !address);
    }
}
