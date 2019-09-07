use crate::expression::Expression;
use crate::value::Value;

pub trait Operation {
    fn eval(&self, args: &[Expression]) -> Value;
}

pub fn get_op(name: &str) -> Option<Box<dyn Operation>> {
    match name {
        "+" => Option::Some(Box::new(OpAdd {})),
        "*" => Option::Some(Box::new(OpMul {})),
        "-" => Option::Some(Box::new(OpSub {})),
        "/" => Option::Some(Box::new(OpDiv {})),
        _ => Option::None,
    }
}

struct OpAdd {}

impl Operation for OpAdd {
    fn eval(&self, args: &[Expression]) -> Value {
        assert!(args.len() > 0);
        let mut sum = 0;
        for val in args {
            let val = val.eval();
            if let Value::Integer(int) = val {
                sum += int;
            } else {
                panic!();
            }
        }
        Value::Integer(sum)
    }
}

struct OpMul {}

impl Operation for OpMul {
    fn eval(&self, args: &[Expression]) -> Value {
        assert!(args.len() > 0);
        let mut product = 1;
        for val in args {
            let val = val.eval();
            if let Value::Integer(int) = val {
                product *= int;
            } else {
                panic!();
            }
        }
        Value::Integer(product)
    }
}

struct OpSub {}

impl Operation for OpSub {
    fn eval(&self, args: &[Expression]) -> Value {
        assert_eq!(args.len(), 2);
        let left = if let Value::Integer(int) = args[0].eval() {
            int
        } else {
            panic!();
        };
        let right = if let Value::Integer(int) = args[1].eval() {
            int
        } else {
            panic!();
        };

        Value::Integer(left - right)
    }
}

struct OpDiv {}

impl Operation for OpDiv {
    fn eval(&self, args: &[Expression]) -> Value {
        assert_eq!(args.len(), 2);
        let left = if let Value::Integer(int) = args[0].eval() {
            int
        } else {
            panic!();
        };
        let right = if let Value::Integer(int) = args[1].eval() {
            int
        } else {
            panic!();
        };

        Value::Integer(left / right)
    }
}
