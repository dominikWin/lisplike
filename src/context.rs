use crate::value::Value;
use std::collections::HashMap;

pub struct Context {
    pub globals: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            globals: HashMap::new(),
        }
    }
}
