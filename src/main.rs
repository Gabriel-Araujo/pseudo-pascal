mod lexical;
mod common;

use std::fs::File;
use std::process::exit;
use std::env;
use std::io::Read;
use lexical::Scanner;

fn main() {
    let input =  consume_file(get_file_path()).unwrap();

    let mut scanner = Scanner::new(&input);

    let tokens = match scanner.init() {
        Ok(values) => {values}
        Err(e) => {
            eprintln!("An error occurred in the lexical parsing.");
            eprintln!("{e}");
            exit(1);
        }
    };

    for token in &tokens {
        println!("{token}");
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
