use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::process;

// TODO
// Read a user input from stdin, return its AST.
pub fn read() -> Option<String> {
    if let Some(input) = read_user_input() {
        let tokens = tokenize(&input);
        // TODO
        // parse_tokens
        return Some(input);
    }

    None
}

fn read_user_input() -> Option<String> {
    print!("user> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(nread) => {
            match nread {
                0 => {
                    // Exit with Ctrl-D.
                    print!("\nexit");
                    process::exit(0);
                }
                1 => {
                    // Empty input (only newline character)
                    return None;
                }
                _ => Some(input),
            }
        }
        Err(err) => panic!("{}", err),
    }
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
mod tests {
    mod tokenize {
        use crate::reader::tokenize;

        #[test]
        fn tokenize_number() {
            let actual = tokenize("123");
            let expected = vec!["123"];
            assert_eq!(actual, expected);
        }

        #[test]
        fn tokenize_symbol() {
            let actual = tokenize("abc");
            let expected = vec!["abc"];
            assert_eq!(actual, expected);
        }

        #[test]
        fn tokenize_number_list() {
            let actual = tokenize("(12 23 34)");
            let expected = vec!["(", "12", "23", "34", ")"];
            assert_eq!(actual, expected);
        }

        #[test]
        fn tokenize_symbol_started_list() {
            let actual = tokenize("(mul 2 3)");
            let expected = vec!["(", "mul", "2", "3", ")"];
            assert_eq!(actual, expected);
        }

        #[test]
        fn tokenize_addition_list() {
            let actual = tokenize("(+ 2 3)");
            let expected = vec!["(", "+", "2", "3", ")"];
            assert_eq!(actual, expected);
        }

        #[test]
        fn tokenize_nested_list() {
            let actual = tokenize("(* 2 (+ 31 4))");
            let expected = vec!["(", "*", "2", "(", "+", "31", "4", ")", ")"];
            assert_eq!(actual, expected);
        }
    }
}
