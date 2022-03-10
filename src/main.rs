mod reader;
mod types;

mod risp {
    use crate::reader::{self, read};
    use crate::types::Val;
    use std::process;

    pub fn repl() {
        loop {
            match read() {
                Ok(val) => print(eval(&val)),
                Err(reader::Error::CtrlD) => {
                    // Exit with Ctrl-D.
                    print!("\nexit");
                    process::exit(0);
                }
                Err(reader::Error::EmptyInput) => continue,
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
