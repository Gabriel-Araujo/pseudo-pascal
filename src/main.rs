mod common;
mod lexical;
mod syntactic;

use crate::syntactic::Parser;
use lexical::Scanner;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::time::Instant;

fn main() {
    let input = consume_file(get_file_path()).unwrap();

    let mut scanner = Scanner::new(&input);

    let now = Instant::now();
    let tokens = match scanner.init() {
        Ok(values) => values,
        Err(e) => {
            eprintln!("An error occurred in the lexical parsing.");
            eprintln!("{e}");
            exit(0);
        }
    };

    println!("Lexical scanner took {}μs.", now.elapsed().as_micros());

    /*for token in &tokens {
        println!("{token}");
    }
    println!("Parser Start");
    */

    let mut parser = Parser::new(&tokens);
    let now = Instant::now();
    let parser_result = parser.init();

    println!("Syntactic parser took {}μs.", now.elapsed().as_micros());
    match parser_result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("An error occurred in the syntactic parsing.");
            eprintln!("{e}");
            exit(0);
        }
    }
}

fn get_file_path() -> String {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        eprintln!("No file path was given.");
        exit(1);
    }

    args[1].clone()
}

fn consume_file(file_path: String) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
