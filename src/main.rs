mod token;
mod lexer;
mod ast;
mod parser;
mod repl;

use crate::repl::Repl;

fn main() {
    println!("This is rmonkey programming language!");
    Repl::new().start();
}
