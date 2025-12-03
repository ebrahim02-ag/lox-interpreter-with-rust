use std::env;
use std::fs;
use std::io::Read;
use std::io::{self, Write, BufRead};

mod scanner;
mod token;
mod token_type;
use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let had_error: bool = false;
    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        std::process::exit(64);
    }

    if args.len() == 2 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }

}

fn run_file(path: &str){
    let contents: Vec<u8> = fs::read(path).unwrap_or_else(|err|{
        eprintln!("Error reading file {}", path);
        std::process::exit(64);
    });
    run(String::from_utf8(contents).unwrap());
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    println!("Welcome to rlox! Type your commands below:");
    
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                run(input);
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }

        print!("> ");
        stdout.flush().unwrap();
    }
}

fn run(source: String){
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        eprintln!("{}", token);
    }
}

fn lox_error(line: &usize, message: &str){
    report(line, "", message)
}

fn report(line: &usize, _where: &str, message: &str){
    eprintln!("line {} Error {}: {}", line, _where, message)
}