mod reader;

mod risp {
    use crate::reader::read_str;
    use std::process;

    pub fn repl() {
        loop {
            let input = read_str();
            match input.len() {
                0 => {
                    // Exit with Ctrl-D.
                    print!("\nexit");
                    process::exit(0);
                }
                1 => {
                    // Empty input (only newline character)
                    if input.as_bytes()[0] == b'\n' {
                        continue;
                    }

                    eprintln!("unknown input: {}", input);
                    process::exit(1);
                }
                _ => print(eval(&input))
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
