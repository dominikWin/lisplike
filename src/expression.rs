use crate::tokenizer::{tokenize, Token};
use crate::value::Value;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    Expression(Token, Vec<Expression>),
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
                    vec![Expression::Value(Value::Integer(1)), Expression::Value(Value::Integer(2))]
                )
            );
        }
    }

    mod from_string {
        use super::*;
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
                    vec![Expression::Value(Value::Integer(1)), Expression::Value(Value::Integer(2))]
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
                            vec![Expression::Value(Value::Integer(3)), Expression::Value(Value::Integer(5))]
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
                            vec![Expression::Value(Value::Integer(3)), Expression::Value(Value::Integer(5))]
                        ),
                        Expression::Expression(
                            Token::Symbol("*".to_string()),
                            vec![Expression::Value(Value::Integer(4)), Expression::Value(Value::Integer(6))]
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
                            vec![Expression::Value(Value::Integer(3)), Expression::Value(Value::Integer(5))]
                        ),
                        Expression::Expression(
                            Token::Symbol("*".to_string()),
                            vec![
                                Expression::Value(Value::Integer(4)),
                                Expression::Expression(
                                    Token::Symbol("*".to_string()),
                                    vec![Expression::Value(Value::Integer(3)), Expression::Value(Value::Integer(5))]
                                )
                            ]
                        )
                    ]
                )
            );
        }
    }
}
