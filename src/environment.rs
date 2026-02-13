use std::collections::HashMap;
use crate::{interpreter::RuntimeError, object::Object, token::Token};

struct Environment {
    variables: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { variables: HashMap::new() }
    }

    pub fn define(&mut self, name: String, obj: Object){
        self.variables.insert(name, obj);
    }

    pub fn get(&self, name: &String) -> Result<&Object, RuntimeError> {
        self.variables.get(name).ok_or_else(| | RuntimeError::new(Token::default(), &format!("Undefined variable '{}'", name)))
    }
}
