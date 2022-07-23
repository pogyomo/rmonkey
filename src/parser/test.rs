#![cfg(test)]

use crate::{
    lexer::Lexer, 
    parser::Parser, 
    ast::{
        Program, Statement, Expression, Identifier, Integer, PrefixExpression,
        InfixExpression, Boolean, IfExpression, BlkStatement, ExpStatement,
        FunctionExpression, Node,
    },
    token::TokenKind
};

#[test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let z = 1 * (2 + 3);";

    let program = parse_input(input);
    test_program_length(&program, 3);

    let tests = vec!["let x = 5;", "let y = 10;", "let z = (1 * (2 + 3));"];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(stmt, test.to_string());
    }
}

#[test]
fn test_ret_statements() {
    let input = "
        return 5;
        return 10;
        return 1 * (2 + 3);";

    let program = parse_input(input);
    test_program_length(&program, 3);

    let tests = vec!["return 5;", "return 10;", "return (1 * (2 + 3));"];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(stmt, test.to_string());
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
        test_exp_statement(stmt, test.to_string());
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

    let tests = vec!["5", "10", "858585"];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(stmt, test.to_string());
    }
}

#[test]
fn test_prefix_expression() {
    let input = "
        !5;
        -15;";

    let program = parse_input(input);
    test_program_length(&program, 2);

    let tests = vec!["(!5)", "(-15)"];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(stmt, test.to_string());
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
        "(5 + 5)",
        "(5 - 5)",
        "(5 * 5)",
        "(5 / 5)",
        "(5 == 5)",
        "(5 != 5)",
        "(5 < 5)",
        "(5 > 5)",
    ];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(stmt, test.to_string());
    }
}

#[test]
fn test_boolean_expression() {
    let input = "
        true;
        false;";

    let program = parse_input(input);
    test_program_length(&program, 2);

    let tests = vec!["true", "false"];
    for (stmt, test) in program.statements.iter().zip(&tests) {
        test_exp_statement(stmt, test.to_string());
    }
}

#[test]
fn test_grouped_expression() {
    let input = "(1 + 2) * 3;";

    let program = parse_input(input);
    test_program_length(&program, 1);

    test_exp_statement(&program.statements[0], "((1 + 2) * 3)".to_string());
}

#[test]
fn test_if_expression() {
    let input = "if (a < 10) { x }";

    let program = parse_input(input);
    test_program_length(&program, 1);

    let stmt = &program.statements[0];
    test_exp_statement(stmt, "if ( (a < 10) ) { x }".to_string());
}

#[test]
fn test_if_else_expression() {
    let input = "if (a < 10) { x } else { y }";

    let program = parse_input(input);
    test_program_length(&program, 1);

    let stmt = &program.statements[0];
    test_exp_statement(stmt, "if ( (a < 10) ) { x } else { y }".to_string());
}

#[test]
fn test_function_statement() {
    let input = "fn(a, b) { a + b }";

    let program = parse_input(input);
    test_program_length(&program, 1);

    let stmt = &program.statements[0];
    test_exp_statement(stmt, "fn(a, b) { (a + b) }".to_string());
}

#[test]
fn test_call_expression() {
    let input = "add(10, a * (10 + b), c == d)";

    let program = parse_input(input);
    test_program_length(&program, 1);

    let stmt = &program.statements[0];
    test_exp_statement(stmt, "add(10, (a * (10 + b)), (c == d))".to_string());
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

fn test_exp_statement(stmt: &Statement, exp: String) {
    if stmt.string() != exp {
        panic!("The statement has different expression: expect {}, got {}", exp, stmt.string());
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
