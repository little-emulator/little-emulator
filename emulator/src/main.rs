use architectures::lc2::Lc2;
use emulator::{Emulator, InputThread};

fn main() {
    // Spawn an input thread
    let input_thread = InputThread::spawn();

    // Create a new LC2, set it up and load the binary
    let mut cpu = Lc2::new(0x3000);
    cpu.setup_memory(input_thread.get_buffer());
    cpu.load_binary("test.obj").unwrap();

    // Run the binary
    cpu.emulate(input_thread);
}
