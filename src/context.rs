use crate::value::Value;
use std::collections::HashMap;

pub struct Context {
    globals: HashMap<String, Value>
}

impl Context {
    pub fn new() -> Self {
        Context {
            globals: HashMap::new(),
        }
    }
}