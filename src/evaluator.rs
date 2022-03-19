use crate::parser::Node::{self, *};

pub fn eval(ast: &Node) -> Result<i64, String> {
    match ast {
        Int(num) => Ok(*num),
        List(list) => eval_list(&list),
        _ => {
            println!("(debug) {:?}", ast);
            Err(format!("Cannot eval"))
        }
    }
}

fn eval_list(list: &Vec<Node>) -> Result<i64, String> {
    if list.len() != 3 {
        return Err("Unexpected length of list".to_string());
    }

    if let Sym(sym) = &list[0] {
        let rhs = eval(&list[1])?;
        let lhs = eval(&list[2])?;
        let result = if sym == "+" {
            rhs + lhs
        } else if sym == "-" {
            rhs - lhs
        } else if sym == "*" {
            rhs * lhs
        } else if sym == "/" {
            rhs / lhs
        } else {
            return Err(format!("Unknown operator: {}", sym));
        };
        return Ok(result);
    }

    Err("Cannot eval".to_string())
}

#[cfg(test)]
mod tests {
    use super::eval;
    use crate::parser::Node::*;

    #[test]
    fn eval_single_number() {
        let ast = Int(42);
        let actual = eval(&ast).unwrap();
        let expected = 42;
        assert_eq!(actual, expected);
    }

    #[test]
    fn eval_arithmetic_list() {
        let ast = List(vec![Sym("+".to_string()), Int(12), Int(23)]);
        let actual = eval(&ast).unwrap();
        let expected = 35;
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
        let expected = 70;
        assert_eq!(actual, expected);
    }
}
