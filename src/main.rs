mod token;
mod lexer;
mod ast;
mod parser;
mod repl;
mod object;
mod eval;

use std::io::{stdout, Write, stdin};

use crate::repl::{Repl, ReplExecKind};

fn main() {
    println!("This is rmonkey programming language!");
    println!("Which type of interpreter do you want to use?");
    println!("  1. Display tokens");
    println!("  2. Display ast");
    println!("  3. Display string of ast");
    println!("  4. Display evaluated value");

    let cin = stdin();
    let kind = loop {
        print!("num of type: ");
        stdout().flush().unwrap();

        let mut input = String::new();
        cin.read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(value) => match value {
                1 => break ReplExecKind::Token,
                2 => break ReplExecKind::Ast,
                3 => break ReplExecKind::String,
                4 => break ReplExecKind::Eval,
                i => println!("Invalid input: {}", i),
            }
            Err(e) => println!("Invalid input: {}", e),
        }
    };
    Repl::new(cin).start(kind);
}
