use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(int) => write!(f, "{}", int)
        }
    }
}