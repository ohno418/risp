mod reader;

mod risp {
    use crate::reader::read_str;

    pub fn repl() {
        let input = read_str();
        print(eval(&input));
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
