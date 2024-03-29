use crate::context::Context;
use crate::ops::get_op;
use crate::tokenizer::{tokenize, Token};
use crate::value::Value;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    Symbol(String),
    Expression(Token, Vec<Expression>),
}

impl Expression {
    pub fn eval(&self, context: &mut Context) -> Value {
        match self {
            Expression::Value(v) => (*v).clone(),
            Expression::Symbol(symbol) => context.globals.get(symbol).unwrap().clone(),
            Expression::Expression(token, args) => {
                let func_name = match token {
                    Token::Symbol(text) => text,
                    _ => panic!(),
                };
                let func = get_op(func_name).unwrap();
                func.eval(args, context)
            }
        }
    }
}

impl From<&[Token]> for Expression {
    fn from(tokens: &[Token]) -> Self {
        Expression::from(&mut tokens.iter().cloned().collect::<VecDeque<Token>>())
    }
}

impl From<&mut VecDeque<Token>> for Expression {
    fn from(tokens: &mut VecDeque<Token>) -> Self {
        assert!(tokens.len() >= 1);

        let first_token = tokens.pop_front().unwrap();
        match &first_token {
            Token::Nil => {
                return Expression::Value(Value::Nil);
            }
            Token::Integer(int) => {
                return Expression::Value(Value::Integer(*int));
            }
            Token::Bool(value) => {
                return Expression::Value(Value::Bool(*value));
            }
            Token::String(string) => {
                return Expression::Value(Value::String(string.to_string()));
            }
            Token::Symbol(value) => {
                return Expression::Symbol(value.to_string());
            }
            _ => {}
        }

        assert_eq!(first_token, Token::LParen);

        let op = tokens.pop_front().unwrap().clone();
        let mut args = Vec::<Expression>::new();

        loop {
            if tokens.front().unwrap() == &Token::RParen {
                tokens.pop_front();
                break;
            }

            args.push(Expression::from(&mut *tokens));
        }

        Expression::Expression(op, args)
    }
}

