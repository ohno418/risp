#[derive(Debug)]
pub enum Val {
    Int(i64),
    Sym(String),
    List(Vec<Val>),
}

pub enum ReadError {
    CtrlD,
    EmptyInput,
    CannotParse(String),
}
