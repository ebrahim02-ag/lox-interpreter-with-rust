
use crate::expr::{Visitor, Expr, Binary, Grouping, Unary, walk_expr};
use crate::token::{Literal};
use crate::object::Object;
use crate::token_type::TokenType;

pub struct Interpreter;
pub struct InterpError;

impl Visitor<Result<Object, InterpError>> for Interpreter {
    fn visit_binaryexp(&mut self, e: &Binary) -> Result<Object, InterpError> {
        Err(InterpError)
    }

    fn visit_groupingexp(&mut self, e: &Grouping) -> Result<Object, InterpError> {
        return self.evaluate(&e.expression)
    }

    fn visit_literalexp(&mut self, e: &Literal) -> Result<Object, InterpError> {
        let literal = e.clone();
        match literal {
            Literal::Bool(i) => Ok(Object::Boolean(i)),
            Literal::Nil => Ok(Object::Null),
            Literal::Number(i) => Ok(Object::Number(i)),
            Literal::String(i) => Ok(Object::String(i))
        }
    }

    fn visit_unaryexp(&mut self, e: &Unary) -> Result<Object, InterpError> {
        // Evaluate the right hand expression, but negate if it the operator is !/-.
        let right = self.evaluate(&e.right)?;
        match e.op.kind {
            TokenType::Minus => {
                match right {
                    Object::Number(right) => Ok(Object::Number(-right)),
                    _ => unreachable!()
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
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, InterpError>{
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
}
