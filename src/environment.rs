use std::collections::HashMap;
use std::cell::RefCell;
use crate::{interpreter::RuntimeError, object::Object, token::Token};

pub struct Environment {
    variables: RefCell<HashMap<String, Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { variables: RefCell::new(HashMap::new()) }
    }

    pub fn define(&self, name: String, obj: Object){
        self.variables.borrow_mut().insert(name, obj);
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        self.variables.borrow().get(&name.lexeme).cloned().ok_or_else(| | RuntimeError::new(Token::default(), &format!("Undefined variable '{}'", name.lexeme)))
    }
}
