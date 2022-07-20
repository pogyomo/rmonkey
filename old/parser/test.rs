#![cfg(test)]
use crate::{lexer::Lexer, ast::{Statement, Node, Expression, Program}, token::Token};
use super::Parser;

#[test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

    let lexer   = Lexer::new(input);
    let parser  = Parser::new(lexer.tokenize());
    let program = parser.parse(); 
    check_parse_error(&parser);

    if program.statements.len() != 3 {
        panic!(
            "program.statements dosen't contain 3 statements: got {}",
            program.statements.len()
        );
    }

    let tests = vec![ "x", "y", "foobar" ];
    for (statement, name) in program.statements.iter().zip(&tests) {
        match statement {
            Statement::LetStatement { ident, .. } => {
                if ident.literal() != *name {
                    panic!("The name of identifier is not {}: got {}", name, ident.literal());
                }
            }
            _ => {
                panic!(
                    "Literal is not {}: got {}",
                    Statement::LetStatement { 
                        token: Token::Let,
                        ident: Expression::Identifier { 
                            token: Token::Ident(""),
                        },
                        expression: Expression::Dummy,
                    }.literal(),
                    statement.literal()
                );
            }
        }
    }
}

#[test]
fn test_return_statements() {
    let input = "
        return 5;
        return 10;
        return 838383;";

    let lexer   = Lexer::new(input);
    let parser  = Parser::new(lexer.tokenize());
    let program = parser.parse(); 
    check_parse_error(&parser);

    if program.statements.len() != 3 {
        panic!(
            "program.statements dosen't contain 3 statements: got {}",
            program.statements.len()
        );
    }

    for statement in program.statements.iter() {
        match statement {
            Statement::ReturnStatement { .. } => (),
            _ => {
                panic!(
                    "Literal is not {}: got {}",
                    Statement::ReturnStatement { 
                        token: Token::Return,
                        ret_value: Expression::Dummy
                    }.literal(),
                    statement.literal()
                );
            }
        }
    }
}

#[test]
fn test_string() {
    let result = "let a = b;";
    let mut ast = Program::new();
    ast.statements.push(
    Statement::LetStatement {
        token: Token::Let,
        ident: Expression::Identifier {
            token: Token::Ident("a")
        },
        expression: Expression::Identifier {
            token: Token::Ident("b")
        }
    });
    if ast.string().as_str() != result {
        panic!("String is different: expect {}, got {}", result, ast.string());
    }
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lexer   = Lexer::new(input);
    let parser  = Parser::new(lexer.tokenize());
    let program = parser.parse();
    check_parse_error(&parser);

    if program.statements.len() != 1 {
        panic!(
            "program.statements dosen't contain only a statement: got {}",
            program.statements.len()
        );
    }

    for statement in program.statements.iter() {
        match statement {
            Statement::ExpressionStatement { expression, .. } => {
                match expression {
                    Expression::Identifier { .. } => {
                        if expression.literal() != "foobar" {
                            panic!(
                                "Identifier is not foobar: got {}",
                                expression.literal()
                            );
                        }
                    }
                    _ => panic!("Expression is not Identifier"),
                }
            }
            _ => {
                panic!(
                    "Literal is not {}: got {}",
                    Statement::ExpressionStatement { 
                        token: Token::Ident(""),
                        expression: Expression::Dummy,
                    }.literal(),
                    statement.literal()
                );
            }
        }
    }
}

fn check_parse_error(parser: &Parser) {
    let error_num = parser.error.borrow().len();
    if error_num != 0 {
        let mut err_msg = String::new();
        err_msg += format!("\n>> Parser has {} errors\n", error_num).as_str();
        for msg in parser.error.borrow().iter() {
            err_msg += format!(">> {}\n", msg).as_str();
        }
        panic!("{}", err_msg);
    }
}
