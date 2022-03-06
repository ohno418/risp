use std::io::prelude::*;
use std::io;

pub fn read_str() -> String {
    print!("user> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(err) => panic!("{}", err),
    }
}

// TODO
// fn tokenize() {
// }
//
// fn read_form() {
// }
