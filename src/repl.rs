use std::{io::{Stdin, stdout, Write}, cell::RefCell, rc::Rc};

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
        let env = Env::new();
        let mut eval = Eval::new(Rc::new(RefCell::new(env)));
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
                            println!("{}", eval.eval(prg).inspect());
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
