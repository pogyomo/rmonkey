use crate::token::{TokenKind, Token};

pub trait Node {
    fn string(&self) -> String;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn string(&self) -> String {
        if self.statements.len() > 1 {
            self.statements[0].string()
        } else {
            String::from("")
        }
    }
}

impl Program {
    pub fn new() -> Program {
        Program { statements: Vec::new() }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    Let(LetStatement),
    Ret(RetStatement),
    Exp(ExpStatement),
    Blk(BlkStatement),
}

impl Node for Statement {
    fn string(&self) -> String {
        match self {
            Statement::Let(stmt) => stmt.string(),
            Statement::Ret(stmt) => stmt.string(),
            Statement::Exp(stmt) => stmt.string(),
            Statement::Blk(stmt) => stmt.string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LetStatement {
    pub ident:   Identifier,
    pub rhs_exp: Expression, 
}

impl Node for LetStatement {
    fn string(&self) -> String {
        format!("let {} = {};", self.ident.string(), self.rhs_exp.string())
    }
}

impl LetStatement {
    pub fn new(ident: Identifier, rhs_exp: Expression) -> LetStatement {
        LetStatement { ident, rhs_exp }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RetStatement {
    pub exp: Expression,
}

impl Node for RetStatement {
    fn string(&self) -> String {
        format!("return {};", self.exp.string())
    }
}

impl RetStatement {
    pub fn new(exp: Expression) -> RetStatement {
        RetStatement { exp }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpStatement {
    pub exp: Expression,
}

impl Node for ExpStatement {
    fn string(&self) -> String {
        self.exp.string()
    }
}

impl ExpStatement {
    pub fn new(exp: Expression) -> ExpStatement {
        ExpStatement { exp }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlkStatement {
    pub statements: Vec<Statement>,
}

impl Node for BlkStatement {
    fn string(&self) -> String {
        let mut ret = String::new();
        for stmt in self.statements.iter() {
            ret.push_str(stmt.string().as_str());
            ret.push(' ');
        }
        ret.pop(); // trim the last space
        ret
    }
}

impl BlkStatement {
    pub fn new(statements: Vec<Statement>) -> BlkStatement {
        BlkStatement { statements }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Ident(Identifier),
    Int(Integer),
    Bool(Boolean),
    Str(StringLiteral),

    // Without boxing two expression, compiler can't detect the size of Expression.
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
    Postfix(Box<PostfixExpression>),

    // Complex (not C) expression
    If(IfExpression),
    Func(FunctionExpression),
    Call(CallExpression),
}

impl Node for Expression {
    fn string(&self) -> String {
        match self {
            Expression::Ident(ident)   => ident.string(),
            Expression::Int(integer)   => integer.string(),
            Expression::Bool(boolean)  => boolean.string(),
            Expression::Str(string)    => string.string(),
            Expression::Prefix(prefix) => prefix.string(),
            Expression::Infix(infix)   => infix.string(),
            Expression::If(if_exp)     => if_exp.string(),
            Expression::Func(func)     => func.string(),
            Expression::Call(call)     => call.string(),
            Expression::Postfix(post)  => post.string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Node for Identifier {
    fn string(&self) -> String {
        self.name.clone()
    }
}

impl Identifier {
    pub fn new(name: String) -> Identifier {
        Identifier { name }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Node for Integer {
    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Integer {
    pub fn new(value: i64) -> Integer {
        Integer { value }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Node for Boolean {
    fn string(&self) -> String {
        match self.value {
            true  => "true".to_string(),
            false => "false".to_string(),
        }
    }
}

impl Boolean {
    pub fn new(value: bool) -> Boolean {
        Boolean { value }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StringLiteral {
    pub str: String,
}

impl Node for StringLiteral {
    fn string(&self) -> String {
        format!("\"{}\"", self.str)
    }
}

impl StringLiteral {
    pub fn new(str: String) -> StringLiteral {
        StringLiteral { str }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrefixExpression {
    pub operator: TokenKind,
    pub rhs_exp: Expression,
}

impl Node for PrefixExpression {
    fn string(&self) -> String {
        let token = Token::new(self.operator, "");
        format!("({}{})", token.literal(), self.rhs_exp.string())
    }
}

impl PrefixExpression {
    pub fn new(operator: TokenKind, rhs_exp: Expression) -> PrefixExpression {
        PrefixExpression { operator, rhs_exp }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Node for InfixExpression {
    fn string(&self) -> String {
        let token = Token::new(self.operator, "");
        format!("({} {} {})", self.lhs_exp.string(), token.literal(), self.rhs_exp.string())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PostfixExpression {
    pub operator: TokenKind,
    pub lhs_exp: Expression,
}

impl Node for PostfixExpression {
    fn string(&self) -> String {
        let token = Token::new(self.operator, "");
        format!("({}{})", token.literal(), self.lhs_exp.string())
    }
}

impl PostfixExpression {
    pub fn new(operator: TokenKind, lhs_exp: Expression) -> PostfixExpression {
        PostfixExpression { operator, lhs_exp }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfExpression {
    pub condition:   Box<Expression>,
    pub consequence: BlkStatement,
    pub alternative: Option<BlkStatement>,
}

impl Node for IfExpression {
    fn string(&self) -> String {
        match self.alternative {
            Some(ref alt) => {
                format!(
                    "if ( {} ) {{ {} }} else {{ {} }}",
                    self.condition.string(),
                    self.consequence.string(),
                    alt.string(),
                )
            }
            None => {
                format!(
                    "if ( {} ) {{ {} }}",
                    self.condition.string(),
                    self.consequence.string(),
                )
            }
        }
    }
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionExpression {
    pub params: Vec<Identifier>,
    pub body:   BlkStatement,
}

impl Node for FunctionExpression {
    fn string(&self) -> String {
        let mut ret = String::new();
        ret.push_str("fn(");

        for param in self.params.iter() {
            ret.push_str(param.string().as_str());
            ret.push_str(", ");
        }

        // Remove ", "
        ret.pop();
        ret.pop();

        ret.push_str(") { ");
        ret.push_str(self.body.string().as_str());
        ret.push_str(" }");
        ret
    }
}

impl FunctionExpression {
    pub fn new(params: Vec<Identifier>, body: BlkStatement) -> FunctionExpression {
        FunctionExpression { params, body }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallExpression {
    pub ident: Box<Expression>, // Identifier or FunctionExpression
    pub args:  Vec<Expression>,
}

impl Node for CallExpression {
    fn string(&self) -> String {
        let mut ret = String::new();
        ret.push_str(format!("{}(", self.ident.string()).as_str());

        for arg in self.args.iter() {
            ret.push_str(arg.string().as_str());
            ret.push_str(", ");
        }

        // Remove ", "
        ret.pop();
        ret.pop();

        ret.push(')');
        ret
    }
}

impl CallExpression {
    pub fn new(ident: Expression, args: Vec<Expression>) -> CallExpression {
        CallExpression { ident: Box::new(ident), args }
    }
}
