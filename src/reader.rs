use regex::Regex;
use std::io;
use std::io::prelude::*;

// TODO
// Read a user input from stdin, return its AST.
pub fn read_str() -> String {
    print!("user> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        panic!("{}", err);
    }

    // TODO
    let tokens = tokenize(&input);

    input
}

fn tokenize(input: &str) -> Vec<String> {
    let re =
        Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###)
            .unwrap();

    let mut tokens = vec![];
    for caps in re.captures_iter(input) {
        tokens.push(String::from(caps.get(1).unwrap().as_str()));
    }
    return tokens;
}

// TODO
// fn read_form() {
// }

#[cfg(test)]
mod test_tokenize {
    use super::tokenize;

    #[test]
    fn tokenize_input_0() {
        let actual = tokenize("(+ 2 3)");
        let expected = vec!["(", "+", "2", "3", ")"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn tokenize_input_1() {
        let actual = tokenize("(* 2 (+ 31 4))");
        let expected = vec!["(", "*", "2", "(", "+", "31", "4", ")", ")"];
        assert_eq!(actual, expected);
    }
}
