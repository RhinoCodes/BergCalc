use std::io::{self, Write};

mod evaluate;
mod expr;
mod parser;
mod differentiate;
mod simplify;

use evaluate::eval;
use parser::parse;
use parser::tree;
use differentiate::differentiate;
use simplify::simplify;
fn main() {
    loop {
        print!("Enter expression: ");
        io::stdout().flush().unwrap(); // ensure the prompt prints before waiting for input

        let mut calc = String::new();
        io::stdin()
            .read_line(&mut calc)
            .expect("Failed to read input");

        let calc = calc.trim();
        let result = parse(calc);
        println!("{:?}", result);
        println!("{:#?}", tree(&result));
        println!("{}", simplify(&differentiate(&tree(&result))));
    }
}
