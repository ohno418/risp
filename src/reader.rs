use std::io::prelude::*;
use std::io;
use std::process;

pub fn read_str() -> String {
    print!("user> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(nread) => {
            // Exit with Ctrl-D.
            if nread == 0 {
                print!("\nexit");
                process::exit(0);
            }

            // Empty input
            // if nread == 1 {
            //     continue;
            // }
        },
        Err(err) => panic!("{}", err),
    }

    input
}

// TODO
// fn tokenize() {
// }
//
// fn read_form() {
// }
