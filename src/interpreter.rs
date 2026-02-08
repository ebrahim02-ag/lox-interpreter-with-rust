
use std::fmt;
use crate::expr::{Visitor, Expr, Binary, Grouping, Unary, walk_expr};
use crate::lox_error;
use crate::token::{Literal, Token};
use crate::object::Object;
use crate::token_type::TokenType;

pub struct Interpreter;
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str) -> Self {
        RuntimeError {
            token,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}] {}", self.token.line, self.message)
    }
}

impl Visitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_binaryexp(&self, e: &Binary) -> Result<Object, RuntimeError> {
        let left = self.evaluate(&e.left)?;
        let right = self.evaluate(&e.right)?;

        match e.op.kind {
            TokenType::Minus => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l-r)),
                _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a number"))
            },
            TokenType::Plus => match (left, right) {
                // you can either add numbers or concat two strings
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l+r)),
                (Object::String(l), Object::String(r)) => {
                    let mut l = l.to_owned();
                    let r = r.as_str();
                    l.push_str(r);
                    Ok(Object::String(l))
                }
                _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a numbers or strings"))
            },
            TokenType::Star => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l*r)),
                _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a numbers"))
            },
            TokenType::Slash => match (left, right) {
                (Object::Number(l), Object::Number(r)) => {
                    if r == 0.0 {
                        return Err(RuntimeError::new(e.op.clone(), "attempted to divide by 0"))
                    }
                    Ok(Object::Number(l/r))
                },
                _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a numbers"))
            },
            TokenType::Greater => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l > r)),
                _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a numbers"))
            },
            TokenType::GreaterEqual => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l >= r)),
                _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a numbers"))
            },
            TokenType::Less => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l < r)),
                _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a numbers"))
            },
            TokenType::LessEqual => match (left, right) {
                (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l <= r)),
                _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a numbers"))
            },
            TokenType::EqualEqual => {
                Ok(Object::Boolean(self.is_equal(&left, &right)))
            },
            TokenType::BangEqual => {
                Ok(Object::Boolean(!self.is_equal(&left, &right)))
            }
            _ => unreachable!()
        }

    }

    fn visit_groupingexp(&self, e: &Grouping) -> Result<Object, RuntimeError> {
        return self.evaluate(&e.expression)
    }

    fn visit_literalexp(&self, e: &Literal) -> Result<Object, RuntimeError> {
        let literal = e.clone();
        match literal {
            Literal::Bool(i) => Ok(Object::Boolean(i)),
            Literal::Nil => Ok(Object::Null),
            Literal::Number(i) => Ok(Object::Number(i)),
            Literal::String(i) => Ok(Object::String(i))
        }
    }

    fn visit_unaryexp(&self, e: &Unary) -> Result<Object, RuntimeError> {
        // Evaluate the right hand expression, but negate if it the operator is !/-.
        let right = self.evaluate(&e.right)?;
        match e.op.kind {
            TokenType::Minus => {
                match right {
                    Object::Number(right) => Ok(Object::Number(-right)),
                    _ => Err(RuntimeError::new(e.op.clone(), "Operand must be a number"))
                }
            },
            TokenType::Bang => { // shout out zhangbanger
                Ok(Object::Boolean(!self.is_truthy(&right)))
            }
            _ => unreachable!()
        }

    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self{}
    }

    pub fn interpret(&self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(obj) => println!("{}", self.stringify(&obj)),
            Err(e) => {
                lox_error(&e.token, &e.message)
            },
        }
    }

    fn stringify(&self, obj: &Object) -> String {
        match obj {
            Object::Null => "nil".to_string(),
            Object::Number(i) => {
                let mut str = i.to_string();
                if str.ends_with(".0") {
                    str.pop();
                    str.pop();
                }
                str
            }
            ,
            Object::Boolean(i) => i.to_string(),
            Object::String(i) => i.to_owned(),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object, RuntimeError>{
        return walk_expr(self, expr)
    }

    fn is_truthy(&self, obj: &Object) -> bool{
        // what is the truth? (Some might sriracha is the best hot sauce).
        // If Object is Null or false then return false, otherwise return true
        match obj {
            Object::Boolean(i) => i.clone(),
            Object::Null => false,
            _ => true
        }
    }

    fn is_equal(&self, left: &Object, right: &Object) -> bool {
        // if bouth are null then true, if both have same type and return their boolean results
        // otherwise it would be false. Two items of different types cannot be equal!!!
        match (left, right) {
            (Object::Null, Object::Null) => true,
            (Object::Boolean(l), Object::Boolean(r)) => l == r,
            (Object::Number(l), Object::Number(r)) => l == r,
            (Object::String(l), Object::String(r)) => l == r,
            _ => false
        }
    }

}
