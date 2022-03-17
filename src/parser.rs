use regex::Regex;

// AST node
#[derive(Debug, PartialEq)]
pub enum Node {
    Int(i64),
    Sym(String),
    List(Vec<Node>),
}

// <expr> ::= <list> | <atom>
pub fn parse(tokens: &[&str]) -> Option<Node> {
    let first = peek_token(tokens)?;

    if first == "(" {
        parse_list(&tokens)
    } else {
        parse_atom(first)
    }
}

// <list> ::= "(" <expr>* ")"
fn parse_list(tokens: &[&str]) -> Option<Node> {
    if peek_token(tokens)? != "(" {
        return None;
    }

    let mut inner: Vec<Node> = vec![];
    let mut rest: &[&str] = &tokens[1..];
    loop {
        if peek_token(rest)? == ")" {
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

fn peek_token<'a>(tokens: &'a [&str]) -> Option<&'a str> {
    Some(*tokens.get(0)?)
}

#[cfg(test)]
mod tests {
    use super::{parse, Node::*};

    #[test]
    fn parse_number() {
        let actual = parse(&vec!["42"]);
        let expected = Int(42);
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn parse_symbol() {
        let actual = parse(&vec!["abc"]);
        let expected = Sym("abc".to_string());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn parse_list() {
        let actual = parse(&vec!["(", "*", "12", "23", ")"]);
        let expected = List(vec![Sym("*".to_string()), Int(12), Int(23)]);
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn parse_nested_list() {
        let actual = parse(&vec!["(", "*", "12", "(", "+", "23", "34", ")", ")"]);
        let expected = List(vec![
            Sym("*".to_string()),
            Int(12),
            List(vec![Sym("+".to_string()), Int(23), Int(34)]),
        ]);
        assert_eq!(actual.unwrap(), expected);
    }
}
