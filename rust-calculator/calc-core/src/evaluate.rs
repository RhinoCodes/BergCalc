use crate::expr::Expr; 
//use std::collections::HashMap;
/*use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;*/
use libm::{sin, cos, tan, exp, log, pow, log10};

pub fn eval(expr: &Expr) -> f64 {
    /*let variables = HashMap::from([
        ('x', 4.0),
        ('e', std::f64::consts::E),
    ]);*/

    match expr {
        Expr::Number(n) => *n,
        Expr::Negate(n) => -eval(n),
        Expr::Add(children) => children.iter().map(eval).sum(),
        Expr::Mult(children) => children.iter().map(eval).product(),
        Expr::Sub(children) => {
            let one = eval(&children[0]);
            let two = eval(&children[1]);
            one - two
        }
        Expr::Div(children) => {
            let one = eval(&children[0]);
            let two = eval(&children[1]);
            one / two
        }
        Expr::Pow(children) => {
            let base = eval(&children[0]);
            let exp = eval(&children[1]);

            pow(base, exp)
        }
        Expr::Null() => 0.0,
        Expr::Variable(n) => 0.0, //variables.get(n).copied().unwrap_or(0.0),
        Expr::Function(name, expre) => {
            match name.as_str() {
                "sin" => sin(eval(&expre)),
                "cos" => cos(eval(&expre)),
                "tan" => tan(eval(&expre)),
                "ln" => log(eval(&expre)),
                "log" => log10(eval(&expre)),
                "csc" => 1.0 / sin(eval(&expre)),
                "sec" => 1.0 / cos(eval(&expre)),
                "cot" => 1.0 / tan(eval(&expre)),
                "exp" => exp(eval(&expre)),
                _ => 0.0,
            }
        },
    }
}