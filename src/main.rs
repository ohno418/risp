use std::io::prelude::*;
use std::io;
use std::process;

mod risp {
    pub fn repl(input: &str) -> &str {
        print(eval(read(input)))
    }

    fn read(input: &str) -> &str {
        input
    }

    fn eval(input: &str) -> &str {
        input
    }

    fn print(result: &str) -> &str {
        result
    }
}

fn main() {
    loop {
        print!("user> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(nread) => {
                // Exit with Ctrl-D.
                if nread == 0 {
                    print!("\nexit");
                    process::exit(0);
                }
            }
            Err(err) => panic!("{}", err),
        }

        print!("{}", risp::repl(&input));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn repl_returns_as_is() {
        let input = "asis";
        assert_eq!(crate::risp::repl(input), "asis");
    }
}
