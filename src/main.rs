use std::env;
use std::fs;
use std::io::Read;
use std::io::{self, Write, BufRead};

mod scanner;
mod token;
mod token_type;
use scanner::Scanner;

use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::TokenType;
mod expr;
mod ast_printer;

fn main() {
let expr = Expr::Binary(expr::Binary {
    left: Box::new(Expr::Unary(expr::Unary {
        op: Token::new(TokenType::Minus, "-", token::Literal::Nil, 1),
        right: Box::new(Expr::Literal(token::Literal::Number(123.0))),
    })),
    op: Token::new(TokenType::Star, "*", token::Literal::Nil, 1),
    right: Box::new(Expr::Grouping(expr::Grouping {
        expression: Box::new(Expr::Literal(token::Literal::Number(45.67))),
    })),
});

let mut printer = ast_printer::AstPrinter;
println!("{}", printer.print(&expr));
}


// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let had_error: bool = false;
//     if args.len() > 2 {
//         eprintln!("Usage: jlox [script]");
//         std::process::exit(64);
//     }

//     if args.len() == 2 {
//         run_file(&args[0]);
//     } else {
//         run_prompt();
//     }

// }

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