mod test;

use crate::{token::{Token, TokenKind}, ast::{Program, Statement}};
use std::cell::Cell;

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
        todo!()
    }
}

impl<'a> Parser<'a> {
    fn curr_token(&self) -> Option<Token> {
        match self.token.get(self.curr.get() + 0) {
            Some(ret) => Some(*ret),
            None      => None,
        }
    }

    fn peek_token(&self) -> Option<Token> {
        match self.token.get(self.curr.get() + 1) {
            Some(ret) => Some(*ret),
            None      => None,
        }
    }

    fn expect_token(&self, kind: TokenKind) {
        if let Some(token) = self.peek_token() {
            if token.kind == kind {
                self.next_token();
            }
        }
    }

    fn next_token(&self) {
        self.curr.set(self.curr.get().wrapping_add(1));
    }
}
