use crate::parser::Node;

pub fn eval(ast: &Node) -> String {
    format!("(debug) {:?}", ast)
}
