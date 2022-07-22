pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: Vec::new() }
    }
}

pub enum Statement {
    Let(LetStatement),
    Ret(RetStatement),
    Exp(ExpStatement),
}

pub struct LetStatement {
    ident:   Identifier,
    rhs_exp: Expression, 
}

pub struct RetStatement {
    exp: Expression,
}

pub struct ExpStatement {
    exp: Expression,
}

pub enum Expression {
    Identifier(Identifier),
    Integer(Integer),

    // Without boxing two expression, compiler can't detect the size of Expression.
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
}

pub struct Identifier {
    name: String,
}

pub struct Integer {
    value: i64,
}

pub struct PrefixExpression {
    rhs_exp: Expression,
}

pub struct InfixExpression {
    lhs_exp: Expression,
    rhs_exp: Expression,
}
