#![cfg(test)]

use std::{rc::Rc, ops::Deref};
use crate::{lexer::Lexer, parser::Parser, ast::{Program, Statement, LetStatement}};

#[test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

    let lexer   = Lexer::new(input);
    let parser  = Parser::new(lexer.tokenize());
    let program = parser.parse(); 
    test_program_length(&program);
}

fn test_let_statement(stmt: Rc<dyn Statement>) {
    todo!()
}

fn test_program_length(prg: &Program) {
    if prg.statements.len() != 3 {
        panic!(
            "program.statements dosen't contain 3 statements: got {}",
            prg.statements.len()
        );
    }
}
