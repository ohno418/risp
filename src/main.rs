mod reader;
mod types;

mod risp {
    use crate::reader::read;
    use crate::types::{Node, ReadError};
    use std::process;

    pub fn repl() {
        loop {
            match read() {
                Ok(node) => print(eval(&node)),
                Err(ReadError::CtrlD) => {
                    // Exit with Ctrl-D.
                    println!("\nexit");
                    process::exit(0);
                }
                Err(ReadError::EmptyInput) => continue,
                Err(ReadError::CannotParse(input)) => {
                    println!("Cannot parse: {}", input);
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
