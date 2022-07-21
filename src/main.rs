mod token;
mod lexer;
mod repl;
mod ast;
mod parser;

use crate::repl::Repl;

fn main() {
    println!("This is rmonkey programming language!");
    println!("Feel free to type in commands");
    Repl::new().start();
}
