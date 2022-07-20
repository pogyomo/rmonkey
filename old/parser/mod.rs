mod test;

use std::{cell::{Cell, RefCell}, collections::HashMap};
use crate::{ast::{Program, Statement, Expression}, token::Token};

pub enum PriorityOrder {
    Lowest      = 0,
    Equals      = 1,
    LessGreater = 2,
    Sum         = 3,
    Product     = 4,
    Prefix      = 5,
    Call        = 6,
}

pub struct Parser<'a> {
    token: Vec<Token<'a>>, 
    curr:  Cell<usize>,
    error: RefCell<Vec<String>>,

    // TODO: [ ] This definition (fn() .. ) may be incorrect (Data: 2022/7/21)
    //           How I call the function like this?:
    prefix_parse_fns:
        RefCell<HashMap<Token<'a>, fn(&Parser) -> Option<Expression<'a>>>>,
    infix_parse_fns:
        RefCell<HashMap<Token<'a>, fn(&Parser, Expression<'a>) -> Option<Expression<'a>>>>,
}

impl<'a> Parser<'a> {
    pub fn new(token: Vec<Token>) -> Parser {
        Parser {
            token,
            curr: Cell::new(0),
            error: RefCell::new(Vec::new()),
            prefix_parse_fns: RefCell::new(HashMap::new()),
            infix_parse_fns:  RefCell::new(HashMap::new()),
        }
    }

    pub fn parse(&self) -> Program {
        let mut ret = Program::new();
        while self.token.len() > self.curr.get() {
            if let Some(statement) = self.parse_statement() {
                ret.statements.push(statement);
            }
            self.curr.set(self.curr.get() + 1);
        }
        ret
    }
}

impl<'a> Parser<'a> {
    fn parse_statement(&self) -> Option<Statement> {
        match self.token.get(self.curr.get())? {
            Token::Let    => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _             => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&self) -> Option<Statement> {
        let name = match self.token.get(self.curr.get() + 1)? {
            Token::Ident(name) => {
                self.curr.set(self.curr.get() + 1);
                name
            }
            _ => {
                self.peek_error(Token::Ident(""));
                return None;
            }
        };

        match self.token.get(self.curr.get() + 1)? {
            Token::Assign => {
                self.curr.set(self.curr.get() + 1);
            }
            _ => {
                self.peek_error(Token::Assign);
                return None;
            }
        }

        // TODO: We need to read expression
        // Skip token until current token become Semicolon
        while let Some(value) = self.token.get(self.curr.get()) {
            if *value == Token::Semicolon {
                break;
            } else {
                self.curr.set(self.curr.get() + 1);
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
        while let Some(value) = self.token.get(self.curr.get()) {
            if *value == Token::Semicolon {
                break;
            } else {
                self.curr.set(self.curr.get() + 1);
            }
        }

        Some(Statement::ReturnStatement {
            token: Token::Return,
            ret_value: Expression::Dummy,
        })
    }

    fn parse_expression_statement(&self) -> Option<Statement> {
        let token = *self.token.get(self.curr.get())?;
        let expression = self.parse_expression()?;

        if *self.token.get(self.curr.get() + 1)? == Token::Semicolon {
            self.curr.set(self.curr.get() + 1);
        }

        Some(Statement::ExpressionStatement { token, expression })
    }

    fn parse_expression(&self, precedence: PriorityOrder) -> Option<Expression> {
        let curr_token = self.token.get(self.curr.get())?;
        let prefix = self.prefix_parse_fns.borrow().get(curr_token);
        if let Some(f) = prefix {
            f(self)
        } else {
            None
        }
    }

    fn parse_identifier(&self) -> Option<Expression> {
    }
}

impl<'a> Parser<'a> {
    fn register_prefix(
        &self, token: Token<'a>,
        f: fn(&Parser) -> Option<Expression<'a>>
    ) {
        self.prefix_parse_fns.borrow_mut().insert(token, f);
    }

    fn register_infix(
        &self, token: Token<'a>,
        f: fn(&Parser, Expression<'a>) -> Option<Expression<'a>>
    ) {
        self.infix_parse_fns.borrow_mut().insert(token, f);
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
}
