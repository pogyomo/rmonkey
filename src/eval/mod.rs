pub mod env;
pub mod object;

use crate::{
    ast::{Program, Statement, Expression, IfExpression, CallExpression},
    token::TokenKind
};
use self::{
    env::Env,
    object::{Object, Integer, Null, Boolean, ReturnValue, ErrorObj, ObjectTrait, Function, StringObj, Buildin},
};
use std::{cell::RefCell, rc::Rc};

pub struct Eval {
    env: Rc<RefCell<Env>>,
}

impl Eval {
    pub fn new(env: Rc<RefCell<Env>>) -> Eval {
        env.borrow_mut().set(
            "puts".to_string(),
            Object::Buildin(Buildin::new(Buildin::print))
        );
        env.borrow_mut().set(
            "exit".to_string(),
            Object::Buildin(Buildin::new(Buildin::exit))
        );
        Eval { env }
    }

    pub fn eval(&mut self, prog: Program) -> Object {
        let mut ret: Object = Object::Null(Null::new());
        for stmt in prog.statements.into_iter() {
            ret = self.stmt(stmt);
            match ret {
                Object::Ret(ret_val) => return *ret_val.value,
                Object::Err(_)       => return ret,
                _ => (),
            }
        }
        ret
    }
}

impl Eval {
    fn stmt(&mut self, stmt: Statement) -> Object {
        match stmt {
            Statement::Exp(stmt) => self.expr(stmt.exp),
            Statement::Let(stmt) => {
                let name    = stmt.ident.name.clone();
                let rhs_exp = self.expr(stmt.rhs_exp);
                self.env.borrow_mut().set(name, rhs_exp.clone());
                rhs_exp
            }
            Statement::Ret(stmt) => Object::Ret(ReturnValue::new(self.expr(stmt.exp))),
            Statement::Blk(stmt) => {
                let mut ret: Object = Object::Null(Null::new());
                for stmt in stmt.statements.into_iter() {
                    ret = self.stmt(stmt);
                    match ret {
                        Object::Ret(_) => return ret,
                        Object::Err(_) => return ret,
                        _ => (),
                    }
                }
                ret
            }
        }
    }

    fn expr(&mut self, expr: Expression) -> Object {
        match expr {
            Expression::Ident(ident)   => {
                match self.env.borrow().get(&ident.name) {
                    Some(obj) => obj.clone(),
                    _ => Object::Err(ErrorObj::new(format!("Identifier not found: {}", ident.name)))
                }
            }
            Expression::Int(int)       => Object::Int(Integer::new(int.value)),
            Expression::Bool(bool)     => Object::Bool(Boolean::new(bool.value)),
            Expression::Str(string)    => Object::Str(StringObj::new(string.str.clone())),

            Expression::Prefix(prefix) => {
                let right = self.expr(prefix.rhs_exp);
                if !self.is_error(&right) {
                    self.prefix(prefix.operator, right)
                } else {
                    right
                }
            }
            Expression::Infix(infix) => {
                let right = self.expr(infix.rhs_exp);
                if self.is_error(&right) {
                    return right;
                }

                let left  = self.expr(infix.lhs_exp);
                if self.is_error(&left) {
                    return left;
                }

                self.infix(infix.operator, left, right)
            }
            Expression::Postfix(postfix) => {
                let right = self.expr(postfix.lhs_exp);
                if !self.is_error(&right) {
                    self.postfix(postfix.operator, right)
                } else {
                    right
                }
            }

            Expression::If(if_expr) => {
                self.if_expr(if_expr)
            }
            Expression::Func(func) => {
                let params = func.params;
                let body   = func.body;
                Object::Func(Function::new(params, body, Rc::clone(&self.env)))
            }
            Expression::Call(call) => {
                self.call_func(call)
            }
        }
    }

    fn postfix(&self, op: TokenKind, right: Object) -> Object {
        match op {
            TokenKind::Inc => {
                match right {
                    Object::Int(int) => Object::Int(Integer::new(int.value + 1)),
                    obj => {
                        Object::Err(
                            ErrorObj::new(
                                format!(
                                    "Invalid uses of postfix operator: {:?} can't applied to {}",
                                    op, obj.inspect()
                                )
                            )
                        )
                    }
                }
            }
            TokenKind::Dec => {
                match right {
                    Object::Int(int) => Object::Int(Integer::new(int.value - 1)),
                    obj => {
                        Object::Err(
                            ErrorObj::new(
                                format!(
                                    "Invalid uses of postfix operator: {:?} can't applied to {}",
                                    op, obj.inspect()
                                )
                            )
                        )
                    }
                }
            }
            _ => Object::Err(ErrorObj::new(format!("Unknown postfix operator: {:?}", op))),
        }
    }

    fn prefix(&self, op: TokenKind, right: Object) -> Object {
        match op {
            TokenKind::Bang => {
                match right {
                    Object::Bool(bool) => Object::Bool(Boolean::new(!bool.value)),
                    Object::Null(_)    => Object::Bool(Boolean::new(false)),
                    _                  => Object::Bool(Boolean::new(false)),
                }
            }
            TokenKind::Minus => {
                match right {
                    Object::Int(int) => Object::Int(Integer::new(-int.value)),
                    obj => {
                        Object::Err(
                            ErrorObj::new(
                                format!(
                                    "Invalid uses of prefix operator: {:?} can't applied to {}",
                                    op, obj.inspect()
                                )
                            )
                        )
                    }
                }
            }
            TokenKind::Inc => {
                match right {
                    Object::Int(int) => Object::Int(Integer::new(int.value + 1)),
                    obj => {
                        Object::Err(
                            ErrorObj::new(
                                format!(
                                    "Invalid uses of prefix operator: {:?} can't applied to {}",
                                    op, obj.inspect()
                                )
                            )
                        )
                    }
                }
            }
            TokenKind::Dec => {
                match right {
                    Object::Int(int) => Object::Int(Integer::new(int.value - 1)),
                    obj => {
                        Object::Err(
                            ErrorObj::new(
                                format!(
                                    "Invalid uses of prefix operator: {:?} can't applied to {}",
                                    op, obj.inspect()
                                )
                            )
                        )
                    }
                }
            }
            _ => Object::Err(ErrorObj::new(format!("Unknown prefix operator: {:?}", op))),
        }
    }

