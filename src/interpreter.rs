
use std::fmt;
use crate::expr::{Visitor, Expr, Binary, Grouping, Unary, Variable as VariableExpr, walk_expr, Assign, Logical};
use crate::lox_error;
use crate::token::{Literal, Token};
use crate::object::Object;
use crate::token_type::TokenType;
use crate::stmt::{Stmt, Visitor as StmtVisitor, Expression, Print, walk_stmt, Variable, Block, If};
use crate::environment::{Environment};
use std::cell::RefCell;
use std::rc::Rc;



pub struct Interpreter {
    // i am using a refcell since i need to mutate environment in place in the visit_block_stm
    // and I am using RC so it's consistent with Environment.enclosing type
    environment: RefCell<Rc<Environment>>,
}

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

impl Visitor<Result<Object, RuntimeError>> for Interpreter{
    fn visit_logicalexp(&self, e: &Logical) -> Result<Object, RuntimeError> {
        let left = self.evaluate(&e.left)?;
        match e.condition.kind {
            TokenType::Or => {
                if self.is_truthy(&left) {
                    return Ok(left)
                }
            }
            TokenType::And => {
                if !self.is_truthy(&left){
                    return Ok(left)
                }
            }
            _ => unreachable!()
        }

        return self.evaluate(&e.right);
    }

    fn visit_variableexp(&self, e: &VariableExpr) -> Result<Object, RuntimeError> {
        return self.environment.borrow().get(&e.name)
    }

    fn visit_assignexp(&self, expr: &Assign) -> Result<Object, RuntimeError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.borrow().assign(&expr.name, value.clone())?;
        Ok(value)
    }

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

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_if_stmt(&self, stmt: &If) -> Result<(), RuntimeError> {
        if self.is_truthy(&self.evaluate(&stmt.condition)?){
            self.execute(&stmt.then_branch);
        } else {
            // if else_branch is not not then execute it
            stmt.else_branch.as_ref().map(|e| self.execute(e));
        }
        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &Block) -> Result<(), RuntimeError> {
        let sr = Rc::clone(&self.environment.borrow());
        self.execute_block(&stmt.statements, Rc::new(Environment::new(Some(sr))))
    }

    fn visit_var_stmt(&self, stmt: &Variable) -> Result<(), RuntimeError> {
        // in the book, the initializer can be None and needs to be handled differently. However, for us
        // the initializer can't be None - it's an enum whose value is type Literal::Nil which evaluates to None.
        // This means we don't need to handle it separately.
        let value = self.evaluate(&stmt.initializer);
        self.environment.borrow().define(stmt.name.lexeme.to_owned(), value?);
        Ok(())
    }


    fn visit_expression(&self, expr: &Expression) -> Result<(), RuntimeError> {
        match self.evaluate(&expr.expression){
            Err(e) => Err(e),
            Ok(_) => Ok(())
        }
    }

    fn visit_print(&self, stmt: &Print) -> Result<(), RuntimeError> {
        let obj = self.evaluate(&stmt.expression);

        match self.evaluate(&stmt.expression) {
            Err(e) => Err(e),
            Ok(obj) => {
                println!("{}", self.stringify(&obj));
                Ok(())
            }
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: RefCell::new(Rc::new(Environment::new(None)))
        }
    }

    pub fn interpret(&self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            match self.execute(&stmt) {
                Err(e) => {
                    lox_error(&e.token, &e.message)
                },
                _ => (),
            }
        }
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), RuntimeError>{
        return walk_stmt(self, stmt)
    }

    fn execute_block(&self, statements: &Vec<Stmt>, environment: Rc<Environment>) -> Result<(), RuntimeError>{
        // temporarily change the enviornment to current block's. Once done revert back to previous bloke.
        let previous = self.environment.replace(environment);

        // this is similar to try finally block from python/java
        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));
        self.environment.replace(previous);
        result
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
        walk_expr(self, expr)
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
