use crate::asm::ASMProgram;
use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn write_asm_to_file(asm: ASMProgram, filename: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

    // indicates that the generated asm code doesn't need an executable stack
    file.write(".section .note.GNU-stack,\"\",@progbits".as_bytes())?;
    Ok(())
}
