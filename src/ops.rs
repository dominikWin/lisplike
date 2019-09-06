use crate::expression::Expression;
use crate::value::Value;

pub trait Operation {
    fn get_name() -> &'static str;

    fn eval(args: &[Expression]) -> Value;
}