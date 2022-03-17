use regex::Regex;

pub fn tokenize(input: &str) -> Vec<&str> {
    let re =
        Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###)
            .expect("invalid regex");

    let mut tokens = vec![];
    for caps in re.captures_iter(input) {
        tokens.push(caps.get(1).expect("failed to get a regex capture").as_str());
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::tokenize;

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
