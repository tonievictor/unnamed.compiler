use crate::lexer::Token;
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
                option = args[2].as_str();
            }
            _ => {
                eprintln!("Invalid mode");
                exit(0);
            }
        }
    }

    let filename = if args.len() > 2 { &args[2] } else { &args[1] };

    let file_content: String;
    match fs::read_to_string(filename) {
        Ok(f) => {
            file_content = f;
        }
        Err(err) => {
            eprintln!("ERROR {}:{}", &args[1], err);
            exit(1);
        }
    }
    let tokens: Vec<Token>;

    match lexer::tokenize(file_content) {
        Ok(Some(t)) => tokens = t,
        Ok(None) => {
            eprintln!("Empty file");
            exit(1);
        }
        Err(err) => {
            eprintln!("ERROR {}:{}", &args[1], err);
            exit(1);
        }
    }

    if option == "--lex" {
        exit(0);
    }

    parser::parse(tokens);

    exit(0);
}
