use std::{
    fs::File,
    io,
    io::{BufRead, BufReader, Read, Write},
};

use architectures::{
    lc2::{Lc2, Register},
    Architecture,
};

fn main() -> io::Result<()> {
    // Create a new LC2 and fill the memory with the bytes from the executable
    let mut cpu = Lc2::default();
    let start_address = load_file(&mut cpu, "test.obj")?;
    setup_lc2(&mut cpu, start_address);

    // Execute the next istruction while the processor is not halted
    while cpu.get_memory(0xffff) & 0x8000 != 0 {
        cpu.step_instruction();
    }

    Ok(())
}

fn load_file(cpu: &mut impl Architecture<Address = u16>, file_name: &str) -> io::Result<u16> {
    // Open "test.o" in a BufReader
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

    // Get the rest of the bytes from the binary
    let memory = reader.bytes().collect::<Result<Vec<u8>, _>>()?;

    cpu.load_bytes(start_address, &memory)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "The file is too long"))?;

    Ok(start_address)
}

fn setup_lc2(cpu: &mut Lc2, start_address: u16) {
    // Setup the start address
    cpu.set_register(&Register::ProgramCounter, start_address);

    // Set the Machine Control Register
    cpu.set_memory(0xffff, 0x8000);

    // Add the memory watcher for the Video Data Register
    cpu.add_memory_watcher(0xf3ff, |data: u16| {
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

        // Flush the buffer
        io::stdout().flush().unwrap();
    });
}
