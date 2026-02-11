use std::env;
use std::fs;
use std::io::Read;
use std::io::{self, Write, BufRead};

mod scanner;
mod parser;
mod token;
mod token_type;
mod expr;
mod ast_printer;
mod interpreter;
mod object;
mod stmt;
use scanner::Scanner;
use parser::Parser;

use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::interpreter::RuntimeError;
use crate::token::Token;
use crate::token_type::TokenType;
use std::sync::atomic::{AtomicBool, Ordering};


static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

// fn main() {
// let expr = Expr::Binary(expr::Binary {
//     left: Box::new(Expr::Unary(expr::Unary {
//         op: Token::new(TokenType::Minus, "-", token::Literal::Nil, 1),
//         right: Box::new(Expr::Literal(token::Literal::Number(123.0))),
//     })),
//     op: Token::new(TokenType::Star, "*", token::Literal::Nil, 1),
//     right: Box::new(Expr::Grouping(expr::Grouping {
//         expression: Box::new(Expr::Literal(token::Literal::Number(45.67))),
//     })),
// });


// }


fn main() {
    let args: Vec<String> = env::args().collect();
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
    if HAD_ERROR.load(Ordering::Relaxed) {
        std::process::exit(64);
    }
    if HAD_RUNTIME_ERROR.load(Ordering::Relaxed) {
        std::process::exit(70);
    }
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
    let tokens = scanner.scan_tokens().clone();
    // for token in &tokens {
    //     eprintln!("{}", token);
    // }

    let mut parser = Parser::new(tokens);
    if let Some(expressions) = parser.parse(){
        // let mut printer = ast_printer::AstPrinter;
        // println!("{}", printer.print(&expressions));
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&expressions);
    } else {
        return
    }

}

fn lox_error(token: &Token, message: &str){
    if token.kind == TokenType::Eof{
        report(&token.line, " at end", message)
    } else {
        let _where = &format!("at '{}'", token.lexeme);
        report(&token.line, _where, message)
    }
}

fn report(line: &usize, _where: &str, message: &str){
    eprintln!("line {} Error {}: {}", line, _where, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}


fn runtime_error(err: RuntimeError){
    eprintln!("{}", err);
    HAD_RUNTIME_ERROR.store(true, Ordering::Relaxed);
}
