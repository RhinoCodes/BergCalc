use crate::expr::Expr; 
use hashbrown::HashMap;
/*use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;*/
use libm::{sin, cos, tan, exp, log, pow, sqrt, log10};

pub fn eval(expr: &Expr) -> f64 {
    eval_vars(expr, &HashMap::new())
}

pub fn eval_x(expr: &Expr, x: f64) -> f64 {
    let mut variables = HashMap::new();
    variables.insert('x', x);
    eval_vars(expr, &variables)
}

pub fn eval_vars(expr: &Expr, vars: &HashMap<char, f64>) -> f64 {
    let mut variables = vars.clone();
    variables.insert('e', core::f64::consts::E);
    match expr {
        Expr::Number(n) => *n,
        Expr::Negate(n) => -eval_vars(n, &variables),
        Expr::Add(children) => children.iter().map(|c| eval_vars(c, &variables)).sum(),
        Expr::Mult(children) => children.iter().map(|c| eval_vars(c, &variables)).product(),
        Expr::Sub(children) => {
            let one = eval_vars(&children[0], &variables);
            let two = eval_vars(&children[1], &variables);
            one - two
        }
        Expr::Div(children) => {
            let one = eval_vars(&children[0], &variables);
            let two = eval_vars(&children[1], &variables);
            one / two
        }
        Expr::Pow(children) => {
            let base = eval_vars(&children[0], &variables);
            let exp = eval_vars(&children[1], &variables);

            pow(base, exp)
        }
        Expr::Null() => 0.0,
        Expr::Variable(n) => variables.get(n).copied().unwrap_or(0.0),
        Expr::Function(name, expre) => {
            match name.as_str() {
                "sin" => sin(eval_vars(&expre, &variables)),
                "cos" => cos(eval_vars(&expre, &variables)),
                "tan" => tan(eval_vars(&expre, &variables)),
                "ln" => log(eval_vars(&expre, &variables)),
                "log" => log10(eval_vars(&expre, &variables)),
                "csc" => 1.0 / sin(eval_vars(&expre, &variables)),
                "sec" => 1.0 / cos(eval_vars(&expre, &variables)),
                "cot" => 1.0 / tan(eval_vars(&expre, &variables)),
                "exp" => exp(eval_vars(&expre, &variables)),
                "sqrt" => sqrt(eval_vars(&expre, &variables)),
                "abs" => eval_vars(&expre, &variables).abs(),
                _ => 0.0,
            }
        },
    }
}