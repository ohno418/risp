use crate::parser::Node::{self, *};

pub fn eval(ast: &Node) -> Result<i64, String> {
    match ast {
        Int(num) => Ok(*num),
        List(list) => eval_list(&list),
        _ => Err(format!("Cannot eval")),
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
        // (+ 12 23)
        let ast = List(vec![Sym("+".to_string()), Int(12), Int(23)]);
        let actual = eval(&ast).unwrap();
        let expected = 35;
        assert_eq!(actual, expected);

        // (- 43 32)
        let ast = List(vec![Sym("-".to_string()), Int(43), Int(32)]);
        let actual = eval(&ast).unwrap();
        let expected = 11;
        assert_eq!(actual, expected);

        // (* 21 2)
        let ast = List(vec![Sym("*".to_string()), Int(21), Int(2)]);
        let actual = eval(&ast).unwrap();
        let expected = 42;
        assert_eq!(actual, expected);

        // (/ 21 3)
        let ast = List(vec![Sym("/".to_string()), Int(21), Int(3)]);
        let actual = eval(&ast).unwrap();
        let expected = 7;
        assert_eq!(actual, expected);
    }

    #[test]
    fn eval_nested_arithmetic_list() {
        // (* 2 (+ 12 23))
        let ast = List(vec![
            Sym("*".to_string()),
            Int(2),
            List(vec![Sym("+".to_string()), Int(12), Int(23)]),
        ]);
        let actual = eval(&ast).unwrap();
        let expected = 70;
        assert_eq!(actual, expected);

        // (* (+ 1 2) (- (/ 21 3) 3))
        let ast = List(vec![
            Sym("*".to_string()),
            List(vec![Sym("+".to_string()), Int(1), Int(2)]),
            List(vec![
                Sym("-".to_string()),
                List(vec![Sym("/".to_string()), Int(21), Int(3)]),
                Int(3),
            ]),
        ]);
        let actual = eval(&ast).unwrap();
        let expected = 12;
        assert_eq!(actual, expected);
    }
}
