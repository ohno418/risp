mod lexer;
mod parser;
mod reader;
mod evaluator;

mod risp {
    use crate::lexer::tokenize;
    use crate::parser::parse;
    use crate::reader::{read_input, ReadError};
    use crate::evaluator::eval;
    use std::process;

    pub fn repl() {
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
            let node = match parse(&tokens) {
                Some(node) => node,
                None => {
                    println!("Cannot parse: {}", &input);
                    continue;
                }
            };
            let result = eval(&node);
            println!("{}", result);
        }
    }
}

use crate::risp::repl;

fn main() {
    repl();
}
