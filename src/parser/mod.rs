mod test;
mod error;
mod order;

use crate::{
    token::{
        Token, TokenKind
    },
    ast::{
        Program, Statement, LetStatement, Identifier, Expression, RetStatement, ExpStatement,
        Integer, PrefixExpression, InfixExpression, Boolean, IfExpression, BlkStatement,
        FunctionExpression, CallExpression, StringLiteral, PostfixExpression
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

        self.next_token();
        let left_exp = self.expression(PriorityOrder::Lowest)?;
        self.expect_peek(TokenKind::Semicolon)?;

        Ok(LetStatement::new(Identifier::new(name), left_exp))
    }

    fn ret_statement(&self) -> Result<RetStatement, Box<dyn Error>> {
        self.next_token();
        let exp = self.expression(PriorityOrder::Lowest)?;
        self.expect_peek(TokenKind::Semicolon)?;

        Ok(RetStatement::new(exp))
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
            TokenKind::Ident => Expression::Ident(self.identifier()?),
            TokenKind::Int   => Expression::Int(self.integer()?),
            TokenKind::Str   => Expression::Str(self.string_literal()?),
            TokenKind::True
            | TokenKind::False => Expression::Bool(self.boolean()?),

            TokenKind::LParenthesis => self.group()?,

            TokenKind::Bang 
            | TokenKind::Minus
            | TokenKind::Inc
            | TokenKind::Dec => Expression::Prefix(Box::new(self.prefix()?)),

            TokenKind::If       => Expression::If(self.if_expression()?),
            TokenKind::Function => Expression::Func(self.func_expression()?),

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
                    left = Expression::Infix(Box::new(self.infix(left)?));
                }
                TokenKind::LParenthesis => {
                    self.next_token();
                    left = Expression::Call(self.call_expression(left)?);
                }
                TokenKind::Inc | TokenKind::Dec => {
                    self.next_token();
                    left = Expression::Postfix(Box::new(self.postfix(left)?));
                }
                _ => {
                    return Ok(left);
                }
            }
        }

        Ok(left)
    }

    fn identifier(&self) -> Result<Identifier, Box<dyn Error>> {
        let token = self.curr_token()?;
        let name  = match token.kind {
            TokenKind::Ident => token.literal.to_string(),
            kind => Err(ParseError::InvalidTokenFound(vec![TokenKind::Ident], kind))?,
        };

        Ok(Identifier::new(name))
    }

    fn integer(&self) -> Result<Integer, Box<dyn Error>> {
        let token = self.curr_token()?;
        let value = match token.kind {
            TokenKind::Int => token.literal.parse()?,
            _ => Err(ParseError::InvalidTokenFound(vec![TokenKind::Int], token.kind))?,
        };

        Ok(Integer::new(value))
    }

    fn string_literal(&self) -> Result<StringLiteral, Box<dyn Error>> {
        let token = self.curr_token()?;
        let value = match token.kind {
            TokenKind::Str => token.literal(),
            _ => Err(ParseError::InvalidTokenFound(vec![TokenKind::Str], token.kind))?,
        };

        Ok(StringLiteral::new(value.to_string()))
    }

    fn boolean(&self) -> Result<Boolean, Box<dyn Error>> {
        match self.curr_token()?.kind {
            TokenKind::True  => Ok(Boolean::new(true)),
            TokenKind::False => Ok(Boolean::new(false)),
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
    fn prefix(&self) -> Result<PrefixExpression, Box<dyn Error>> {
        let operator = self.curr_token()?.kind;
        self.next_token();
        let rhs_exp  = self.expression(PriorityOrder::Prefix)?;

        Ok(PrefixExpression::new(operator, rhs_exp))
    }

    // expression 'op' expression
    fn infix(&self, left: Expression) -> Result<InfixExpression, Box<dyn Error>> {
        let operator = self.curr_token()?.kind;
        let order = self.curr_order()?;
        self.next_token();
        let right = self.expression(order)?;

        Ok(InfixExpression::new(operator, left, right))
    }

    // expression 'op'
    fn postfix(&self, left: Expression) -> Result<PostfixExpression, Box<dyn Error>> {
        let operator = self.curr_token()?.kind;

        Ok(PostfixExpression::new(operator, left))
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

    fn if_expression(&self) -> Result<IfExpression, Box<dyn Error>> {
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

        Ok(IfExpression::new(cond, cons, alt))
    }

    fn func_expression(&self) -> Result<FunctionExpression, Box<dyn Error>> {
        let kind = self.peek_token()?.kind;
        if !self.expect_peek(TokenKind::LParenthesis)? {
            Err(ParseError::InvalidTokenFound(vec![TokenKind::LParenthesis], kind))?
        }

        let params = self.func_paramators()?;

        let kind = self.peek_token()?.kind;
        if !self.expect_peek(TokenKind::LCurlyBracket)? {
            Err(ParseError::InvalidTokenFound(vec![TokenKind::LCurlyBracket], kind))?
        }

        let body = self.blk_statement()?;

        Ok(FunctionExpression::new(params, body))
    }

    fn func_paramators(&self) -> Result<Vec<Identifier>, Box<dyn Error>> {
        let mut ret = Vec::new();

        if self.peek_token_is(TokenKind::RParenthesis)? {
            self.next_token();
            return Ok(ret);
        } else {
            self.next_token();
        }

        ret.push(self.identifier()?);
        while self.peek_token_is(TokenKind::Comma)? {
            self.next_token();
            self.next_token();
            ret.push(self.identifier()?);
        }

        let kind = self.peek_token()?.kind;
        if !self.expect_peek(TokenKind::RParenthesis)? {
            Err(ParseError::InvalidTokenFound(vec![TokenKind::RParenthesis], kind))?
        } else {
            Ok(ret)
        }
    }

    fn call_expression(&self, left: Expression) -> Result<CallExpression, Box<dyn Error>> {
        let args = self.call_arguments()?;
        Ok(CallExpression::new(left, args))
    }

    fn call_arguments(&self) -> Result<Vec<Expression>, Box<dyn Error>> {
        let mut ret = Vec::new();

        if self.peek_token_is(TokenKind::RParenthesis)? {
            self.next_token();
            return Ok(ret);
        } else {
            self.next_token();
        }

        ret.push(self.expression(PriorityOrder::Lowest)?);
        while self.peek_token_is(TokenKind::Comma)? {
            self.next_token();
            self.next_token();
            ret.push(self.expression(PriorityOrder::Lowest)?);
        }

        let kind = self.peek_token()?.kind;
        if !self.expect_peek(TokenKind::RParenthesis)? {
            Err(ParseError::InvalidTokenFound(vec![TokenKind::RParenthesis], kind))?
        } else {
            Ok(ret)
        }
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
            TokenKind::LParenthesis                => PriorityOrder::Call,
            TokenKind::Inc      | TokenKind::Dec   => PriorityOrder::Postfix,
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
