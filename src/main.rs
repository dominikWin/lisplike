mod context;
mod expression;
mod ops;
mod tokenizer;
mod value;

use context::Context;
use expression::Expression;
use std::io::{self, Write};

fn main() {
    let mut context = Context::new();
    let mut line = String::new();
    loop {
        if line.is_empty() {
            print!("repl> ");
        }
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            println!();
            break;
        }
        if line.is_empty() || line == "\n" {
            line.clear();
            continue;
        }
        if line.chars().filter(|c| *c == '(').count() > line.chars().filter(|c| *c == ')').count() {
            continue;
        }
        let expr: Expression = Expression::from(line.as_str());
        line.clear();
        println!("{}", expr.eval(&mut context));
    }
}
