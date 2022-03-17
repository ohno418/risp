// AST node
#[derive(Debug)]
pub enum Node {
    Int(i64),
    Sym(String),
    List(Vec<Node>),
}

pub enum ReadError {
    CtrlD,
    EmptyInput,
    CannotParse(String),
}
