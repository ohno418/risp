mod reader;
mod types;

mod risp {
    use crate::reader::read;
    use crate::types::Val;

    pub fn repl() {
        loop {
            match read() {
                Some(val) => print(eval(&val)),
                None => continue,
            }
        }
    }

    fn eval(val: &Val) -> &Val {
        val
    }

    fn print(result: &Val) {
        // TODO
        // print!("{}", result);
    }
}

use crate::risp::repl;

fn main() {
    repl();
}
