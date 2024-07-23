mod input_thread;
use input_thread::InputThread;

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

fn main() {
    // Spawn a input thread
    let input_thread = InputThread::spawn();

    // Create a new LC2
    let mut cpu = Lc2::new(0x3000);
    setup_lc2(&mut cpu, input_thread.get_buffer());

    // Fill the memory with the bytes from the executable
    let start_address = load_file(&mut cpu, "test.obj").unwrap();
    cpu.set_register(&Register::ProgramCounter, start_address);

    // Get the input buffer
    let input_buffer = input_thread.get_buffer();

    // While the CPU is active...
    while cpu.get_memory(0xffff) & 0x8000 != 0 {
        // Is the input thread died, exit
        if !input_thread.is_alive() {
            break;
        }

        // Get the Keyboard Status Register
        let keyboard_status_register = cpu.get_memory(0xf400);

        // If the input buffer is not empty...
        if let Some(input_byte) = input_buffer.lock().unwrap().front() {
            let input_byte = u16::from(*input_byte);

            // Set the Keyboard Status Register
            if keyboard_status_register & 0x8000 == 0 {
                cpu.set_memory(0xf400, keyboard_status_register | 0x8000);
            }

            // Set the Keyboard Data Register
            cpu.set_memory(0xf401, input_byte);
        }
        // Else unset the Keyboard Status Register
        else if keyboard_status_register & 0x8000 != 0 {
            cpu.set_memory(0xf400, keyboard_status_register & 0x7fff);
        }

        // Step a CPU instruction
        cpu.step_instruction();
    }
}

fn load_file(cpu: &mut impl Architecture<Address = u16>, file_name: &str) -> io::Result<u16> {
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
    let start_address = u16::from_be_bytes(start_address);
    reader.consume(2);

    // Get the rest of the bytes from the binary
    let memory = reader.bytes().collect::<Result<Vec<u8>, _>>()?;

    // Load the bytes into the CPU
    cpu.load_bytes(start_address, &memory)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "The file is too long"))?;

    Ok(start_address)
}

