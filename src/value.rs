use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(int) => write!(f, "{}", int),
            Value::Bool(value) => write!(f, "{}", if *value {"true"} else {"false"}),
            Value::Nil => write!(f, "nil"),
        }
    }
}