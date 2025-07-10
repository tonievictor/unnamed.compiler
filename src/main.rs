use std::env;
use std::fs;
use std::process::exit;

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
    // if &filename[(filename.len() - 2)..] != ".c" {
    //     eprintln!("Cannot execute {}: Invalid filetype", filename);
    //     exit(1);
    // }

    let file_content = match fs::read_to_string(filename) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("ERROR {}: {}", &args[1], err);
            exit(1);
        }
    };

    let tokens = match lexer::tokenize(file_content) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("ERROR {}: {}", &args[1], err);
            exit(1);
        }
    };

    if option == "--lex" {
        exit(0);
    }
    parser::parse(&mut tokens.iter());
}
