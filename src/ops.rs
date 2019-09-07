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
        "=" => Option::Some(Box::new(OpEq {})),
        "<" => Option::Some(Box::new(OpLt {})),
        ">" => Option::Some(Box::new(OpGt {})),
        "and" => Option::Some(Box::new(OpAnd {})),
        "or" => Option::Some(Box::new(OpOr {})),
        "not" => Option::Some(Box::new(OpNot {})),
        "print" => Option::Some(Box::new(OpPrint {})),
        "if" => Option::Some(Box::new(OpIf {})),
        "while" => Option::Some(Box::new(OpWhile {})),
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
            sum += val.unwrap_integer();
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
            product *= val.unwrap_integer();
        }
        Value::Integer(product)
    }
}

struct OpSub {}

impl Operation for OpSub {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = args[0].eval(context).unwrap_integer();
        let right = args[1].eval(context).unwrap_integer();
        Value::Integer(left - right)
    }
}

struct OpDiv {}

impl Operation for OpDiv {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = args[0].eval(context).unwrap_integer();
        let right = args[1].eval(context).unwrap_integer();
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
        let control = args[0].eval(context).unwrap_bool();

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

struct OpWhile {}

impl Operation for OpWhile {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        loop {
            let control = args[0].eval(context).unwrap_bool();
            if !control {
                break;
            }
            args[1].eval(context);
        }
        Value::Nil
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

struct OpEq {}

impl Operation for OpEq {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = args[0].eval(context);
        let right = args[1].eval(context);
        Value::Bool(left == right)
    }
}

struct OpLt {}

impl Operation for OpLt {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = args[0].eval(context).unwrap_integer();
        let right = args[1].eval(context).unwrap_integer();
        Value::Bool(left < right)
    }
}

struct OpGt {}

impl Operation for OpGt {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = args[0].eval(context).unwrap_integer();
        let right = args[1].eval(context).unwrap_integer();
        Value::Bool(left > right)
    }
}

struct OpAnd {}

impl Operation for OpAnd {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = args[0].eval(context).unwrap_bool();
        let right = args[1].eval(context).unwrap_bool();
        Value::Bool(left && right)
    }
}

struct OpOr {}

impl Operation for OpOr {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 2);
        let left = args[0].eval(context).unwrap_bool();
        let right = args[1].eval(context).unwrap_bool();
        Value::Bool(left || right)
    }
}

struct OpNot {}

impl Operation for OpNot {
    fn eval(&self, args: &[Expression], context: &mut Context) -> Value {
        assert_eq!(args.len(), 1);
        let val = args[0].eval(context).unwrap_bool();
        Value::Bool(!val)
    }
}
