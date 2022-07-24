use std::io::{Stdin, stdout, Write};

use crate::{
    lexer::Lexer,
    parser::Parser, ast::Node,
    eval::{Eval, env::Env},
    object::ObjectTrait
};

pub enum ReplExecKind {
    Token,
    Ast,
    String,
    Eval,
}

pub struct Repl {
    cin: Stdin,
}

impl Repl {
    pub fn new(cin: Stdin) -> Repl {
        Repl { cin }
    }

    pub fn start(&mut self, kind: ReplExecKind) {
        let mut env = Env::new();
        loop {
            print!(">> ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            self.cin.read_line(&mut buf).unwrap();
            let lexer  = Lexer::new(buf.as_str());

            match kind {
                ReplExecKind::Token => {
                    println!("{:#?}", lexer.tokenize());
                }
                ReplExecKind::Ast => {
                    let parser = Parser::new(lexer.tokenize());
                    match parser.parse() {
                        Ok(prg) => {
                            for stmt in prg.statements.iter() {
                                println!("{:#?}", stmt);
                            }
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
                }
                ReplExecKind::String => {
                    let parser = Parser::new(lexer.tokenize());
                    match parser.parse() {
                        Ok(prg) => {
                            for stmt in prg.statements.iter() {
                                println!("{}", stmt.string());
                            }
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
                }
                ReplExecKind::Eval => {
                    let parser = Parser::new(lexer.tokenize());
                    match parser.parse() {
                        Ok(prg) => {
                            println!("{}", Eval::new(&mut env).eval(prg).inspect());
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
                }
            }
        }
    }
}
