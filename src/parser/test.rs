#![cfg(test)]

use crate::{
    lexer::Lexer, 
    parser::Parser, 
    ast::{
        Program, Statement, Expression, Identifier, Integer, PrefixExpression,
        InfixExpression, Boolean, IfExpression, BlkStatement, ExpStatement
    },
    token::TokenKind
};

#[test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

    let program = parse_input(input);
    test_program_length(&program, 3);

    let tests = vec!["x", "y", "foobar"];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_let_statement(stmt, &test.to_string());
    }
}

#[test]
fn test_ret_statements() {
    let input = "
        return 5;
        return 10;
        return add(1, 2);";

    let program = parse_input(input);
    test_program_length(&program, 3);

    for stmt in program.statements.iter() {
        test_ret_statement(stmt);
    }
}

#[test]
fn test_identifier_expression() {
    let input = "
        foo;
        bar;
        foobar;";

    let program = parse_input(input);
    test_program_length(&program, 3);

    let tests = vec!["foo", "bar", "foobar"];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(
            stmt,
            &Expression::Ident(Identifier {
                name: test.to_string()
            }));
    }
}

#[test]
fn test_integer_expression() {
    let input = "
        5;
        10;
        858585;";

    let program = parse_input(input);
    test_program_length(&program, 3);

    let tests = vec![5, 10, 858585];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(
            stmt,
            &Expression::Int(
                Integer {
                    value: *test
                }
            )
        );
    }
}

#[test]
fn test_prefix_expression() {
    let input = "
        !5;
        -15;";

    let program = parse_input(input);
    test_program_length(&program, 2);

    let tests = vec![(TokenKind::Bang, 5), (TokenKind::Minus, 15)];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(
            stmt,
            &Expression::Prefix(
                Box::new(
                    PrefixExpression {
                        operator: test.0,
                        rhs_exp: Expression::Int(
                            Integer::new(test.1)
                        )
                    }
                )
            )
        );
    }
}

#[test]
fn test_infix_expression() {
    let input = "
        5 + 5;
        5 - 5;
        5 * 5;
        5 / 5;
        5 == 5;
        5 != 5;
        5 < 5;
        5 > 5;";

    let program = parse_input(input);
    test_program_length(&program, 8);

    let tests = vec![
        (TokenKind::Plus,     5, 5),
        (TokenKind::Minus,    5, 5),
        (TokenKind::Asterisk, 5, 5),
        (TokenKind::Slash,    5, 5),
        (TokenKind::Eq,       5, 5),
        (TokenKind::NotEq,    5, 5),
        (TokenKind::LT,       5, 5),
        (TokenKind::GT,       5, 5),
    ];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(
            stmt,
            &Expression::Infix(
                Box::new(
                    InfixExpression::new(
                        test.0,
                        Expression::Int(
                            Integer::new(test.1)
                        ),
                        Expression::Int(
                            Integer::new(test.2)
                        )
                    )
                )
            )
        );
    }
}

#[test]
fn test_boolean_expression() {
    let input = "
        true;
        false;";

    let program = parse_input(input);
    test_program_length(&program, 2);

    let tests = vec![ true, false ];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(
            stmt,
            &Expression::Bool(
                Boolean::new(
                    *test
                )
            )
        );
    }
}

#[test]
fn test_grouped_expression() {
    let input = "(1 + 2) * 3;";

    let program = parse_input(input);
    test_program_length(&program, 1);

    test_exp_statement(
        &program.statements[0],
        &Expression::Infix(
            Box::new(
                InfixExpression::new(
                    TokenKind::Asterisk,
                    Expression::Infix(
                        Box::new(
                            InfixExpression::new(
                                TokenKind::Plus,
                                Expression::Int(
                                    Integer::new(1)
                                ),
                                Expression::Int(
                                    Integer::new(2)
                                )
                            )
                        )
                    ),
                    Expression::Int(
                        Integer::new(3)
                    )
                )
            )
        )
    );
}

#[test]
fn test_if_expression() {
    let input = "if (a < 10) { x }";

    let program = parse_input(input);
    test_program_length(&program, 1);

    let stmt = &program.statements[0];
    test_exp_statement(
        stmt,
        &Expression::If(
            IfExpression::new(
                Expression::Infix(
                    Box::new(
                        InfixExpression {
                            operator: TokenKind::LT,
                            lhs_exp: Expression::Ident(
                                Identifier::new("a".to_string())
                            ),
                            rhs_exp: Expression::Int(
                                Integer::new(10)
                            )
                        }
                    )
                ),
                BlkStatement::new(
                    vec![
                        Statement::Exp(
                            ExpStatement::new(
                                Expression::Ident(
                                    Identifier::new("x".to_string())
                                )
                            )
                        )
                    ]
                ),
                None
            )
        )
    );
}

#[test]
fn test_if_else_expression() {
    let input = "if (a < 10) { x } else { y }";

    let program = parse_input(input);
    test_program_length(&program, 1);

    let stmt = &program.statements[0];
    test_exp_statement(
        stmt,
        &Expression::If(
            IfExpression::new(
                Expression::Infix(
                    Box::new(
                        InfixExpression {
                            operator: TokenKind::LT,
                            lhs_exp: Expression::Ident(
                                Identifier::new("a".to_string())
                            ),
                            rhs_exp: Expression::Int(
                                Integer::new(10)
                            )
                        }
                    )
                ),
                BlkStatement::new(
                    vec![
                        Statement::Exp(
                            ExpStatement::new(
                                Expression::Ident(
                                    Identifier::new("x".to_string())
                                )
                            )
                        )
                    ]
                ),
                Some(
                    BlkStatement::new(
                        vec![
                            Statement::Exp(
                                ExpStatement::new(
                                    Expression::Ident(
                                        Identifier::new("y".to_string())
                                    )
                                )
                            )
                        ]
                    )
                ),
            )
        )
    );
}

fn test_let_statement(stmt: &Statement, name: &String) {
    // Is it let statement?
    let stmt = match stmt {
        Statement::Let(stmt) => stmt,
        _ => panic!("This statement is not LetStatement: got {:?}", stmt),
    };

    // Is the identifier is same?
    if stmt.ident.name != *name {
        panic!("The identifier has different name: expect {}, got {}", name, stmt.ident.name);
    }

    // TODO
    eprintln!("TODO: We need to check expression in let statement.");
}

fn test_ret_statement(stmt: &Statement) {
    // Is it return statement?
    match stmt {
        Statement::Ret(_) => (),
        _ => panic!("This statement is not RetStatement: got {:?}", stmt),
    };

    // TODO
    eprintln!("TODO: We need to check expression in return statement.");
}

fn test_exp_statement(stmt: &Statement, exp: &Expression) {
    match stmt {
        Statement::Exp(stmt) => {
            if stmt.exp != *exp {
                panic!("The statement is different: expect {:?}, got {:?}", exp, stmt.exp);
            }
        }
        _ => panic!("This statement is not ExpStatement: got {:?}", stmt),
    }
}

fn test_program_length(prg: &Program, len: usize) {
    if prg.statements.len() != len {
        panic!(
            "program.statements dosen't contain {} statements: got {}",
            len, prg.statements.len()
        );
    }
}

fn parse_input(input: &str) -> Program {
    let lexer   = Lexer::new(input);
    let parser  = Parser::new(lexer.tokenize());
    parser.parse().unwrap_or_else(|err| panic!("{}", err))
}
