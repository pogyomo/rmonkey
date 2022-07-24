pub mod env;

use crate::{
    ast::{Program, Statement, Expression, IfExpression},
    object::{Object, Integer, Null, Boolean, ReturnValue, ErrorObj, ObjectTrait, Function},
    token::TokenKind
};
use self::env::Env;

pub struct Eval<'a> {
    env: &'a mut Env,
}

impl<'a> Eval<'a> {
    pub fn new(env: &mut Env) -> Eval {
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

impl<'a> Eval<'a> {
    fn stmt(&mut self, stmt: Statement) -> Object {
        match stmt {
            Statement::Exp(stmt) => self.expr(stmt.exp),
            Statement::Let(stmt) => {
                let name    = stmt.ident.name.clone();
                let rhs_exp = self.expr(stmt.rhs_exp);
                self.env.set(name, rhs_exp.clone());
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
                match self.env.get(&ident.name) {
                    Some(obj) => obj.clone(),
                    _ => Object::Err(ErrorObj::new(format!("Identifier not found: {}", ident.name)))
                }
            }
            Expression::Int(int)       => Object::Int(Integer::new(int.value)),
            Expression::Bool(bool)     => Object::Bool(Boolean::new(bool.value)),

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

            Expression::If(if_expr) => {
                self.if_expr(if_expr)
            }
            Expression::Func(func) => {
                let params = func.params;
                let body   = func.body;
                Object::Func(Function::new(params, body, Env::new()))
            }

            _ => Object::Err(ErrorObj::new(format!("Unknown expression found: {:?}", expr))),
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
}

impl<'a> Eval<'a> {
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
