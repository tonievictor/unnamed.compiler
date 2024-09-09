use std::env;
use std::fs;
use std::process::exit;

pub mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage {0} <OPTIONS>", &args[0]);
        exit(1);
    }

    let file: String;
    match fs::read_to_string(&args[1]) {
        Ok(f) => {
            file = f;
        },
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
    let tokens = lexer::tokenize(file);
    dbg!(tokens);

    if args.len() > 2 {
        match args[2].as_str() {
            "--lex" => {
                println!("Lexing mode");
                exit(0);
            }
            "--parse" => {
                println!("Parsing mode");
                exit(0);
            }
            "--codegen" => {
                println!("Codegen mode");
                exit(0);
            }
            _ => {
                println!("Invalid mode");
                exit(0);
            }
        }
    }

    exit(0);
}
