use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

mod lc2;

mod input_thread;
pub use input_thread::InputThread;

pub trait Emulator: architectures::Architecture {
    fn emulate(&mut self, input_thread: InputThread);

    /// # Errors
    ///
    /// This method will return an `Err` if there is an error with the file, or
    /// if the binary is too short or too long
    fn load_binary(&mut self, file_name: &str) -> std::io::Result<()>;
    fn setup_memory(&mut self, input_buffer: Arc<Mutex<VecDeque<u8>>>);
}
