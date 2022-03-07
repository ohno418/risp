mod reader;

mod risp {
    use crate::reader::read;

    pub fn repl() {
        loop {
            match read() {
                Some(input) => print(eval(&input)),
                None => continue,
            }
        }
    }

    fn eval(input: &str) -> &str {
        input
    }

    fn print(result: &str) {
        print!("{}", result);
    }
}

use crate::risp::repl;

fn main() {
    repl();
}
