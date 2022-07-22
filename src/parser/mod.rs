mod test;
mod error;
mod order;

use crate::{
    token::{
        Token, TokenKind
    },
    ast::{
        Program, Statement, LetStatement, Identifier, Expression, RetStatement, ExpStatement,
        Integer, PrefixExpression, InfixExpression, Boolean, IfExpression, BlkStatement
    },
};
use self::{error::ParseError, order::PriorityOrder};
use std::{cell::Cell, error::Error};

pub struct Parser<'a> {
    token: Vec<Token<'a>>,
    curr:  Cell<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(token: Vec<Token>) -> Parser {
        Parser {
            token,
            curr: Cell::new(0),
        }
    }

    pub fn parse(&self) -> Result<Program, Box<dyn Error>> {
        let mut ret = Program::new();
        while !self.curr_token_is(TokenKind::Eof)? {
            ret.statements.push(self.statement()?);
            self.next_token();
        }
        Ok(ret)
    }
}

impl<'a> Parser<'a> {
    fn statement(&self) -> Result<Statement, Box<dyn Error>> {
        match self.curr_token()?.kind {
            TokenKind::Let    => Ok(Statement::Let(self.let_statement()?)),
            TokenKind::Return => Ok(Statement::Ret(self.ret_statement()?)),
            _                 => self.exp_statement(),
        }
    }

    fn let_statement(&self) -> Result<LetStatement, Box<dyn Error>> {
        if !self.expect_peek(TokenKind::Ident)? {
            let err = ParseError::InvalidTokenFound(vec![TokenKind::Ident], self.curr_token()?.kind);
            Err(err)?;
        }

        let name = self.curr_token()?.literal.to_string();

        if !self.expect_peek(TokenKind::Assign)? {
            let err = ParseError::InvalidTokenFound(vec![TokenKind::Ident], self.curr_token()?.kind);
            Err(err)?;
        }

        // TODO: We need to parse expression in this place
        while self.curr_token()?.kind != TokenKind::Semicolon {
            self.next_token(); // Skip expression
        }

        let ret = LetStatement::new(Identifier { name }, Expression::Dummy);
        Ok(ret)
    }

    fn ret_statement(&self) -> Result<RetStatement, Box<dyn Error>> {
        // TODO: We need to parse expression in this place
        while self.curr_token()?.kind != TokenKind::Semicolon {
            self.next_token(); // Skip expression
        }

        let ret = RetStatement::new(Expression::Dummy);
        Ok(ret)
    }

    fn exp_statement(&self) -> Result<Statement, Box<dyn Error>> {
        let exp = self.expression(PriorityOrder::Lowest)?;

        self.expect_peek(TokenKind::Semicolon)?;
        Ok(Statement::Exp(ExpStatement::new(exp)))

    }

    fn blk_statement(&self) -> Result<BlkStatement, Box<dyn Error>> {
        self.next_token();

        let mut ret = Vec::new();
        while !self.curr_token_is(TokenKind::RCurlyBracket)? {
            let stmt = self.statement()?;
            ret.push(stmt);
            self.next_token();
        }

        Ok(BlkStatement::new(ret))
    }

    fn expression(&self, order: PriorityOrder) -> Result<Expression, Box<dyn Error>> {
        let mut left = match self.curr_token()?.kind {
            TokenKind::Ident => self.identifier()?,
            TokenKind::Int   => self.integer()?,
            TokenKind::LParenthesis => self.group()?,
            TokenKind::True | TokenKind::False => self.boolean()?,
            TokenKind::Bang | TokenKind::Minus => self.prefix()?,
            TokenKind::If => self.if_expression()?,
            kind => Err(ParseError::NoSuchExpressionStartWith(kind))?,
        };

        while !self.peek_token_is(TokenKind::Semicolon)? && order < self.peek_order()? {
            match self.peek_token()?.kind {
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Asterisk
                | TokenKind::Slash
                | TokenKind::Eq
                | TokenKind::NotEq
                | TokenKind::LT
                | TokenKind::GT => {
                    self.next_token();
                    left = self.infix(left)?;
                }
                _ => {
                    return Ok(left);
                }
            }
        }

        Ok(left)
    }

    fn identifier(&self) -> Result<Expression, Box<dyn Error>> {
        let token = self.curr_token()?;
        let name  = match token.kind {
            TokenKind::Ident => token.literal.to_string(),
            kind => Err(ParseError::InvalidTokenFound(vec![TokenKind::Ident], kind))?,
        };

        Ok(Expression::Ident(Identifier::new(name)))
    }

    fn integer(&self) -> Result<Expression, Box<dyn Error>> {
        let token = self.curr_token()?;
        let value = match token.kind {
            TokenKind::Int => token.literal.parse()?,
            _ => Err(ParseError::InvalidTokenFound(vec![TokenKind::Int], token.kind))?,
        };

        Ok(Expression::Int(Integer::new(value)))
    }

