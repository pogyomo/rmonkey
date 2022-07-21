use crate::token::Token;
use std::rc::Rc;

pub trait Node {
    fn literal(&self) -> &str;
    fn string(&self) -> String;
}

pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl<'a> Node for Program<'a> {
    fn literal(&self) -> &str {
        if let Some(statement) = self.statements.get(0) {
            statement.literal()
        } else {
            ""
        }
    }

    fn string(&self) -> String {
        let mut ret = String::new();
        for statement in self.statements.iter() {
            ret.push_str(statement.string().as_str());
        }
        ret
    }
}

impl<'a> Program<'a> {
    pub fn new() -> Program<'a> {
        Program { statements: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    LetStatement{
        token: Token<'a>,
        ident: Expression<'a>,
        expression: Expression<'a>,
    },
    ReturnStatement {
        token: Token<'a>,
        ret_value: Expression<'a>,
    },
    ExpressionStatement {
        token: Token<'a>,
        expression: Expression<'a>,
    }
}

impl<'a> Node for Statement<'a> {
    fn literal(&self) -> &str {
        match self {
            Self::LetStatement{ token, .. }        => token.literal_of(),
            Self::ReturnStatement{ token, .. }     => token.literal_of(),
            Self::ExpressionStatement{ token, .. } => token.literal_of(),
        }
    }

    fn string(&self) -> String {
        let mut ret = String::new();
        match self {
            Self::LetStatement { ident, expression, .. } => {
                ret.push_str(self.literal());
                ret.push(' ');
                ret.push_str(ident.string().as_str());
                ret.push_str(" = ");
                ret.push_str(expression.string().as_str());
                ret.push(';');
            }
            Self::ReturnStatement { ret_value, .. } => {
                ret.push_str(self.literal());
                ret.push(' ');
                ret.push_str(ret_value.string().as_str());
                ret.push(';');
            }
            Self::ExpressionStatement { expression, .. } => {
                ret.push_str(expression.string().as_str());
                ret.push(';');
            }
        }
        ret
    }
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Identifier {
        token: Token<'a>,
    },
    Integer {
        token: Token<'a>,
        value: i64,
    },
    PrefixExpression {
        token: Token<'a>,
        right: Rc<Expression<'a>>,
    },
    InfixExpression {
        token: Token<'a>,
        left:  Rc<Expression<'a>>,
        right: Rc<Expression<'a>>,
    },
    Boolean {
        token: Token<'a>,
        value: bool,
    },
    Dummy,
}

impl<'a> Node for Expression<'a> {
    fn literal(&self) -> &str {
        match self {
            Self::Identifier { token, .. }       => token.literal_of(),
            Self::Integer { token, .. }          => token.literal_of(),
            Self::PrefixExpression { token, .. } => token.literal_of(),
            Self::InfixExpression { token, .. }  => token.literal_of(),
            Self::Boolean { token, .. }          => token.literal_of(),
            _ => "",
        }
    }

    fn string(&self) -> String {
        match self {
            Self::Identifier { .. } => self.literal().to_string(),
            Self::Integer { .. }    => self.literal().to_string(),
            Self::Boolean { .. }    => self.literal().to_string(),
            Self::PrefixExpression { right, .. } => {
                let mut ret = String::new();
                ret.push('(');
                ret.push_str(self.literal());
                ret.push(' ');
                ret.push_str(right.string().as_str());
                ret.push(')');
                ret
            }
            Self::InfixExpression { left, right, .. } => {
                let mut ret = String::new();
                ret.push('(');
                ret.push_str(left.string().as_str());
                ret.push(' ');
                ret.push_str(self.literal());
                ret.push(' ');
                ret.push_str(right.string().as_str());
                ret.push(')');
                ret
            }
            Self::Dummy => "Dummy Expression (Will be removed)".to_string(),
        }
    }
}
