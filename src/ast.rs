use crate::token::TokenKind;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: Vec::new() }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Ret(RetStatement),
    Exp(ExpStatement),
    Blk(BlkStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub ident:   Identifier,
    pub rhs_exp: Expression, 
}

impl LetStatement {
    pub fn new(ident: Identifier, rhs_exp: Expression) -> LetStatement {
        LetStatement { ident, rhs_exp }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RetStatement {
    pub exp: Expression,
}

impl RetStatement {
    pub fn new(exp: Expression) -> RetStatement {
        RetStatement { exp }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExpStatement {
    pub exp: Expression,
}

impl ExpStatement {
    pub fn new(exp: Expression) -> ExpStatement {
        ExpStatement { exp }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlkStatement {
    statements: Vec<Statement>,
}

impl BlkStatement {
    pub fn new(statements: Vec<Statement>) -> BlkStatement {
        BlkStatement { statements }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(Identifier),
    Int(Integer),
    Bool(Boolean),

    // Without boxing two expression, compiler can't detect the size of Expression.
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),

    // Complex (not C) expression
    If(IfExpression),

    Dummy,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Identifier {
        Identifier { name }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Integer {
        Integer { value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Boolean {
        Boolean { value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrefixExpression {
    pub operator: TokenKind,
    pub rhs_exp: Expression,
}

impl PrefixExpression {
    pub fn new(operator: TokenKind, rhs_exp: Expression) -> PrefixExpression {
        PrefixExpression { operator, rhs_exp }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InfixExpression {
    pub operator: TokenKind,
    pub lhs_exp: Expression,
    pub rhs_exp: Expression,
}

impl InfixExpression {
    pub fn new(operator: TokenKind, lhs_exp: Expression, rhs_exp: Expression) -> InfixExpression {
        InfixExpression { operator, lhs_exp, rhs_exp }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlkStatement,
    pub alternative: Option<BlkStatement>,
}

impl IfExpression {
    pub fn new(cond: Expression, cons: BlkStatement, alt: Option<BlkStatement>) -> IfExpression {
        IfExpression {
            condition: Box::new(cond),
            consequence: cons,
            alternative: alt
        }
    }
}
