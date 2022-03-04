mod risp {
    use std::io::prelude::*;
    use std::io;
    use std::process;

    pub fn repl() {
        print(eval(&read()));
    }

    fn read() -> String {
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

                // Empty input
                // if nread == 1 {
                //     continue;
                // }
            },
            Err(err) => panic!("{}", err),
        }

        input
    }

    fn eval(input: &str) -> &str {
        input
    }

    fn print(result: &str) {
        print!("{}", result);
    }
}

fn main() {
    use crate::risp::repl;

    loop {
        repl();
    }
}

#[cfg(test)]
mod test {
    use crate::risp::repl;

    #[test]
    fn repl_returns_as_is() {
        let input = "asis";
        assert_eq!(repl(input), "asis");
    }
}