    fn infix(&self, op: TokenKind, left: Object, right: Object) -> Object {
        if let (Object::Int(left), Object::Int(right)) = (&left, &right) {
            let left  = left.value;
            let right = right.value;
            return match op {
                TokenKind::Plus     => Object::Int(Integer::new(left + right)),
                TokenKind::Minus    => Object::Int(Integer::new(left - right)),
                TokenKind::Asterisk => Object::Int(Integer::new(left * right)),
                TokenKind::Slash    => Object::Int(Integer::new(left / right)),
                TokenKind::LT       => Object::Bool(Boolean::new(left < right)),
                TokenKind::GT       => Object::Bool(Boolean::new(left > right)),
                _ => {
                    Object::Err(
                        ErrorObj::new(
                            format!(
                                "Invalid uses of infix operator: {:?} can't applied to {} and {}",
                                op, left, right
                            )
                        )
                    )
                }
            };
        }

        if let (Object::Bool(left), Object::Bool(right)) = (&left, &right) {
            let left  = left.value;
            let right = right.value;
            return match op {
                TokenKind::Eq       => Object::Bool(Boolean::new(left == right)),
                TokenKind::NotEq    => Object::Bool(Boolean::new(left != right)),
                _ => {
                    Object::Err(
                        ErrorObj::new(
                            format!(
                                "Invalid uses of infix operator: {:?} can't applied to {} and {}",
                                op, left, right
                            )
                        )
                    )
                }
            };
        }

        if let (Object::Str(left), Object::Str(right)) = (&left, &right) {
            let left  = &left.str;
            let right = &right.str;
            return match op {
                TokenKind::Plus  => Object::Str(StringObj::new(format!("{}{}", left, right))),
                TokenKind::Eq    => Object::Bool(Boolean::new(left == right)),
                TokenKind::NotEq => Object::Bool(Boolean::new(left != right)),
                _ => {
                    Object::Err(
                        ErrorObj::new(
                            format!(
                                "Invalid uses of infix operator: {:?} can't applied to {} and {}",
                                op, left, right
                            )
                        )
                    )
                }
            };
        }

        Object::Err(
            ErrorObj::new(
                format!(
                    "You can't use {} or {} as operand", left.inspect(), right.inspect()
                )
            )
        )
    }

    fn if_expr(&mut self, if_expr: IfExpression) -> Object {
        let cond = self.expr(*if_expr.condition);
        if self.is_error(&cond) {
            return cond;
        } else {
            if self.is_truthy(&cond) {
                return self.stmt(Statement::Blk(if_expr.consequence));
            }else if let Some(stmt) = if_expr.alternative {
                return self.stmt(Statement::Blk(stmt));
            } else {
                Object::Err(ErrorObj::new(format!("No else found")))
            }
        }
    }

    fn call_func(&mut self, call: CallExpression) -> Object {
        let func = match *call.ident {
            Expression::Ident(ident) => self.expr(Expression::Ident(ident)),
            Expression::Func(func)   => self.expr(Expression::Func(func)),
            Expression::Call(call)   => self.expr(Expression::Call(call)),
            _ => return Object::Err(ErrorObj::new(format!("You can't call function from {:?}", call))),
        };

        let func = match func {
            Object::Func(func) => func,
            Object::Buildin(b) => return (b.func)(call.args.iter().map(|arg| self.expr(arg.clone())).collect()),
            _ => return Object::Err(ErrorObj::new(format!("{:?} is not a function", func))),
        };

        if call.args.len() != func.params.len() {
            return Object::Err(
                ErrorObj::new(
                    format!(
                        "Number of argument is less than or greater than: expect {}, got {}",
                        func.params.len(), call.args.len()
                    )
                )
            );
        }
        let mut args = Vec::new();
        for (arg, ident) in call.args.iter().zip(func.params.iter()) {
            let name = ident.name.clone();
            let arg  = self.expr(arg.clone());
            args.push((name, arg));
        }

        let mut local_env = Env::new_with_outer(Rc::clone(&func.env));
        for (name, arg) in args.iter() {
            local_env.set(name.clone(), arg.clone());
        }

        let curr_env = Rc::clone(&self.env);
        self.env = Rc::new(RefCell::new(local_env));
        let ret = self.stmt(Statement::Blk(func.body));
        self.env = curr_env;

        match ret {
            Object::Ret(ret) => *ret.value,
            _ => ret,
        }
    }
}

impl Eval {
    fn is_truthy(&self, obj: &Object) -> bool {
        match obj {
            Object::Null(_)    => false,
            Object::Bool(bool) => bool.value,
            _                  => true,
        }
    }

    fn is_error(&self, obj: &Object) -> bool {
        match obj {
            Object::Err(_) => true,
            _              => false,
        }
    }
}