#[allow(clippy::too_many_lines)]
fn setup_lc2(cpu: &mut Lc2, input_buffer: Arc<Mutex<VecDeque<u8>>>) {
    // Set the Video Status Register and the Machine Control Register
    cpu.set_memory(0xf3fc, 0x8000);
    cpu.set_memory(0xffff, 0x8000);

    // Add the memory watcher for the Video Data Register
    cpu.add_memory_watcher(0xf3ff, WatcherType::OnWrite, |data: u16| {
        // Print the character to stdout
        print!(
            "{}",
            char::from_u32(u32::from(data) & 0xff).expect("Character is not convertible to UTF-8")
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
    cpu.add_memory_watcher(0xf401, WatcherType::OnRead, move |_| {
        input_buffer.lock().unwrap().pop_front();
    });

    // ================================= Trap ==================================

    // Setup the trap vector
    for address in 0x0000..0x00ff {
        cpu.set_memory(
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

    // Invalid trap syscall (From LC2Simulate)
    cpu.load_bytes(
        0xfd00,
        &[
            0x3f, 0x65, // ST R7, save_r7
            0x31, 0x64, // ST R0, save_r0
            0x21, 0x0c, // LD R0, line_feed
            0xf0, 0x21, // OUT
            0xe1, 0x0d, // LEA R0, banner_1
            0xf0, 0x22, // PUTS
            0xe1, 0x40, // LEA R0, banner_2
            0xf0, 0x22, // PUTS
            0xf0, 0x25, // HALT
            0x21, 0x64, // ST R0, save_r0
            0x2f, 0x65, // ST R7, save_r7
            0xd0, 0x00, // RET
            0x00, 0x0a, // line_feed: .fill 0x000a
            // banner_1: .stringz "\nA trap was executed with an illegal vector number."
            0x00, 0x41, 0x00, 0x20, 0x00, 0x74, 0x00, 0x72, 0x00, 0x61, 0x00, 0x70, 0x00, 0x20,
            0x00, 0x77, 0x00, 0x61, 0x00, 0x73, 0x00, 0x20, 0x00, 0x65, 0x00, 0x78, 0x00, 0x65,
            0x00, 0x63, 0x00, 0x75, 0x00, 0x74, 0x00, 0x65, 0x00, 0x64, 0x00, 0x20, 0x00, 0x77,
            0x00, 0x69, 0x00, 0x74, 0x00, 0x68, 0x00, 0x20, 0x00, 0x61, 0x00, 0x6e, 0x00, 0x20,
            0x00, 0x69, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x65, 0x00, 0x67, 0x00, 0x61, 0x00, 0x6c,
            0x00, 0x20, 0x00, 0x76, 0x00, 0x65, 0x00, 0x63, 0x00, 0x74, 0x00, 0x6f, 0x00, 0x72,
            0x00, 0x20, 0x00, 0x6e, 0x00, 0x75, 0x00, 0x6d, 0x00, 0x62, 0x00, 0x65, 0x00, 0x72,
            0x00, 0x2e, 0x00, 0x00, // End banner_1
            // banner_2: .stringz "Machine state should be questioned."
            0x00, 0x4d, 0x00, 0x61, 0x00, 0x63, 0x00, 0x68, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x65,
            0x00, 0x20, 0x00, 0x73, 0x00, 0x74, 0x00, 0x61, 0x00, 0x74, 0x00, 0x65, 0x00, 0x20,
            0x00, 0x73, 0x00, 0x68, 0x00, 0x6f, 0x00, 0x75, 0x00, 0x6c, 0x00, 0x64, 0x00, 0x20,
            0x00, 0x62, 0x00, 0x65, 0x00, 0x20, 0x00, 0x71, 0x00, 0x75, 0x00, 0x65, 0x00, 0x73,
            0x00, 0x74, 0x00, 0x69, 0x00, 0x6f, 0x00, 0x6e, 0x00, 0x65, 0x00, 0x64, 0x00, 0x2e,
            0x00, 0x00, // End banner_2
            0x00, 0x00, // save_r0: .fill 0x0000
            0x00, 0x00, // save_r7: .fill 0x0000
        ],
    )
    .expect("Couldn't put trap subroutine at address 0xfd00");

    // GETC Syscall (From LC2Simulate)
    // TODO: Delete R7
    cpu.load_bytes(
        0x0400,
        &[
            0x3e, 0x08, // ST R7, save_r7
            0xa0, 0x06, // ready_loop: LDI R0, keyboard_status_register
            0x06, 0x01, // BRZP ready_loop
            0xa0, 0x07, // LDI R0, keyboard_data_register
            0x2e, 0x08, // LD R7, save_r7
            0xd0, 0x00, // RET
            0xf4, 0x00, // keyboard_status_register: .fill 0xf400
            0xf4, 0x01, // keyboard_data_register: .fill 0xf401
            0x00, 0x00, // save_r7: .fill 0x0000
        ],
    )
    .expect("Couldn't put trap subroutine at address 0x0400");

    // OUT Syscall (From LC2Simulate)
    // TODO: Delete R7
    cpu.load_bytes(
        0x0430,
        &[
            0x3e, 0x3b, // ST R7, save_r7
            0x32, 0x3a, // ST R1, save_r1
            0xa2, 0x38, // ready_loop: LDI R1, video_status_register
            0x06, 0x32, // BRZP 0x432, ready_loop
            0xb0, 0x39, // STI R0, video_data_register
            0x22, 0x3a, // LD R1, save_r1
            0x2e, 0x3b, // LD R7, save_r7
            0xd0, 0x00, // RET
            0xf3, 0xfc, // video_status_register: .fill 0xf3fc
            0xf3, 0xff, // video_data_register: .fill 0xf3ff
            0x00, 0x00, // save_r1: .fill 0x0000
            0x00, 0x00, // save_r7: .fill 0x0000
        ],
    )
    .expect("Couldn't put trap subroutine at address 0x0430");

    // PUTS Syscall (From LC2Simulate)
    cpu.load_bytes(
        0x0450,
        &[
            0x3e, 0x67, // ST R7, save_r7
            0x30, 0x64, // ST R0, save_r0
            0x32, 0x65, // ST R1, save_r1
            0x34, 0x66, // ST R2, save_r2
            0x62, 0x00, // print_loop: LDR R1, R0, 0
            0x04, 0x5b, // BRZ end_print_loop
            0xa4, 0x60, // ready_loop: LDI R2, video_status_register
            0x06, 0x56, // BRZP ready_loop
            0xb2, 0x61, // STI R1, video_data_register
            0x10, 0x21, // ADD R0, R0, 1
            0x0e, 0x54, // BR print_loop
            0x20, 0x64, // end_print_loop: LD R0, save_r0
            0x22, 0x65, // LD R1, save_r1
            0x24, 0x66, // LD R2, save_r2
            0x2e, 0x67, // LD R7, save_r7
            0xd0, 0x00, // RET
            0xf3, 0xfc, // video_status_register: .fill 0xf3fc
            0xf3, 0xff, // video_data_register: .fill 0xf3ff
            0xf3, 0xfd, // horizontal_screen_position: .fill 0xf3fd
            0xf3, 0xfe, // vertical_screen_position: .fill 0xf3fe
            0x00, 0x00, // save_r0: .fill 0x0000
            0x00, 0x00, // save_r1: .fill 0x0000
            0x00, 0x00, // save_r2: .fill 0x0000
            0x00, 0x00, // save_r7: .fill 0x0000
        ],
    )
    .expect("Couldn't put trap subroutine at address 0x0450");

    // IN Syscall (From LC2Simulate)
    cpu.load_bytes(
        0x04a0,
        &[
            0x3e, 0xc8, // ST R7, save_r7
            0x36, 0xc7, // ST R3, save_r3
            0x34, 0xc6, // ST R2, save_r2
            0x32, 0xc5, // ST R1, save_r1
            0x20, 0xc4, // LD R0, line_feed
            0x48, 0xb9, // JSR print_subroutine
            0xe2, 0xc9, // LEA R1, prompt
            0x60, 0x40, // print_loop: LDR R0, R1, 0
            0x04, 0xac, // BRZ wait_char_loop
            0x48, 0xb9, // JSR print_subroutine
            0x12, 0x61, // ADD R1, R1, 1
            0x0e, 0xa7, // BR print_loop
            0xa6, 0xc3, // wait_char_loop: LDI R3, keyboard_status_register
            0x06, 0xac, // BRZP wait_char_loop
            0xa0, 0xc2, // LDI R0, keyboard_data_register
            0x14, 0x20, // ADD R2, R0, 0
            0x48, 0xb9, // JSR print_subroutine
            0x20, 0xc4, // LD R0, line_feed
            0x48, 0xb9, // JSR print_subroutine
            0x10, 0xa0, // ADD R0, R2, 0
            0x22, 0xc5, // LD R3, save_r3
            0x24, 0xc6, // LD R2, save_r2
            0x26, 0xc7, // LD R1, save_r1
            0x2e, 0xc8, // LD R7, save_r7
            0xd0, 0x00, // RET
            0x3e, 0xbf, // print_subroutine: ST R7, save_r7_2
            0xa6, 0xc1, // ready_loop: LDI R3, video_status_register
            0x06, 0xb9, // BRZP ready_loop
            0xb0, 0xc0, // STI R0, video_data_register
            0x2e, 0xbf, // LD R7, save_r7_2
            0xd0, 0x00, // RET
            0x00, 0x00, // save_r7_2: .fill 0x0000
            0xf3, 0xff, // video_data_register: .fill 0xf3ff
            0xf3, 0xfc, // video_status_register: .fill 0xf3fc
            0xf4, 0x01, // keyboard_data_register: .fill 0xf401
            0xf4, 0x00, // keyboard_status_register: .fill 0xf400
            0x00, 0x0a, // line_feed: .fill 0x000a
            0x00, 0x00, // save_r1: .fill 0x0000
            0x00, 0x00, // save_r2: .fill 0x0000
            0x00, 0x00, // save_r3: .fill 0x0000
            0x00, 0x00, // save_r7: .fill 0x0000
            // prompt: .stringz "Input a character>"
            0x00, 0x49, 0x00, 0x6e, 0x00, 0x70, 0x00, 0x75, 0x00, 0x74, 0x00, 0x20, 0x00, 0x61,
            0x00, 0x20, 0x00, 0x63, 0x00, 0x68, 0x00, 0x61, 0x00, 0x72, 0x00, 0x61, 0x00, 0x63,
            0x00, 0x74, 0x00, 0x65, 0x00, 0x72, 0x00, 0x3e, 0x00, 0x00, //End prompt
        ],
    )
    .expect("Couldn't put trap subroutine at address 0x04a0");

    // PUTSP Syscall (From LC2Simulate)
    cpu.load_bytes(
        0x04e0,
        &[
            0x3f, 0x08, // ST R7, save_r7
            0x31, 0x04, // ST R0, save_r0
            0x33, 0x05, // ST R1, save_r1
            0x35, 0x06, // ST R2, save_r2
            0x37, 0x07, // ST R3, save_r3
            0x12, 0x20, // ADD R1, R0, 0
            0x60, 0x40, // print_loop: LDR R0, R1, 0
            0x04, 0xee, // BRZ end_print_loop
            0x48, 0xf6, // JSR print_subroutine
            0x25, 0x02, // LD R2, mask
            0x50, 0x02, // AND R0, R0, R2
            0x04, 0xee, // BRZ end_print_loop
            0x12, 0x61, // ADD R1, R1, 1
            0x0e, 0xe6, // BR print_loop
            0x21, 0x03, // end_print_loop: LD R0, line_feed
            0x48, 0xf6, // JSR print_subroutine
            0x21, 0x04, // LD R0, save_r0
            0x23, 0x05, // LD R1, save_r1
            0x25, 0x06, // LD R2, save_r2
            0x27, 0x07, // LD R3, save_r3
            0x2f, 0x08, // LD R7, save_r7
            0xd0, 0x00, // RET
            0x3e, 0xfd, // print_subroutine: ST R7, save_r7_2
            0xa6, 0xff, // LDI R3, video_status_register
            0x08, 0xfa, // BRN end_ready_loop
            0x0e, 0xf6, // BR print_subroutine
            0xb0, 0xfe, // end_ready_loop: STI R0, video_data_register
            0x2e, 0xfd, // LD R7, save_r7_2
            0xd0, 0x00, // RET
            0x00, 0x00, // save_r7_2: .fill 0x0000
            0xf3, 0xff, // video_data_register: .fill 0xf3ff
            0xf3, 0xfc, // video_status_register: .fill 0xf3fc
            0xf3, 0xfd, // horizontal_screen_position: .fill 0xf3fd
            0xf3, 0xfe, // vertical_screen_position: .fill 0xf3fe
            0xff, 0x00, // mask: .fill 0xff00
            0x00, 0x0a, // line_feed: .fill 0x000a
            0x00, 0x00, // save_r0: .fill 0x0000
            0x00, 0x00, // save_r1: .fill 0x0000
            0x00, 0x00, // save_r2: .fill 0x0000
            0x00, 0x00, // save_r3: .fill 0x0000
            0x00, 0x00, // save_r7: .fill 0x0000
        ],
    )
    .expect("Couldn't put trap subroutine at address 0x04e0");

    // HALT Syscall (From LC2Simulate)
    cpu.load_bytes(
        0xfd70,
        &[
            0x3f, 0x7f, // ST R7, save_r7
            0x33, 0x7e, // ST R1, save_r1
            0x31, 0x7d, // ST R0, save_r0
            0xe1, 0x80, // LEA R0, banner
            0xf0, 0x22, // PUTS
            0xa3, 0xa5, // LDI R1, machine_control_register
            0x21, 0xa6, // LD R0, mask
            0x50, 0x40, // AND R0, R1, R0
            0xb1, 0xa5, // STI R0, machine_control_register
            0x21, 0x7d, // ST R0, save_r0
            0x23, 0x7e, // ST R1, save_r1
            0x2f, 0x7f, // ST R7, save_r7
            0xd0, 0x00, // RET
            0x00, 0x00, // save_r0: .fill 0x0000
            0x00, 0x00, // save_r1: .fill 0x0000
            0x00, 0x00, // save_r7: .fill 0x0000
            // banner: .stringz "\n----- Halting the processor ----- \n"
            0x00, 0x0a, 0x00, 0x2d, 0x00, 0x2d, 0x00, 0x2d, 0x00, 0x2d, 0x00, 0x2d, 0x00, 0x20,
            0x00, 0x48, 0x00, 0x61, 0x00, 0x6c, 0x00, 0x74, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x67,
            0x00, 0x20, 0x00, 0x74, 0x00, 0x68, 0x00, 0x65, 0x00, 0x20, 0x00, 0x70, 0x00, 0x72,
            0x00, 0x6f, 0x00, 0x63, 0x00, 0x65, 0x00, 0x73, 0x00, 0x73, 0x00, 0x6f, 0x00, 0x72,
            0x00, 0x20, 0x00, 0x2d, 0x00, 0x2d, 0x00, 0x2d, 0x00, 0x2d, 0x00, 0x2d, 0x00, 0x20,
            0x00, 0x0a, 0x00, 0x00, // End banner
            0xff, 0xff, // machine_control_register: .fill 0xffff
            0x7f, 0xff, // mask: .fill 0x7fff
        ],
    )
    .expect("Couldn't put trap subroutine at address 0xfd70");
}
