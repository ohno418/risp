use crate::parser::Node;

pub fn eval(ast: &Node) -> Option<String> {
    Some(format!("(debug) {:?}", ast))
}

#[cfg(test)]
mod tests {
    use super::eval;
    use crate::parser::Node::*;

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
