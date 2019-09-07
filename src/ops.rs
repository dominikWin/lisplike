use crate::context::Context;
use crate::expression::Expression;
use crate::value::Value;

pub trait Operation {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value;
}

pub fn get_op(name: &str) -> Option<Box<dyn Operation>> {
    match name {
        "+" => Option::Some(Box::new(OpAdd {})),
        "*" => Option::Some(Box::new(OpMul {})),
        "-" => Option::Some(Box::new(OpSub {})),
        "/" => Option::Some(Box::new(OpDiv {})),
        "print" => Option::Some(Box::new(OpPrint {})),
        "if" => Option::Some(Box::new(OpIf {})),
        "block" => Option::Some(Box::new(OpBlock {})),
        "global" => Option::Some(Box::new(OpGlobal {})),
        _ => Option::None,
    }
}

struct OpAdd {}

impl Operation for OpAdd {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert!(args.len() > 0);
        let mut sum = 0;
        for val in args {
            let val = val.eval(context);
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
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert!(args.len() > 0);
        let mut product = 1;
        for val in args {
            let val = val.eval(context);
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
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = if let Value::Integer(int) = args[0].eval(context) {
            int
        } else {
            panic!();
        };
        let right = if let Value::Integer(int) = args[1].eval(context) {
            int
        } else {
            panic!();
        };

        Value::Integer(left - right)
    }
}

struct OpDiv {}

impl Operation for OpDiv {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = if let Value::Integer(int) = args[0].eval(context) {
            int
        } else {
            panic!();
        };
        let right = if let Value::Integer(int) = args[1].eval(context) {
            int
        } else {
            panic!();
        };

        Value::Integer(left / right)
    }
}

struct OpPrint {}

impl Operation for OpPrint {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 1);
        println!("{}", args[0].eval(context));
        Value::Nil
    }
}

struct OpIf {}

impl Operation for OpIf {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert!(args.len() >= 2 && args.len() <= 3);
        let control = if let Value::Bool(val) = args[0].eval(context) {
            val
        } else {
            panic!();
        };

        if control {
            return args[1].eval(context);
        }

        if args.len() == 3 {
            args[2].eval(context)
        } else {
            Value::Nil
        }
    }
}

struct OpBlock {}

impl Operation for OpBlock {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        let mut last_val = Value::Nil;

        for arg in args {
            last_val = arg.eval(context);
        }

        last_val
    }
}

struct OpGlobal {}

impl Operation for OpGlobal {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let global_name = if let Expression::Symbol(symbol) = &args[0] {
            symbol.to_string()
        } else {
            panic!();
        };
        let value = args[1].eval(context);
        context.globals.insert(global_name, value);

        Value::Nil
    }
}
