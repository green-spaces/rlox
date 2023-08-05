use std::collections::HashMap;

use crate::{interpreter::Value, token::Token, RunTimeError};

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn put(&mut self, name: String, value: Value) {
        let _ = self.values.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> Result<Value, RunTimeError> {
        self.values
            .get(&token.lexeme)
            .cloned()
            .ok_or_else(|| RunTimeError::UndefinedVariable(token.clone()))
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Result<(), RunTimeError> {
        self.values
            .get_mut(&token.lexeme)
            .map(|v| *v = value)
            .ok_or_else(|| RunTimeError::UndefinedVariable(token.clone()))
    }
}
