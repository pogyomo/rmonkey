use std::io::{stdin, Stdin, stdout, Write};

use crate::{lexer::Lexer, parser::Parser};

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
            /*
            for token in Lexer::new(buf.as_str()).tokenize().iter() {
                println!("[Token: {:?}, Literal: {:?}]", token, token.literal_of());
            }
            */
            let lexer  = Lexer::new(buf.as_str());
            let parser = Parser::new(lexer.tokenize());
            let program = parser.parse().unwrap_or_else(|err| panic!("{}", err));
            for stmt in program.statements.iter() {
                println!("{:#?}", stmt);
            }
        }
    }
}
