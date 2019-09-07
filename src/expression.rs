use crate::ops::get_op;
use crate::tokenizer::{tokenize, Token};
use crate::value::Value;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    Expression(Token, Vec<Expression>),
}

impl Expression {
    pub fn eval(&self) -> Value {
        match self {
            Expression::Value(v) => (*v).clone(),
            Expression::Expression(token, args) => {
                let func_name = match token {
                    Token::Symbol(text) => text,
                    _ => panic!(),
                };
                let func = get_op(func_name).unwrap();
                func.eval(args)
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
        if let Token::Integer(int) = &first_token {
            return Expression::Value(Value::Integer(*int));
        }
        if let Token::Symbol(string) = &first_token {
            match string.as_str() {
                "nil" => return Expression::Value(Value::Nil),
                _ => {},
            }
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
            assert_eq!(Expression::from("5").eval(), Value::Integer(5));
        }

        #[test]
        fn test_add() {
            assert_eq!(Expression::from("(+ 4 5)").eval(), Value::Integer(9));
        }

        #[test]
        fn test_mul() {
            assert_eq!(Expression::from("(* 4 5)").eval(), Value::Integer(20));
        }

        #[test]
        fn test_sub() {
            assert_eq!(Expression::from("(- 4 5)").eval(), Value::Integer(-1));
        }

        #[test]
        fn test_div() {
            assert_eq!(Expression::from("(/ 63 10)").eval(), Value::Integer(6));
        }

        #[test]
        fn test_math() {
            assert_eq!(
                Expression::from("(+ 4 (* 3 5) (* 4 6))").eval(),
                Value::Integer(43)
            );
        }

        #[test]
        fn test_print_int() {
            assert_eq!(Expression::from("(print 5)").eval(), Value::Nil);
        }

        #[test]
        fn test_print_nil() {
            assert_eq!(Expression::from("(print nil)").eval(), Value::Nil);
        }
    }
}
