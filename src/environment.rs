use std::collections::HashMap;
use std::cell::RefCell;
use crate::{interpreter::RuntimeError, object::Object, token::Token};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: RefCell<HashMap<String, Object>>, // Refcell so i don't keep passing mut self.
    // it would be really ineficciennt to store a copy of every environment.
    // so it makes sense to store a reference. I am using Rc since I don't
    // wanna deal with lifetime annotations to use & references.
    // RC makes it so mans can co-own the john.
    enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<Environment>>) -> Self {
        Environment {
            variables: RefCell::new(HashMap::new()),
            enclosing: enclosing,
        }
    }

    pub fn define(&self, name: String, obj: Object){
        self.variables.borrow_mut().insert(name, obj);
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        // if value not in in current scope, walk the chain and check the parent scope until it cannot be found
        if let Some(value) = self.variables.borrow().get(&name.lexeme).cloned() {
            return Ok(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        Err(RuntimeError::new(
            Token::default(),
            &format!("Undefined variable '{}'", name.lexeme),
        ))
    }

    pub fn assign(&self, name: &Token, obj: Object) -> Result<(), RuntimeError>{
        if self.variables.borrow().contains_key(&name.lexeme) {
            self.define(name.lexeme.clone(), obj);
            return Ok(());
        }

        // gotta check if the john has an enclosing
        if let Some(enclosing) = &self.enclosing {
            return enclosing.assign(name, obj);
        }

        Err(RuntimeError::new(Token::default(), &format!("Undefined variable '{}'", name.lexeme)))
    }
}
