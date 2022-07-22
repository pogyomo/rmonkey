mod token;
mod lexer;
mod ast;
mod parser;

use crate::ast::*;

fn main() {
    let lst = Statement::Let(LetStatement {
        ident: Identifier {
            name: "name".to_string(),
        },
        rhs_exp: PrefixExpression {
            rhs_exp: Expression::Prefix(PrefixExpression {
                rhs_exp: Expression::Integer(Integer {
                    value: 0
                })
            })
        }
    });
}
