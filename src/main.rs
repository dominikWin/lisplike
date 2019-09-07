mod expression;
mod ops;
mod tokenizer;
mod value;

use std::io::{self, Write};
use expression::Expression;

fn main() {
    loop {
        let mut line = String::new();
        print!("repl> ");
        io::stdout().flush();
        io::stdin().read_line(&mut line).unwrap();
        let expr: Expression = Expression::from(line.as_str());
        println!("{}", expr.eval());
    }
}
