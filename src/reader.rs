use crate::types::Val;
use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::process;

// TODO: Result<Val, InputError>
// Read a user input from stdin, return its AST.
pub fn read() -> Option<Val> {
    if let Some(input) = read_user_input() {
        let tokens = tokenize(&input);
        let val = parse(&tokens);
        return Some(val);
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

fn parse(tokens: &Vec<String>) -> Val {
    let int_re = Regex::new(r"[0-9]+").unwrap();

    match tokens.first() {
        Some(tok) => {
            if int_re.is_match(&tok) {
                return Val::Int(tokens[0].parse().unwrap());
            }

            Val::Sym(tok.to_string())
        }
        None => panic!("No token to parse."),
    }
}

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

    mod parse {
        use crate::reader::parse;
        use crate::types::*;

        #[test]
        fn parse_number() -> Result<(), String> {
            let tokens = vec![String::from("42")];
            let expected = 42;
            if let Val::Int(n) = parse(&tokens) {
                if n == expected {
                    return Ok(());
                } else {
                    return Err(format!("expected {}, but got {}", expected, n));
                }
            }

            Err("expected Val::Int".to_string())
        }

        #[test]
        fn parse_symbol() -> Result<(), String> {
            let tokens = vec![String::from("abc")];
            let expected = "abc";
            if let Val::Sym(s) = parse(&tokens) {
                if s == expected {
                    return Ok(());
                } else {
                    return Err(format!("expected {}, but got {}", expected, s));
                }
            }

            Err("expected Val::Sym".to_string())
        }
    }
}
