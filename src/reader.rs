use crate::types::{Node, ReadError};
use regex::Regex;
use std::io;
use std::io::prelude::*;

// Read user input from stdin, return its AST.
pub fn read() -> Result<Node, ReadError> {
    let input = read_user_input()?.trim().to_string();
    let tokens = tokenize(&input);
    match parse(&tokens) {
        Some(ast) => Ok(ast),
        None => Err(ReadError::CannotParse(input)),
    }
}

fn read_user_input() -> Result<String, ReadError> {
    print!("user> ");
    io::stdout().flush().expect("failed to flush");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(nread) => match nread {
            0 => Err(ReadError::CtrlD),
            1 => Err(ReadError::EmptyInput), // only newline
            _ => Ok(input),
        },
        Err(err) => panic!("{}", err),
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    let re =
        Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###)
            .expect("invalid regex");

    let mut tokens = vec![];
    for caps in re.captures_iter(input) {
        tokens.push(caps.get(1).expect("failed to get a regex capture").as_str());
    }
    return tokens;
}

// <expr> ::= <list> | <atom>
fn parse(tokens: &[&str]) -> Option<Node> {
    let &first = tokens.first()?;

    if first == "(" {
        parse_list(&tokens)
    } else {
        parse_atom(first)
    }
}

// <list> ::= "(" <expr>* ")"
fn parse_list(tokens: &[&str]) -> Option<Node> {
    if *tokens.get(0)? != "(" {
        return None;
    }

    let mut inner: Vec<Node> = vec![];
    let mut rest: &[&str] = &tokens[1..];
    loop {
        if *rest.get(0)? == ")" {
            break;
        }

        let node = parse(rest)?;
        let tokens_to_consume = match &node {
            Node::List(list) => list.len() + 2,
            _ => 1,
        };
        inner.push(node);
        rest = &rest[tokens_to_consume..];
    }

    Some(Node::List(inner))
}

// <atom> ::= <number> | <symbol>
fn parse_atom(token: &str) -> Option<Node> {
    // number
    let int_re = Regex::new(r"[0-9]+").expect("invalid regex");
    if int_re.is_match(token) {
        return Some(Node::Int(token.parse().expect("failed to parse a token")));
    }

    // symbol
    return Some(Node::Sym(token.to_string()));
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
            let tokens = vec!["42"];
            let expected = 42;
            if let Some(Node::Int(n)) = parse(&tokens) {
                if n == expected {
                    return Ok(());
                } else {
                    return Err(format!("expected {}, but got {}", expected, n));
                }
            }

            Err("expected Node::Int".to_string())
        }

        #[test]
        fn parse_symbol() -> Result<(), String> {
            let tokens = vec!["abc"];
            let expected = "abc";
            if let Some(Node::Sym(s)) = parse(&tokens) {
                if s == expected {
                    return Ok(());
                } else {
                    return Err(format!("expected {}, but got {}", expected, s));
                }
            }

            Err("expected Node::Sym".to_string())
        }

        #[test]
        fn parse_list() -> Result<(), String> {
            let tokens = vec!["(", "*", "12", "23", ")"];
            if let Some(Node::List(inner)) = parse(&tokens) {
                if let [Node::Sym(s), Node::Int(12), Node::Int(23)] = inner.as_slice() {
                    if s == "*" {
                        return Ok(());
                    }
                }
            }
            Err("parse wrong".to_string())
        }

        #[test]
        fn parse_nested_list() -> Result<(), String> {
            let tokens = vec!["(", "*", "12", "(", "+", "23", "34", ")", ")"];
            if let Some(Node::List(inner0)) = parse(&tokens) {
                if let [Node::Sym(s0), Node::Int(12), Node::List(inner1)] = inner0.as_slice() {
                    if let [Node::Sym(s1), Node::Int(23), Node::Int(34)] = inner1.as_slice() {
                        if s0 == "*" && s1 == "+" {
                            return Ok(());
                        }
                    }
                }
            }
            Err("parse wrong".to_string())
        }
    }
}
