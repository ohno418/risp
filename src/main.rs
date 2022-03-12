mod reader;
mod types;

mod risp {
    use crate::reader::read;
    use crate::types::{ReadError, Val};
    use std::process;

    pub fn repl() {
        loop {
            match read() {
                Ok(val) => print(eval(&val)),
                Err(ReadError::CtrlD) => {
                    // Exit with Ctrl-D.
                    println!("\nexit");
                    process::exit(0);
                }
                Err(ReadError::EmptyInput) => continue,
            }
        }
    }

    fn eval(val: &Val) -> &Val {
        val
    }

    fn print(_result: &Val) {
        // TODO
        // print!("{}", result);
    }
}

use crate::risp::repl;

fn main() {
    repl();
}
