use crate::{interpreter::Value, token::Token, RunTimeError};
use std::collections::HashMap;
use std::default;

// TODO Figure out how to do enclosing without a clone
#[derive(Debug, Clone, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn enclosing(enclosing: Environment) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn parent(&self) -> Option<&Box<Environment>> {
        self.enclosing.as_ref()
    }

    pub fn put(&mut self, name: String, value: Value) {
        let _ = self.values.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> Result<Value, RunTimeError> {
        match self.values.get(&token.lexeme).cloned() {
            Some(value) => Ok(value),
            None => match &self.enclosing {
                Some(env) => env.get(token),
                None => Err(RunTimeError::UndefinedVariable(token.clone())),
            },
        }
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Result<(), RunTimeError> {
        match self
            .values
            .get_mut(&token.lexeme)
            .map(|v| *v = value.clone())
        {
            Some(v) => Ok(v),
            None => match &mut self.enclosing {
                Some(env) => env.assign(token, value),
                None => Err(RunTimeError::UndefinedVariable(token.clone())),
            },
        }
    }
}
