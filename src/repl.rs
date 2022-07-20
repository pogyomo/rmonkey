use std::io::{stdin, Stdin, stdout, Write};

use crate::lexer::Lexer;

pub struct Repl {
    cin: Stdin,
}

impl Repl {
    pub fn new() -> Repl {
        Repl { cin: stdin() }
    }

    pub fn start(&mut self) {
        loop {
            print!(">> ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            self.cin.read_line(&mut buf).unwrap();
            for token in Lexer::new(buf.as_str()).tokenize().iter() {
                println!("{:?}", token);
            }
        }
    }
}
