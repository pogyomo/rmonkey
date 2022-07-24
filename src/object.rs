use crate::{eval::env::Env, ast::{Identifier, BlkStatement, Node}};
use std::{rc::Rc, cell::RefCell};

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Object {
    Int(Integer),
    Bool(Boolean),
    Null(Null),
    Ret(ReturnValue),
    Func(Function),
    Err(ErrorObj),
}

impl ObjectTrait for Object {
    fn inspect(&self) -> String {
        match self {
            Self::Int(integer)  => integer.inspect(),
            Self::Bool(boolean) => boolean.inspect(),
            Self::Null(null)    => null.inspect(),
            Self::Ret(ret)      => ret.inspect(),
            Self::Func(func)    => func.inspect(),
            Self::Err(err)      => err.inspect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Integer {
        Integer { value }
    }
}

impl ObjectTrait for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Boolean {
        Boolean { value }
    }
}

impl ObjectTrait for Boolean {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Null;

impl Null {
    pub fn new() -> Null {
        Null {}
    }
}

impl ObjectTrait for Null {
    fn inspect(&self) -> String {
        format!("null")
    }
}

#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub value: Box<Object>,
}

impl ReturnValue {
    pub fn new(value: Object) -> ReturnValue {
        ReturnValue { value: Box::new(value) }
    }
}

impl ObjectTrait for ReturnValue {
    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

#[derive(Debug, Clone)]
pub struct ErrorObj {
    pub msg: String,
}

impl ErrorObj {
    pub fn new(msg: String) -> ErrorObj {
        ErrorObj { msg }
    }
}

impl ObjectTrait for ErrorObj {
    fn inspect(&self) -> String {
        format!("An error happen: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<Identifier>,
    pub body: BlkStatement,
    pub env:  Rc<RefCell<Env>>,
}

impl Function {
    pub fn new(params: Vec<Identifier>, body: BlkStatement, env: Rc<RefCell<Env>>) -> Function {
        Function { params, body, env }
    }
}

impl ObjectTrait for Function {
    fn inspect(&self) -> String {
        let mut ret = String::new();
        ret.push_str("fn(");
        for ident in self.params.iter() {
            ret.push_str(ident.string().as_str());
            ret.push_str(", ")
        }
        ret.pop();
        ret.pop();
        ret.push_str("){\n");
        ret.push_str(self.body.string().as_str());
        ret.push_str("\n}");
        ret
    }
}
