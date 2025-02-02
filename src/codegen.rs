use crate::asm::{ASMFunctionDefinition, ASMProgram, ASMStatement, Instruction, Operand, Register};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn write_asm_to_file(asm: ASMProgram, filename: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

    //.globl <name>
    let globl = format!("global {}\n\n", asm.function.name.as_str());
    file.write(globl.as_bytes())?;

    write_func(&file, asm.function)?;

    // indicates that the generated asm code doesn't need an executable stack
    file.write("section .note.GNU-stack,\"\",@progbits".as_bytes())?;
    Ok(())
}

fn write_func(mut file: &File, func: ASMFunctionDefinition) -> Result<(), io::Error> {
    let name = format!("{}:\n", func.name.as_str());
    file.write(name.as_bytes())?;
    let mut instructions = String::new();

    for i in func.instructions.iter() {
        let instr = get_instructions(i.clone());
        instructions.push_str(&instr);
    }

    file.write(instructions.as_str().as_bytes())?;
    Ok(())
}

fn get_instructions(instr: Instruction) -> String {
    match instr {
        Instruction::Ret => String::from("ret\n"),
        Instruction::Mov(statement) => String::from(format!("mov {}\n", get_statement(statement))),
    }
}

fn get_statement(stm: ASMStatement) -> String {
    let src = get_operand(stm.src);
    let dst = get_operand(stm.dst);

    String::from(format!("{}, {}", dst.as_str(), src.as_str()))
}

fn get_operand(opr: Operand) -> String {
    match opr {
        Operand::Imm(val) => val.to_string(),
        Operand::Register(reg) => match reg {
            Register::EAX => String::from("eax"),
        },
    }
}
