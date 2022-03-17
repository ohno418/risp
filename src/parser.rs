use regex::Regex;

// AST node
#[derive(Debug)]
pub enum Node {
    Int(i64),
    Sym(String),
    List(Vec<Node>),
}

// <expr> ::= <list> | <atom>
pub fn parse(tokens: &[&str]) -> Option<Node> {
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
    use crate::parser::{parse, Node::*};

    #[test]
    fn parse_number() -> Result<(), String> {
        let tokens = vec!["42"];
        let expected = 42;
        if let Some(Int(n)) = parse(&tokens) {
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
        if let Some(Sym(s)) = parse(&tokens) {
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
        if let Some(List(inner)) = parse(&tokens) {
            if let [Sym(s), Int(12), Int(23)] = inner.as_slice() {
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
        if let Some(List(inner0)) = parse(&tokens) {
            if let [Sym(s0), Int(12), List(inner1)] = inner0.as_slice() {
                if let [Sym(s1), Int(23), Int(34)] = inner1.as_slice() {
                    if s0 == "*" && s1 == "+" {
                        return Ok(());
                    }
                }
            }
        }
        Err("parse wrong".to_string())
    }
}
