use std::env;
use std::fs;
use std::process::exit;

pub mod lexer;
pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage {0} <filename.un>", &args[0]);
        exit(1);
    }

    let file_content = match fs::read_to_string(&args[1]) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("ERROR {}: {}", &args[1], err);
            exit(1);
        }
    };

    let tokens = match lexer::tokenize(&file_content) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("ERROR {}: {}", &args[1], err);
            exit(1);
        }
    };

    let _ = parser::parse(tokens);
}
