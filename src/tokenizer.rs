#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParen,
    RParen,
    Nil,
    Symbol(String),
    Integer(i32),
    Bool(bool),
}

impl From<&str> for Token {
    fn from(string: &str) -> Self {
        let mut token = match string {
            "(" => Token::LParen,
            ")" => Token::RParen,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "nil" => Token::Nil,
            text => Token::Symbol(text.to_string()),
        };

        if let Token::Symbol(text) = &token {
            assert!(!text.is_empty());
            if let Ok(int) = text.parse::<i32>() {
                token = Token::Integer(int);
            }
        }

        token
    }
}

fn split_syntax(string: &str) -> Vec<String> {
    let mut out = vec![];

    let mut buffer = String::new();

    for c in string.chars() {
        let buffer = &mut buffer;
        if c.is_whitespace() {
            if !buffer.is_empty() {
                out.push(buffer.clone());
                buffer.clear();
            }
        } else if c == '(' || c == ')' {
            if !buffer.is_empty() {
                out.push(buffer.clone());
                buffer.clear();
            }
            buffer.push(c);
            if !buffer.is_empty() {
                out.push(buffer.clone());
                buffer.clear();
            }
        } else {
            buffer.push(c);
        }
    }
    if !buffer.is_empty() {
        out.push(buffer.clone());
        buffer.clear();
    }

    out
}

pub fn tokenize(string: &str) -> Vec<Token> {
    let parts = split_syntax(string);
    parts
        .iter()
        .map(|part| Token::from(part.as_ref()))
        .collect::<Vec<Token>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod split {
        use super::*;

        #[test]
        fn test_empty() {
            assert_eq!(split_syntax(""), Vec::<String>::new());
        }

        #[test]
        fn test_just_whitespace() {
            assert_eq!(split_syntax("  \t"), Vec::<String>::new());
        }

        #[test]
        fn test_just_newline() {
            assert_eq!(split_syntax("\n"), Vec::<String>::new());
        }

        #[test]
        fn test_int() {
            assert_eq!(split_syntax("5"), vec!["5"]);
        }

        #[test]
        fn test_int_with_whitespace() {
            assert_eq!(split_syntax("\t\t5  "), vec!["5"]);
        }

        #[test]
        fn test_neg_int_with_whitespace() {
            assert_eq!(split_syntax("-8  "), vec!["-8"]);
        }

        #[test]
        fn test_math_operators() {
            assert_eq!(split_syntax("\t\t +\t- * /     "), vec!["+", "-", "*", "/"]);
        }

        #[test]
        fn test_basic_math_symbols() {
            assert_eq!(
                split_syntax("3 * 2 / / + 881     \t-3  -\t"),
                vec!["3", "*", "2", "/", "/", "+", "881", "-3", "-"]
            );
        }

        #[test]
        fn test_l_paren() {
            assert_eq!(split_syntax("  \t("), vec!["("]);
        }

        #[test]
        fn test_r_paren() {
            assert_eq!(split_syntax("  \t)\t\t"), vec![")"]);
        }

        #[test]
        fn test_multiple_parens() {
            assert_eq!(split_syntax("(( \n)( "), vec!["(", "(", ")", "("]);
        }

        #[test]
        fn test_mixed_with_parens() {
            assert_eq!(
                split_syntax("7(3)(+))"),
                vec!["7", "(", "3", ")", "(", "+", ")", ")"]
            );
        }

        #[test]
        fn test_math_expr() {
            assert_eq!(
                split_syntax("(+ 4 (* 3 5))"),
                vec!["(", "+", "4", "(", "*", "3", "5", ")", ")"]
            );
        }
    }

    mod str_to_token {
        use super::*;

        #[test]
        fn test_lparen() {
            assert_eq!(Token::from("("), Token::LParen);
        }

        #[test]
        fn test_rparen() {
            assert_eq!(Token::from("("), Token::LParen);
        }

        #[test]
        fn test_nil() {
            assert_eq!(Token::from("nil"), Token::Nil);
        }

        #[test]
        fn test_bool() {
            assert_eq!(Token::from("true"), Token::Bool(true));
            assert_eq!(Token::from("false"), Token::Bool(false));
        }

        #[test]
        fn test_integer() {
            assert_eq!(Token::from("123"), Token::Integer(123));
        }

        #[test]
        fn test_neg_integer() {
            assert_eq!(Token::from("-88"), Token::Integer(-88));
        }

        #[test]
        fn test_arbitrary_symbol() {
            assert_eq!(Token::from("432af"), Token::Symbol("432af".to_string()));
        }

        #[test]
        fn test_math_operators() {
            assert_eq!(Token::from("+"), Token::Symbol("+".to_string()));
            assert_eq!(Token::from("-"), Token::Symbol("-".to_string()));
            assert_eq!(Token::from("*"), Token::Symbol("*".to_string()));
            assert_eq!(Token::from("/"), Token::Symbol("/".to_string()));
        }
    }

    mod tokenize {
        use super::*;

        #[test]
        fn test_empty() {
            assert_eq!(tokenize(""), vec![]);
        }

        #[test]
        fn test_math_expr() {
            assert_eq!(
                tokenize("(+ 4 (* 3 5))"),
                vec![
                    Token::LParen,
                    Token::Symbol("+".to_string()),
                    Token::Integer(4),
                    Token::LParen,
                    Token::Symbol("*".to_string()),
                    Token::Integer(3),
                    Token::Integer(5),
                    Token::RParen,
                    Token::RParen
                ]
            );
        }
    }
}
