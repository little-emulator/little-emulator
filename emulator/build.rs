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

fn build_trap_routines() {
    let trap_routines = ["getc", "out", "puts", "in", "putsp", "halt", "invalid"];
    let architectures = ["lc2"];

    // For every architecture...
    for architecture in architectures {
        // Create an output directory for the routines
        let out_dir_path = format!("{}/{}", env::var("OUT_DIR").unwrap(), architecture);
        if fs::metadata(&out_dir_path).is_err() {
            fs::create_dir(&out_dir_path).unwrap();
        }

        // For every trap routine...
        for trap in trap_routines {
            // Get the assembly path
            let src_path = format!(
                "{}/src/{}/trap_routines/{}.asm",
                env::var("CARGO_MANIFEST_DIR").unwrap(),
                architecture,
                trap,
            );

            // Compile the assembly
            let (binary, _symbol_table) = Lc2AssemblerBuilder::new()
                .prepend_start_address(false)
                .optional_end(true)
                .build()
                .assemble(&fs::read_to_string(&src_path).unwrap())
                .unwrap();

            // Write the output into the output file
            let mut dest =
                File::create(&Path::new(&out_dir_path).join(format!("{trap}.o"))).unwrap();
            dest.write_all(&binary).unwrap();
        }
    }
}
