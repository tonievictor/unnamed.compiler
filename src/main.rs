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

    let file_content: String;
    match fs::read_to_string(&args[1]) {
        Ok(f) => {
            file_content = f;
        },
        Err(err) => {
            eprintln!("ERROR {}:{}", &args[1], err);
            exit(1);
        }
    }

    if args.len() > 2 {
        match args[2].as_str() {
            "--lex" => {
                let tokens = lexer::tokenize(file_content);
                match tokens {
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        println!("Empty file");
                        exit(1);
                    }
                    Err(err) => {
                        eprintln!("ERROR {}:{}", &args[1], err);
                        exit(1);
                    }
                }
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

    let tokens = lexer::tokenize(file_content);
    match tokens {
        Ok(Some(t)) => {dbg!(t);}
        Ok(None) => {
            println!("Empty file");
            exit(1);
        }
        Err(err) => {
            eprintln!("ERROR {}:{}", &args[1], err);
            exit(1);
        }
    }

    exit(0);
}