impl From<&str> for Expression {
    fn from(string: &str) -> Self {
        Expression::from(tokenize(string).as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_tokens {
        use super::*;
        #[test]
        fn test_int() {
            assert_eq!(
                Expression::from(vec![Token::Integer(3)].as_slice()),
                Expression::Value(Value::Integer(3))
            );
        }

        #[test]
        fn test_simple_add() {
            assert_eq!(
                Expression::from("(+ 1 2)"),
                Expression::Expression(
                    Token::Symbol("+".to_string()),
                    vec![
                        Expression::Value(Value::Integer(1)),
                        Expression::Value(Value::Integer(2))
                    ]
                )
            );
        }
    }

    mod from_string {
        use super::*;

        #[test]
        fn test_nil() {
            assert_eq!(Expression::from("nil"), Expression::Value(Value::Nil));
        }

        #[test]
        fn test_int() {
            assert_eq!(Expression::from("6"), Expression::Value(Value::Integer(6)));
        }

        #[test]
        fn test_simple_add() {
            assert_eq!(
                Expression::from("(+ 1 2)"),
                Expression::Expression(
                    Token::Symbol("+".to_string()),
                    vec![
                        Expression::Value(Value::Integer(1)),
                        Expression::Value(Value::Integer(2))
                    ]
                )
            );
        }

        #[test]
        fn test_math_expr() {
            assert_eq!(
                Expression::from("(+ 4 (* 3 5))"),
                Expression::Expression(
                    Token::Symbol("+".to_string()),
                    vec![
                        Expression::Value(Value::Integer(4)),
                        Expression::Expression(
                            Token::Symbol("*".to_string()),
                            vec![
                                Expression::Value(Value::Integer(3)),
                                Expression::Value(Value::Integer(5))
                            ]
                        )
                    ]
                )
            );
        }

        #[test]
        fn test_math_expr2() {
            assert_eq!(
                Expression::from("(+ 4 (* 3 5) (* 4 6))"),
                Expression::Expression(
                    Token::Symbol("+".to_string()),
                    vec![
                        Expression::Value(Value::Integer(4)),
                        Expression::Expression(
                            Token::Symbol("*".to_string()),
                            vec![
                                Expression::Value(Value::Integer(3)),
                                Expression::Value(Value::Integer(5))
                            ]
                        ),
                        Expression::Expression(
                            Token::Symbol("*".to_string()),
                            vec![
                                Expression::Value(Value::Integer(4)),
                                Expression::Value(Value::Integer(6))
                            ]
                        )
                    ]
                )
            );
        }

        #[test]
        fn test_math_expr3() {
            assert_eq!(
                Expression::from("(+ 4 (* 3 5) (* 4 (* 3 5)))"),
                Expression::Expression(
                    Token::Symbol("+".to_string()),
                    vec![
                        Expression::Value(Value::Integer(4)),
                        Expression::Expression(
                            Token::Symbol("*".to_string()),
                            vec![
                                Expression::Value(Value::Integer(3)),
                                Expression::Value(Value::Integer(5))
                            ]
                        ),
                        Expression::Expression(
                            Token::Symbol("*".to_string()),
                            vec![
                                Expression::Value(Value::Integer(4)),
                                Expression::Expression(
                                    Token::Symbol("*".to_string()),
                                    vec![
                                        Expression::Value(Value::Integer(3)),
                                        Expression::Value(Value::Integer(5))
                                    ]
                                )
                            ]
                        )
                    ]
                )
            );
        }
    }

    mod eval {
        use super::*;
        #[test]
        fn test_int() {
            assert_eq!(
                Expression::from("5").eval(&mut Context::new()),
                Value::Integer(5)
            );
        }

        #[test]
        fn test_nil() {
            assert_eq!(
                Expression::from("nil").eval(&mut Context::new()),
                Value::Nil
            );
        }

        #[test]
        fn test_bool() {
            assert_eq!(
                Expression::from("true").eval(&mut Context::new()),
                Value::Bool(true)
            );

            assert_eq!(
                Expression::from("false").eval(&mut Context::new()),
                Value::Bool(false)
            );
        }

        #[test]
        fn test_string() {
            assert_eq!(
                Expression::from("\"Hello, world!\"").eval(&mut Context::new()),
                Value::String("Hello, world!".to_string())
            );
        }

        #[test]
        fn test_add() {
            assert_eq!(
                Expression::from("(+ 4 5)").eval(&mut Context::new()),
                Value::Integer(9)
            );
        }

        #[test]
        fn test_mul() {
            assert_eq!(
                Expression::from("(* 4 5)").eval(&mut Context::new()),
                Value::Integer(20)
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                Expression::from("(- 4 5)").eval(&mut Context::new()),
                Value::Integer(-1)
            );
        }

        #[test]
        fn test_div() {
            assert_eq!(
                Expression::from("(/ 63 10)").eval(&mut Context::new()),
                Value::Integer(6)
            );
        }

        #[test]
        fn test_mod() {
            assert_eq!(
                Expression::from("(% 63 100)").eval(&mut Context::new()),
                Value::Integer(63)
            );

            assert_eq!(
                Expression::from("(% 101 2)").eval(&mut Context::new()),
                Value::Integer(1)
            );
        }

        #[test]
        fn test_eq() {
            assert_eq!(
                Expression::from("(= 63 10)").eval(&mut Context::new()),
                Value::Bool(false)
            );
            assert_eq!(
                Expression::from("(= 63 63)").eval(&mut Context::new()),
                Value::Bool(true)
            );

            assert_eq!(
                Expression::from("(= true true)").eval(&mut Context::new()),
                Value::Bool(true)
            );
            assert_eq!(
                Expression::from("(= true false)").eval(&mut Context::new()),
                Value::Bool(false)
            );

            assert_eq!(
                Expression::from("(= nil nil)").eval(&mut Context::new()),
                Value::Bool(true)
            );
        }

        #[test]
        fn test_lt() {
            assert_eq!(
                Expression::from("(< 63 63)").eval(&mut Context::new()),
                Value::Bool(false)
            );
            assert_eq!(
                Expression::from("(< 5 63)").eval(&mut Context::new()),
                Value::Bool(true)
            );
            assert_eq!(
                Expression::from("(< 7 3)").eval(&mut Context::new()),
                Value::Bool(false)
            );
        }

        #[test]
        fn test_gt() {
            assert_eq!(
                Expression::from("(> 63 63)").eval(&mut Context::new()),
                Value::Bool(false)
            );
            assert_eq!(
                Expression::from("(> 5 63)").eval(&mut Context::new()),
                Value::Bool(false)
            );
            assert_eq!(
                Expression::from("(> 7 3)").eval(&mut Context::new()),
                Value::Bool(true)
            );
        }

        #[test]
        fn test_and() {
            assert_eq!(
                Expression::from("(and true true)").eval(&mut Context::new()),
                Value::Bool(true)
            );
            assert_eq!(
                Expression::from("(and true false)").eval(&mut Context::new()),
                Value::Bool(false)
            );
            assert_eq!(
                Expression::from("(and false true)").eval(&mut Context::new()),
                Value::Bool(false)
            );
            assert_eq!(
                Expression::from("(and false false)").eval(&mut Context::new()),
                Value::Bool(false)
            );
        }

        #[test]
        fn test_or() {
            assert_eq!(
                Expression::from("(or true true)").eval(&mut Context::new()),
                Value::Bool(true)
            );
            assert_eq!(
                Expression::from("(or true false)").eval(&mut Context::new()),
                Value::Bool(true)
            );
            assert_eq!(
                Expression::from("(or false true)").eval(&mut Context::new()),
                Value::Bool(true)
            );
            assert_eq!(
                Expression::from("(or false false)").eval(&mut Context::new()),
                Value::Bool(false)
            );
        }

        #[test]
        fn test_not() {
            assert_eq!(
                Expression::from("(not true)").eval(&mut Context::new()),
                Value::Bool(false)
            );
            assert_eq!(
                Expression::from("(not false)").eval(&mut Context::new()),
                Value::Bool(true)
            );
        }

        #[test]
        fn test_math() {
            assert_eq!(
                Expression::from("(+ 4 (* 3 5) (* 4 6))").eval(&mut Context::new()),
                Value::Integer(43)
            );
        }

        #[test]
        fn test_print_int() {
            assert_eq!(
                Expression::from("(print 5)").eval(&mut Context::new()),
                Value::Nil
            );
        }

        #[test]
        fn test_print_nil() {
            assert_eq!(
                Expression::from("(print nil)").eval(&mut Context::new()),
                Value::Nil
            );
        }

        #[test]
        fn test_if() {
            assert_eq!(
                Expression::from("(if true 4 nil)").eval(&mut Context::new()),
                Value::Integer(4)
            );

            assert_eq!(
                Expression::from("(if false 4 nil)").eval(&mut Context::new()),
                Value::Nil
            );
        }

        #[test]
        fn test_if_else() {
            assert_eq!(
                Expression::from("(if true 4 nil)").eval(&mut Context::new()),
                Value::Integer(4)
            );

            assert_eq!(
                Expression::from("(if false 4 nil)").eval(&mut Context::new()),
                Value::Nil
            );

            assert_eq!(
                Expression::from("(if true nil 4)").eval(&mut Context::new()),
                Value::Nil
            );

            assert_eq!(
                Expression::from("(if false 4 4)").eval(&mut Context::new()),
                Value::Integer(4)
            );
        }

        #[test]
        fn test_while() {
            assert_eq!(
                Expression::from(
                    "(block (global i 0) (while (< i 10) (block (print i) (global i (+ i 1)))) i)"
                )
                .eval(&mut Context::new()),
                Value::Integer(10)
            );
        }

        #[test]
        fn test_block() {
            assert_eq!(
                Expression::from("(block)").eval(&mut Context::new()),
                Value::Nil
            );

            assert_eq!(
                Expression::from("(block 5)").eval(&mut Context::new()),
                Value::Integer(5)
            );

            assert_eq!(
                Expression::from("(block 5 7)").eval(&mut Context::new()),
                Value::Integer(7)
            );

            assert_eq!(
                Expression::from("(block (+ 5 2) 5 1 true)").eval(&mut Context::new()),
                Value::Bool(true)
            );
        }

        #[test]
        fn test_global() {
            let mut context = Context::new();
            assert_eq!(
                Expression::from("(global abc 4)").eval(&mut context),
                Value::Nil
            );
            assert_eq!(context.globals.get("abc"), Option::Some(&Value::Integer(4)));
        }

        #[test]
        fn test_global_multi() {
            let mut context = Context::new();
            assert_eq!(
                Expression::from("(block (global abc 4) (global a 1) (global abc 7) a)")
                    .eval(&mut context),
                Value::Integer(1)
            );
            assert_eq!(context.globals.get("abc"), Option::Some(&Value::Integer(7)));
        }
    }
}
