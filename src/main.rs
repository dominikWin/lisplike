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
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            println!();
            break;
        }
        if line.is_empty() || line == "\n" {
            continue;
        }
        let expr: Expression = Expression::from(line.as_str());
        println!("{}", expr.eval());
    }
}
