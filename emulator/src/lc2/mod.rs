use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
    sync::{Arc, Mutex},
};

use architectures::{
    lc2::{Lc2, Register},
    Architecture, WatcherType,
};

macro_rules! embed_assembly {
    ($architecture: expr, $trap_routine: expr) => {{
        include_bytes!(concat!(
            env!("OUT_DIR"),
            "/",
            $architecture,
            "/",
            $trap_routine,
            ".o"
        ))
    }};
}

impl crate::Emulator for Lc2 {
    fn emulate(&mut self, input_thread: crate::InputThread) {
        // Get the input buffer
        let input_buffer = input_thread.get_buffer();

        // While the CPU is active...
        while self.get_memory(0xffff) & 0x8000 != 0 {
            // Step a CPU instruction
            self.step_instruction();

            // Is the input thread is not healthy, exit
            if !input_thread.is_healthy() {
                break;
            }

            // Get the Keyboard Status Register
            let keyboard_status_register = self.get_memory(0xf400);

            // If the input buffer is not empty...
            if let Some(input_byte) = input_buffer.lock().unwrap().front() {
                let input_byte = u16::from(*input_byte);

                // Set the Keyboard Status Register
                if keyboard_status_register & 0x8000 == 0 {
                    self.set_memory(0xf400, keyboard_status_register | 0x8000);
                }

                // Set the Keyboard Data Register
                self.set_memory(0xf401, input_byte);
            }
            // Else unset the Keyboard Status Register
            else if keyboard_status_register & 0x8000 != 0 {
                self.set_memory(0xf400, keyboard_status_register & 0x7fff);
            }
        }
    }

    fn load_binary(&mut self, file_name: &str) -> io::Result<()> {
        // Open the file in a BufReader
        let mut reader = BufReader::new(File::open(file_name)?);

        // Get the start address from the first two bytes of the binary
        let mut start_address = [0u8; 2];
        start_address.clone_from_slice(reader.fill_buf()?.get(..2).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "File must be at least 2 bytes long",
            )
        })?);
        reader.consume(2);
        let start_address = u16::from_be_bytes(start_address);

        // Set the Program Counter to `start_address`
        self.set_register(&Register::ProgramCounter, start_address);

        // Put the rest of the binary in memory starting from `start_address`
        self.load_bytes(
            start_address,
            &reader.bytes().collect::<Result<Vec<u8>, _>>()?,
        )
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "The file is too long"))?;

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn setup_memory(&mut self, input_buffer: Arc<Mutex<VecDeque<u8>>>) {
        // Set the Video Status Register and the Machine Control Register
        self.set_memory(0xf3fc, 0x8000);
        self.set_memory(0xffff, 0x8000);

        // Add the memory watcher for the Video Data Register
        self.add_memory_watcher(0xf3ff, WatcherType::OnWrite, |data: u16| {
            // Print the character to stdout
            print!(
                "{}",
                char::from_u32(u32::from(data) & 0xff)
                    .expect("Character is not convertible to UTF-8")
            );

            // If the data contains another character (packed string), print it to
            // stdout
            if data & 0xff00 != 0 {
                print!(
                    "{}",
                    char::from_u32(u32::from(data) >> 8)
                        .expect("Character is not convertible to UTF-8")
                );
            }

            // Flush the stdout buffer
            io::stdout()
                .flush()
                .expect("Couldn't flush the stdout buffer");
        });

        // If the Keyboard Data Register is read, remove the first byte in the
        // input buffer
        self.add_memory_watcher(0xf401, WatcherType::OnRead, move |_| {
            input_buffer.lock().unwrap().pop_front();
        });

        // ================================= Trap ==================================

        // Setup the trap vector
        for address in 0x0000..0x00ff {
            self.set_memory(
                address,
                match address {
                    0x20 => 0x0400, // GETC
                    0x21 => 0x0430, // OUT
                    0x22 => 0x0450, // PUTS
                    0x23 => 0x04a0, // IN
                    0x24 => 0x04e0, // PUTSP
                    0x25 => 0xfd70, // HALT
                    _ => 0xfd00,    // Invalid trap
                },
            );
        }

        // GETC syscall
        self.load_bytes(0x0400, embed_assembly!("lc2", "getc"))
            .expect("Couldn't put trap subroutine at address 0x0400");

        // OUT syscall
        self.load_bytes(0x0430, embed_assembly!("lc2", "out"))
            .expect("Couldn't put trap subroutine at address 0x0430");

        // PUTs Syscall
        self.load_bytes(0x0450, embed_assembly!("lc2", "puts"))
            .expect("Couldn't put trap subroutine at address 0x0450");

        // IN syscall
        self.load_bytes(0x04a0, embed_assembly!("lc2", "in"))
            .expect("Couldn't put trap subroutine at address 0x04a0");

        // PUTSP syscall
        self.load_bytes(0x04e0, embed_assembly!("lc2", "putsp"))
            .expect("Couldn't put trap subroutine at address 0x04e0");

        // HALT syscall
        self.load_bytes(0xfd70, embed_assembly!("lc2", "halt"))
            .expect("Couldn't put trap subroutine at address 0xfd70");

        // Invalid trap syscall
        self.load_bytes(0xfd00, embed_assembly!("lc2", "invalid"))
            .expect("Couldn't put trap subroutine at address 0xfd00");
    }
}
