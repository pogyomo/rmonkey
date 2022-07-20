mod token;
mod lexer;
mod parser;
mod repl;
mod ast;

use crate::repl::Repl;

fn main() {
    println!("This is rmonkey programming language!");
    println!("Feel free to type in commands");
    Repl::new().start();
}
