use std::io;
use std::io::prelude::*;

pub enum ReadError {
    CtrlD,
    EmptyInput,
}

pub fn read_input() -> Result<String, ReadError> {
    print!("user> ");
    io::stdout().flush().expect("failed to flush");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(nread) => match nread {
            0 => Err(ReadError::CtrlD),
            1 => Err(ReadError::EmptyInput), // only newline
            _ => Ok(input),
        },
        Err(err) => panic!("{}", err),
    }
}
