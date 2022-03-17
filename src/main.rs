mod lexer;
mod parser;
mod reader;

mod risp {
    use crate::lexer::tokenize;
    use crate::parser::{parse, Node};
    use crate::reader::{read_input, ReadError};
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
            match parse(&tokens) {
                Some(node) => print(eval(&node)),
                None => {
                    println!("Cannot parse: {}", &input);
                    continue;
                }
            }
        }
    }

    fn eval(node: &Node) -> &Node {
        node
    }

    fn print(result: &Node) {
        println!("{:?}", result);
    }
}

use crate::risp::repl;

fn main() {
    repl();
}
