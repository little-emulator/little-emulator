use assemblers::{lc2::Lc2AssemblerBuilder, Assembler};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    build_trap_routines();
}

/// Take all of the trap routine assembly files that are contained into the
/// "emulator/src/<architecture>/trap_routines/<routine_name>.asm" directory and
/// build them into "$OUT_DIR/<architecture>/<routine_name>.asm.o"
#[allow(clippy::doc_markdown)]
fn build_trap_routines() {
    // Create a new assembler
    let assembler = Lc2AssemblerBuilder::new()
        .prepend_start_address(false)
        .optional_end(true)
        .enable_stringzp(true)
        .build();

    // Get all the architectures dirs
    let architectures_dirs: Vec<fs::DirEntry> =
        fs::read_dir(Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("src"))
            .unwrap()
            .flatten()
            .filter(|x| x.metadata().unwrap().is_dir())
            .collect();

    // For every architecture...
    for architecture_dir in architectures_dirs {
        // Create an output directory for the assembled routines
        let output_dir =
            Path::new(&env::var("OUT_DIR").unwrap()).join(architecture_dir.file_name());
        if output_dir.metadata().is_err() {
            fs::create_dir(&output_dir).unwrap();
        }

        // Get the assembly files inside of the "trap_routines/" directory
        let assembly_files =
            fs::read_dir(Path::new(&architecture_dir.path()).join("trap_routines"))
                .unwrap()
                .flatten()
                .filter(|x| {
                    x.file_name().to_string_lossy().ends_with(".asm")
                        && !x.metadata().unwrap().is_dir()
                })
                .collect::<Vec<_>>();

        // For every assembly file...
        for assembly_file in assembly_files {
            // Get the contents of the file
            let assembly = fs::read_to_string(assembly_file.path()).unwrap();

            // Compile the assembly
            let (binary, _) = assembler.assemble(&assembly).unwrap();

            // Write the binary into the output file
            let output_file_name = format!("{}.o", assembly_file.file_name().to_string_lossy());
            File::create(Path::new(&output_dir).join(output_file_name))
                .unwrap()
                .write_all(&binary)
                .unwrap();
        }
    }
}
