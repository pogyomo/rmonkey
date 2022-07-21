mod test;

use std::{cell::{Cell, RefCell}, rc::Rc};
use crate::{ast::{Program, Statement, Expression}, token::Token};

#[derive(PartialEq, PartialOrd)]
pub enum PriorityOrder {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct Parser<'a> {
    token: Vec<Token<'a>>, 
    curr:  Cell<usize>,
    error: RefCell<Vec<String>>,
}

impl<'a> Parser<'a> {
    pub fn new(token: Vec<Token>) -> Parser {
        Parser {
            token,
            curr: Cell::new(0),
            error: RefCell::new(Vec::new()),
        }
    }

    pub fn parse(&self) -> Program {
        let mut ret = Program::new();
        while self.token.len() > self.curr.get() {
            if let Some(statement) = self.parse_statement() {
                ret.statements.push(statement);
            }
            self.next_token();
        }
        ret
    }
}

impl<'a> Parser<'a> {
    fn parse_statement(&self) -> Option<Statement> {
        match self.curr_token()? {
            Token::Let    => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _             => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&self) -> Option<Statement> {
        let name = match self.peek_token()? {
            Token::Ident(name) => {
                self.next_token();
                name
            }
            _ => {
                self.peek_error(Token::Ident(""));
                return None;
            }
        };

        match self.peek_token()? {
            Token::Assign => self.next_token(),
            _ => {
                self.peek_error(Token::Assign);
                return None;
            }
        }

        // TODO: We need to read expression
        // Skip token until current token become Semicolon
        while let Some(value) = self.curr_token() {
            if value == Token::Semicolon {
                break;
            } else {
                self.next_token();
            }
        }

        Some(Statement::LetStatement {
            token: Token::Let,
            ident: Expression::Identifier { token: Token::Ident(name) },
            // Currently, I don't read expression.
            expression: Expression::Dummy,
        })
    }

    fn parse_return_statement(&self) -> Option<Statement> {
        // TODO: We need to read expression
        // Skip token until current token become Semicolon
        while let Some(value) = self.curr_token() {
            if value == Token::Semicolon {
                break;
            } else {
                self.next_token();
            }
        }

        Some(Statement::ReturnStatement {
            token: Token::Return,
            ret_value: Expression::Dummy,
        })
    }

    fn parse_expression_statement(&self) -> Option<Statement> {
        let token = self.curr_token()?;
        let expression = self.parse_expression(PriorityOrder::Lowest)?;

        if self.peek_token()? == Token::Semicolon {
            self.next_token();
        }

        Some(Statement::ExpressionStatement { token, expression })
    }

    fn parse_expression(&self, precedence: PriorityOrder) -> Option<Expression> {
        let curr_token = self.curr_token()?;
        let mut left = match curr_token {
            Token::Ident(_)            => self.parse_identifier()?,
            Token::Int(_)              => self.parse_integer_literal()?,
            Token::True | Token::False => self.parse_boolean()?,
            Token::Bang | Token::Minus => self.parse_prefix_expression()?,
            Token::LParenthesis        => self.parse_grouped_expression()?,
            _ => {
                self.no_prefix_error(curr_token);
                return None;
            }
        };

        while
            self.peek_token()? != Token::Semicolon &&
            precedence < self.peek_priority_order()?
        {
            left = match self.peek_token()? {
                Token::Plus
                | Token::Minus 
                | Token::Slash 
                | Token::Asterisk
                | Token::Eq 
                | Token::NotEq 
                | Token::LT 
                | Token::GT => {
                    self.next_token();
                    self.parse_infix_expression(left)?
                }
                _ => {
                    return Some(left);
                }
            };
        }

        Some(left)
    }

    fn parse_identifier(&self) -> Option<Expression> {
        let ident = Expression::Identifier {
            token: self.curr_token()?
        };
        Some(ident)
    }

    fn parse_integer_literal(&self) -> Option<Expression> {
        let token = self.curr_token()?;
        let value = match token.literal_of().parse::<i64>() {
            Ok(value) => value,
            Err(e)    => {
                self.error.borrow_mut().push(e.to_string());
                return None;
            }
        };
        Some(Expression::Integer { token, value })
    }

    fn parse_prefix_expression(&self) -> Option<Expression> {
        let token = self.curr_token()?;
        self.next_token();
        let right = self.parse_expression(PriorityOrder::Prefix)?;

        Some(Expression::PrefixExpression { token, right: Rc::new(right) })
    }

    fn parse_infix_expression(&self, left: Expression<'a>) -> Option<Expression> {
        let token = self.curr_token()?;
        let order = self.curr_priority_order()?;
        self.next_token();
        let right = self.parse_expression(order)?;

        let ret = Expression::InfixExpression {
            token,
            left:  Rc::new(left),
            right: Rc::new(right)
        };

        Some(ret)
    }

    fn parse_boolean(&self) -> Option<Expression> {
        Some(Expression::Boolean {
            token: self.curr_token()?,
            value: self.curr_token()? == Token::True
        })
    }

    fn parse_grouped_expression(&self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression(PriorityOrder::Lowest)?;
        match self.peek_token()? {
            Token::RParenthesis => self.next_token(),
            _ => return None,
        }

        Some(expression)
    }
}

impl<'a> Parser<'a> {
    fn curr_token(&self) -> Option<Token> {
        match self.token.get(self.curr.get() + 0) {
            Some(value) => Some(*value),
            None        => None,
        }
    }

    fn peek_token(&self) -> Option<Token> {
        match self.token.get(self.curr.get() + 1) {
            Some(value) => Some(*value),
            None        => None,
        }
    }

    fn next_token(&self) {
        self.curr.set(self.curr.get() + 1);
    }
}

impl<'a> Parser<'a> {
    fn token_to_priority_order(token: Token) -> PriorityOrder {
        match token {
            Token::Eq    | Token::NotEq    => PriorityOrder::Equals,
            Token::LT    | Token::GT       => PriorityOrder::LessGreater,
            Token::Plus  | Token::Minus    => PriorityOrder::Sum,
            Token::Slash | Token::Asterisk => PriorityOrder::Product,
            _                              => PriorityOrder::Lowest,
        }
    }

    fn curr_priority_order(&self) -> Option<PriorityOrder> {
        match self.token.get(self.curr.get() + 0) {
            Some(token) => Some(Parser::token_to_priority_order(*token)),
            None        => Some(PriorityOrder::Lowest),
        }
    }

    fn peek_priority_order(&self) -> Option<PriorityOrder> {
        match self.token.get(self.curr.get() + 1) {
            Some(token) => Some(Parser::token_to_priority_order(*token)),
            None        => Some(PriorityOrder::Lowest),
        }
    }
}

impl<'a> Parser<'a> {
    fn peek_error(&self, token: Token) {
        if let Some(value) = self.token.get(self.curr.get() + 1) {
            let msg = format!(
                "expected next token to be {:?}, but got {:?}",
                token, value);
            self.error.borrow_mut().push(msg);
        }
    }

    fn no_prefix_error(&self, token: Token) {
        let msg = format!("No prefix parse function for {} not found.", token.literal_of());
        self.error.borrow_mut().push(msg);
    }
}