    fn boolean(&self) -> Result<Expression, Box<dyn Error>> {
        match self.curr_token()?.kind {
            TokenKind::True  => Ok(Expression::Bool(Boolean::new(true))),
            TokenKind::False => Ok(Expression::Bool(Boolean::new(false))),
            kind => {
                Err(
                    ParseError::InvalidTokenFound(
                        vec![TokenKind::True, TokenKind::False], kind
                    )
                )?
            }
        }
    }

    // 'op' expression
    fn prefix(&self) -> Result<Expression, Box<dyn Error>> {
        let operator = self.curr_token()?.kind;
        self.next_token();
        let rhs_exp  = self.expression(PriorityOrder::Prefix)?;

        let ret = Expression::Prefix(
            Box::new(
                PrefixExpression::new(operator, rhs_exp)
            )
        );
        Ok(ret)
    }

    // expression 'op' expression
    fn infix(&self, left: Expression) -> Result<Expression, Box<dyn Error>> {
        let operator = self.curr_token()?.kind;
        let order = self.curr_order()?;
        self.next_token();
        let right = self.expression(order)?;

        Ok(Expression::Infix(Box::new(InfixExpression::new(operator, left, right))))
    }

    // '(' expression ')''
    fn group(&self) -> Result<Expression, Box<dyn Error>> {
        self.next_token();

        let exp = self.expression(PriorityOrder::Lowest)?;

        let kind = self.peek_token()?.kind;
        if self.expect_peek(TokenKind::RParenthesis)? {
            Ok(exp)
        } else {
            Err(ParseError::InvalidTokenFound(vec![TokenKind::RParenthesis], kind))?
        }
    }

    fn if_expression(&self) -> Result<Expression, Box<dyn Error>> {
        let kind = self.peek_token()?.kind;
        if !self.expect_peek(TokenKind::LParenthesis)? {
            Err(ParseError::InvalidTokenFound(vec![TokenKind::LParenthesis], kind))?
        }

        self.next_token();
        let cond = self.expression(PriorityOrder::Lowest)?;

        let kind = self.peek_token()?.kind;
        if !self.expect_peek(TokenKind::RParenthesis)? {
            Err(ParseError::InvalidTokenFound(vec![TokenKind::RParenthesis], kind))?
        }

        let kind = self.peek_token()?.kind;
        if !self.expect_peek(TokenKind::LCurlyBracket)? {
            Err(ParseError::InvalidTokenFound(vec![TokenKind::LCurlyBracket], kind))?
        }

        let cons = self.blk_statement()?;

        let alt = if self.expect_peek(TokenKind::Else)? {
            let kind = self.peek_token()?.kind;
            if !self.expect_peek(TokenKind::LCurlyBracket)? {
                Err(ParseError::InvalidTokenFound(vec![TokenKind::LCurlyBracket], kind))?
            }
            Some(self.blk_statement()?)
        } else {
            None
        };

        Ok(Expression::If(IfExpression::new(cond, cons, alt)))
    }
}

// Helper functions
impl<'a> Parser<'a> {
    fn curr_token(&self) -> Result<Token, ParseError> {
        match self.token.get(self.curr.get() + 0) {
            Some(ret) => Ok(*ret),
            None => Err(ParseError::FailedToReadToken),
        }
    }

    fn peek_token(&self) -> Result<Token, ParseError> {
        match self.token.get(self.curr.get() + 1) {
            Some(ret) => Ok(*ret),
            None => Err(ParseError::FailedToReadToken),
        }
    }

    fn curr_token_is(&self, kind: TokenKind) -> Result<bool, ParseError> {
        match self.curr_token()?.kind {
            val if val == kind => Ok(true),
            _ => Ok(false),
        }
    }

    fn peek_token_is(&self, kind: TokenKind) -> Result<bool, ParseError> {
        match self.peek_token()?.kind {
            val if val == kind => Ok(true),
            _ => Ok(false),
        }
    }

    fn expect_peek(&self, kind: TokenKind) -> Result<bool, ParseError> {
        if self.peek_token_is(kind)? {
            self.next_token();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn next_token(&self) {
        self.curr.set(self.curr.get().wrapping_add(1));
    }

    fn token_to_order(kind: TokenKind) -> PriorityOrder {
        match kind {
            TokenKind::Eq       | TokenKind::NotEq => PriorityOrder::Equals,
            TokenKind::LT       | TokenKind::GT    => PriorityOrder::LessGreater,
            TokenKind::Plus     | TokenKind::Minus => PriorityOrder::Sum,
            TokenKind::Asterisk | TokenKind::Slash => PriorityOrder::Product,
            _ => PriorityOrder::Lowest,
        }
    }

    fn curr_order(&self) -> Result<PriorityOrder, Box<dyn Error>> {
        Ok(Parser::token_to_order(self.curr_token()?.kind))
    }

    fn peek_order(&self) -> Result<PriorityOrder, Box<dyn Error>> {
        Ok(Parser::token_to_order(self.peek_token()?.kind))
    }
}
