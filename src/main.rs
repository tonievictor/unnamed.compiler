use std::env;
use std::fs;
use std::process::exit;

pub mod asm;
pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage {0} <OPTIONS>", &args[0]);
        exit(1);
    }

    let mut option = "";

    if args.len() > 2 {
        match args[1].as_str() {
            "--lex" | "--codegen" | "--parse" => {
                option = args[1].as_str();
            }
            _ => {
                eprintln!("Invalid mode");
                exit(1);
            }
        }
    }

    let filename = if args.len() > 2 { &args[2] } else { &args[1] };
    if &filename[(filename.len() - 2)..] != ".c" {
        eprintln!("Cannot execute {}: Invalid filetype", filename);
        exit(1);
    }

    let file_content = match fs::read_to_string(filename) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("ERROR {}: {}", &args[1], err);
            exit(1);
        }
    };

    let tokens = match lexer::tokenize(file_content) {
        Ok(Some(t)) => t,
        Ok(None) => {
            eprintln!("Empty file");
            exit(1);
        }
        Err(err) => {
            eprintln!("ERROR {}: {}", &args[1], err);
            exit(1);
        }
    };

    if option == "--lex" {
        println!("{:#?}", tokens);
        exit(0);
    }

    let ast = match parser::parse(tokens) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("ERROR {}: {}", &args[1], err);
            exit(1);
        }
    };

    if option == "--parse" {
        println!("{:#?}", ast);
        exit(0);
    }
    let asm = asm::to_asm(ast);

    match codegen::write_asm_to_file(
        asm,
        format!("{}.s", &filename[..(filename.len() - 2)]).as_str(),
    ) {
        Ok(_) => {
            println!("finished compilation")
        }
        Err(err) => {
            eprintln!(
                "An error occured while generating assembly instructions {:?}",
                err
            );
            exit(1);
        }
    }
}
