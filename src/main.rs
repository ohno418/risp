mod evaluator;
mod lexer;
mod parser;
mod reader;

use crate::evaluator::eval;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::reader::{read_input, ReadError};
use std::process;

fn main() {
    loop {
        let input = match read_input() {
            Ok(input) => input.trim().to_string(),
            Err(ReadError::CtrlD) => {
                println!("\nexit");
                process::exit(0);
            }
            Err(ReadError::EmptyInput) => continue,
        };

        let tokens = tokenize(&input);
        let ast = match parse(&tokens) {
            Some(ast) => ast,
            None => {
                println!("Cannot parse: {}", &input);
                continue;
            }
        };
        match eval(&ast) {
            Some(result) => println!("{}", result),
            None => {
                println!("Cannot evaluate: {}", input);
                continue;
            }
        }
    }
}
