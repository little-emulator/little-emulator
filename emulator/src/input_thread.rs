use console::Key;
use std::{
    collections::VecDeque,
    sync::{
        mpsc::{self, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread,
};

pub struct InputThread {
    buffer: Arc<Mutex<VecDeque<u8>>>,
    stop_signal: Sender<()>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl InputThread {
    #[must_use]
    pub fn spawn() -> Self {
        // Create a new buffer and a channel to stop the thread
        let buffer = Arc::new(Mutex::new(VecDeque::<u8>::new()));
        let (stop_tx, stop_rx) = mpsc::channel::<()>();

        // Create the thread
        let buffer_thread = buffer.clone();
        let handle = thread::spawn(move || {
            // Create a new unbuffered terminal on stdout and disable the cursor
            let term = console::Term::stdout();
            let _ = term.hide_cursor();

            loop {
                // Read stdin without echo or buffering (raw console)
                let Ok(key) = term.read_key_raw() else { break };

                // Stop if the channel is broken or if is not empty
                match stop_rx.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => (),
                }

                // Get a lock on the buffer
                let Ok(mut buffer) = buffer_thread.lock() else {
                    break;
                };

                // Put the key into the buffer as a [u8]
                match key {
                    Key::CtrlC => break,

                    Key::Char(char) => buffer.extend(char.to_string().bytes()),
                    Key::Backspace => buffer.push_back(0x08),
                    Key::Del => buffer.push_back(0x7f),
                    Key::Enter => buffer.push_back(b'\n'),
                    Key::Escape => buffer.push_back(0x1b),
                    Key::Tab | Key::BackTab => buffer.push_back(b'\t'),

                    _ => (),
                }
            }

            // Re-enable the cursor
            let _ = term.show_cursor();
        });

        // Return the struct
        Self {
            buffer,
            stop_signal: stop_tx,
            join_handle: Some(handle),
        }
    }

    #[must_use]
    pub fn get_buffer(&self) -> Arc<Mutex<VecDeque<u8>>> {
        self.buffer.clone()
    }

    #[must_use]
    pub fn is_healthy(&self) -> bool {
        if let Some(handle) = self.join_handle.as_ref() {
            return !handle.is_finished();
        }

        false
    }
}

impl Drop for InputThread {
    fn drop(&mut self) {
        // Print a message
        println!();
        if self.is_healthy() {
            // TODO: Send a char to stdin to exit without pressing any keys
            println!("\nPress any key to continue...");
        } else {
            eprintln!("\nCtrl-C detected, closing...");
        }

        // Send the stop signal to the thread
        let _ = self.stop_signal.send(());

        // Join the thread
        let _ = self.join_handle.take().map(thread::JoinHandle::join);
    }
}
