use crate::parser::Node::{self, *};

pub fn eval(ast: &Node) -> Result<String, String> {
    match ast {
        Int(num) => Ok(num.to_string()),
        List(list) => {
            if list.len() != 3 {
                return Err("Unexpected length of list".to_string());
            }

            if let (Sym(sym), Int(r), Int(l)) = (&list[0], &list[1], &list[2]) {
                let result: i64 = if sym == "+" {
                    r + l
                } else if sym == "-" {
                    r - l
                } else if sym == "*" {
                    r * l
                } else if sym == "/" {
                    r / l
                } else {
                    return Err(format!("Unknown operator: {}", sym));
                };
                return Ok(result.to_string());
            }

            Err("Cannot eval".to_string())
        }
        _ => {
            println!("(debug) {:?}", ast);
            Err(format!("Cannot eval"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval;
    use crate::parser::Node::*;

    #[test]
    fn eval_single_number() {
        let ast = Int(42);
        let actual = eval(&ast).unwrap();
        let expected = "42";
        assert_eq!(actual, expected);
    }

    #[test]
    fn eval_arithmetic_list() {
        let ast = List(vec![Sym("+".to_string()), Int(12), Int(23)]);
        let actual = eval(&ast).unwrap();
        let expected = "35";
        assert_eq!(actual, expected);
    }

    #[test]
    fn eval_nested_arithmetic_list() {
        let ast = List(vec![
            Sym("*".to_string()),
            Int(2),
            List(vec![Sym("+".to_string()), Int(12), Int(23)]),
        ]);
        let actual = eval(&ast).unwrap();
        let expected = "70";
        assert_eq!(actual, expected);
    }
}
