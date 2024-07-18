use super::*;

use std::rc::Rc;
use std::sync::atomic::{AtomicU16, Ordering};

#[test]
fn get_and_set() {
    let mut cpu = Lc2::new(0x3000);

    // For every register...
    for i in 0..8u16 {
        let register = Register::Gpr(Gpr::try_from(i as usize).unwrap());

        // Set the register
        cpu.set_register(&register, 3000 + i);

        // Check if the register has been set
        assert_eq!(cpu.get_register(&register), 3000 + i);
    }
}

#[test]
fn u8_to_gpr() {
    for i in 0..8u16 {
        let _ = Register::Gpr(Gpr::try_from(i as usize).unwrap());
    }
}

#[test]
#[should_panic(expected = "Only numbers between 0 and 7 can be converted into registers!")]
fn u8_to_invalid_gpr() {
    Gpr::try_from(8).unwrap();
}

#[test]
fn memory_registers() {
    let mut cpu = Lc2::new(0x3000);

    // Set the MAR to 0x3000 and the MDR to 0x3042
    cpu.set_memory(0x3000, 0x3042);
    assert_eq!(cpu.get_register(&Register::MemoryAddressRegister), 0x3000);
    assert_eq!(cpu.get_register(&Register::MemoryDataRegister), 0x3042);

    // Change another memory cell
    cpu.set_memory(0x3001, 0x0000);

    // Check if the MAR and the MDR are right if 0x3000 is requested
    let _ = cpu.get_memory(0x3000);
    assert_eq!(cpu.get_register(&Register::MemoryAddressRegister), 0x3000);
    assert_eq!(cpu.get_register(&Register::MemoryDataRegister), 0x3042);
}

#[test]
fn watchers() {
    // Create a new LC2 and an atomic u16 to store the watcher results
    let mut cpu = Lc2::new(0x3000);
    let value = Rc::new(AtomicU16::new(0));

    // For every register...
    for i in 0..8u16 {
        let register = Register::Gpr(Gpr::try_from(i as usize).unwrap());

        // Create a watcher that adds the answer to the ultimate question of
        // life, the universe, and everything to the value that is put into the
        // register
        let value_watcher = value.clone();
        cpu.add_register_watcher(&register, move |new_value| {
            value_watcher.store(new_value + 42, Ordering::Relaxed);
        });

        // Set the register
        cpu.set_register(&register, 3000 + i);

        // Check if the watcher has been called
        assert_eq!(value.load(Ordering::Relaxed), 3042 + i);

        // Remove the watcher and assert that nothing changes
        cpu.remove_register_watcher(&register);
        cpu.set_register(&register, 6000 + i);
        assert_eq!(value.load(Ordering::Relaxed), 3042 + i);
    }
}
