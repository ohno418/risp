use std::io::prelude::*;
use std::io;

mod mal {
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

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        print!("{}", mal::repl(&input));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn repl_returns_as_is() {
        let input = "asis";
        assert_eq!(crate::mal::repl(input), "asis");
    }
}
