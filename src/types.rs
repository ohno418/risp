pub enum Val {
    Int(i64),
    Sym(String),
    // first and rest
    List(Box<Val>, Box<Val>),
}

pub enum ReadError {
    CtrlD,
    EmptyInput,
}
