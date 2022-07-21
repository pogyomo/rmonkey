use std::io::{stdin, Stdin, stdout, Write};

use crate::{lexer::Lexer, parser::Parser, ast::Node};

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
            for stmt in parser.parse().statements.iter() {
                println!("{}", stmt.string());
            }
        }
    }
}
