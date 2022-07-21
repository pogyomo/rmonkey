#![cfg(test)]

use crate::{lexer::Lexer, ast::{Statement, Node, Expression, Program}, token::Token};
use super::Parser;
use std::rc::Rc;

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
                test_identifier(ident.clone(), name);
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
                        test_identifier(expression.clone(), "foobar");
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

#[test]
fn test_integer_literal_expression() {
    let input = "5;";

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
                    Expression::Integer { value, .. } => {
                        if expression.literal() != "5" {
                            panic!(
                                "Literal is not \"5\": got {}",
                                expression.literal()
                            );
                        }

                        if *value != 5 {
                            panic!(
                                "Value is not 5: got {}",
                                value
                            );
                        }
                    }
                    _ => panic!("Expression is not Integer"),
                }
            }
            _ => {
                panic!(
                    "Literal is not {}: got {}",
                    Statement::ExpressionStatement { 
                        token: Token::Int("5"),
                        expression: Expression::Integer {
                            token: Token::Int("5"),
                            value: 5
                        }
                    }.literal(),
                    statement.literal()
                );
            }
        }
    }
}

#[test]
fn test_parsing_prefix_expressions() {
    struct PrefixTest<'a> {
        input: &'a str,
        operator: &'a str,
        value: i64,
    }

    let tests = vec![
        PrefixTest { input: "!5;",  operator: "!", value: 5  },
        PrefixTest { input: "-15;", operator: "-", value: 15 },
    ];

    for test in &tests {
        let lexer   = Lexer::new(test.input);
        let parser  = Parser::new(lexer.tokenize());
        let program = parser.parse();
        check_parse_error(&parser);

        if program.statements.len() != 1 {
            panic!(
                "program.statements dosen't contain only a statement: got {}",
                program.statements.len()
            );
        }

        let statement = &program.statements[0];
        match statement {
            Statement::ExpressionStatement { expression, .. } => {
                match expression {
                    Expression::PrefixExpression { right: ref exp, .. } => {
                        if expression.literal() != test.operator {
                            panic!(
                                "Literal is not \"{}\": got {}",
                                test.operator, expression.literal()
                            );
                        }

                        test_integer_literal(exp.as_ref().clone(), test.value);
                    }
                    _ => panic!("Expression is not PrefixExpression"),
                }
            }
            _ => {
                panic!(
                    "Literal is not {}: got {}",
                    Statement::ExpressionStatement { 
                        token: Token::Int("5"),
                        expression: Expression::Integer {
                            token: Token::Int("5"),
                            value: 5
                        }
                    }.literal(),
                    statement.literal()
                );
            }
        }

    }
}

#[test]
fn test_parsing_infix_expressions() {
    struct InfixTest<'a> {
        input: &'a str,
        left: i64,
        operator: &'a str,
        right: i64,
    }

    impl<'a> InfixTest<'a> {
        pub fn new(input: &'a str, left: i64, right: i64, operator: &'a str) -> InfixTest<'a> {
            InfixTest { input, left, operator, right }
        }
    }

    let tests = vec![
        InfixTest::new("5 + 5;",  5, 5, "+"),
        InfixTest::new("5 - 5;",  5, 5, "-"),
        InfixTest::new("5 * 5;",  5, 5, "*"),
        InfixTest::new("5 / 5;",  5, 5, "/"),
        InfixTest::new("5 > 5;",  5, 5, ">"),
        InfixTest::new("5 < 5;",  5, 5, "<"),
        InfixTest::new("5 == 5;", 5, 5, "=="),
        InfixTest::new("5 != 5;", 5, 5, "!="),
    ];

    for test in &tests {
        let lexer   = Lexer::new(test.input);
        let parser  = Parser::new(lexer.tokenize());
        let program = parser.parse();
        check_parse_error(&parser);

        if program.statements.len() != 1 {
            panic!(
                "program.statements dosen't contain only a statement: got {}",
                program.statements.len()
            );
        }

        let statement = &program.statements[0];
        match statement {
            Statement::ExpressionStatement { expression, .. } => {
                match expression {
                    Expression::InfixExpression { ref left, ref right, .. } => {
                        if expression.literal() != test.operator {
                            panic!(
                                "Literal is not \"{}\": got {}",
                                test.operator, expression.literal()
                            );
                        }

                        test_integer_literal(left.as_ref().clone(),  test.left);
                        test_integer_literal(right.as_ref().clone(), test.right);
                    }
                    _ => panic!("Expression is not InfixExpression"),
                }
            }
            _ => {
                panic!(
                    "Literal is not {}: got {}",
                    Statement::ExpressionStatement { 
                        token: Token::Int("5"),
                        expression: Expression::Integer {
                            token: Token::Int("5"),
                            value: 5
                        }
                    }.literal(),
                    statement.literal()
                );
            }
        }

    }
}

#[test]
fn test_bool() {
    let input = "
        true;
        false;";

    let lexer   = Lexer::new(input);
    let parser  = Parser::new(lexer.tokenize());
    let program = parser.parse(); 
    check_parse_error(&parser);

    if program.statements.len() != 2 {
        panic!(
            "program.statements dosen't contain 3 statements: got {}",
            program.statements.len()
        );
    }

    let tests = vec![
        "true",
        "false",
    ];

    for (statement, test) in program.statements.iter().zip(&tests) {
        match statement {
            Statement::ExpressionStatement { expression, .. } => {
                match expression {
                    Expression::Boolean { token, .. } => {
                        if token.literal_of() != *test {
                            panic!("Value of boolean is not {}: got {}", test, token.literal_of());
                        }
                    }
                    _ => panic!("Expression is not Boolean: got {:?}", expression),
                }
            }
            _ => {
                panic!("Statement is not ExpressionStatement: got {:?}", statement);
            }
        }
    }
}

#[test]
fn test_parentheses() {
    let input = "1 * (2 + 3);";

    let lexer   = Lexer::new(input);
    let parser  = Parser::new(lexer.tokenize());
    let program = parser.parse(); 
    check_parse_error(&parser);

    if program.statements.len() != 1 {
        panic!(
            "program.statements dosen't contain 1 statements: got {}",
            program.statements.len()
        );
    }

    let statement = &program.statements[0];
    match statement {
        Statement::ExpressionStatement { expression, .. } => {
            let test = Expression::InfixExpression {
                token: Token::Asterisk,
                left: Rc::new(Expression::Integer {
                    token: Token::Ident("1"),
                    value: 5
                }),
                right: Rc::new(Expression::InfixExpression {
                    token: Token::Plus,
                    left: Rc::new(Expression::Integer {
                        token: Token::Ident("2"),
                        value: 2
                    }),
                    right: Rc::new(Expression::Integer {
                        token: Token::Ident("3"),
                        value: 3
                    })
                })
            };

            if test.string() != expression.string() {
                panic!("Expression is different: expect {}, got {}", test.string(), expression.string());
            }
        }
        _ => {
            panic!("Statement is not ExpressionStatement: got {:?}", statement);
        }
    }
}

fn test_integer_literal(expression: Expression, value: i64) {
    match expression {
        Expression::Integer { value: v, .. } => if v != value {
            panic!("Value is not {}: got {}", value, v);
        }
        _ => panic!("This expression is not Integer: got {:?}", expression),
    }
}

fn test_identifier(expression: Expression, name: &str) {
    match expression {
        Expression::Identifier { token } => if token.literal_of() != name {
            panic!("Name of identifier is not {}: got {}", name, token.literal_of());
        }
        _ => panic!("This expression is not Identifier: got {:?}", expression),
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
