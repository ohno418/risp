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
            let node = match parse(&tokens) {
                Some(node) => node,
                None => {
                    println!("Cannot parse: {}", &input);
                    continue;
                }
            };
            eval(&node);

            // debug
            println!("{:?}", node);
        }
    }

    fn eval(node: &Node) -> &Node {
        // TODO
        node
    }
}

use crate::risp::repl;

fn main() {
    repl();
}
