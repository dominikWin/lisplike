use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    Bool(bool),
    String(String),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(int) => write!(f, "{}", int),
            Value::Bool(value) => write!(f, "{}", if *value { "true" } else { "false" }),
            Value::String(string) => write!(f, "\"{}\"", string),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Value {
    pub fn is_integer(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn unwrap_integer(&self) -> i32 {
        match self {
            Value::Integer(int) => *int,
            _ => panic!(),
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match self {
            Value::Bool(val) => *val,
            _ => panic!(),
        }
    }

    pub fn unwrap_string(&self) -> &str {
        match self {
            Value::String(string) => string,
            _ => panic!(),
        }
    }
}
