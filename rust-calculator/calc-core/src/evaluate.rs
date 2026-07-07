use crate::expr::Expr;
use hashbrown::HashMap;
/*use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;*/
use libm::{cos, exp, log, log10, pow, sin, sqrt, tan};

pub struct Evaluator {
    vars: HashMap<char, f64>,
}

impl Evaluator {
    pub fn new() -> Self {
        let vars = HashMap::new();
        //vars.insert('e', core::f64::consts::E);
        Evaluator { vars }
    }

    pub fn eval_x(&self, expr: &Expr, x: f64) -> f64 {
        eval_vars(expr, &|c| match c {
            'x' => Some(x),
            'e' => Some(core::f64::consts::E),
            'π' => Some(core::f64::consts::PI),
            _ => self.vars.get(&c).copied(),
        })
    }

    pub fn eval(&mut self, expr: &Expr) -> f64 {
        eval_vars(expr, &|c| match c {
            'e' => Some(core::f64::consts::E),
            'π' => Some(core::f64::consts::PI),
            _ => self.vars.get(&c).copied(),
        })
    }
}

pub fn eval(expr: &Expr) -> f64 {
    eval_vars(expr, &|c| match c {
        'e' => Some(core::f64::consts::E),
        'π' => Some(core::f64::consts::PI),
        _ => Some(0.0),
    })
}

pub fn eval_vars<F: Fn(char) -> Option<f64>>(expr: &Expr, variables: &F) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Negate(n) => -eval_vars(n, variables),
        Expr::Add(children) => children.iter().map(|c| eval_vars(c, variables)).sum(),
        Expr::Mult(children) => children.iter().map(|c| eval_vars(c, variables)).product(),
        Expr::Sub(children) => {
            let one = eval_vars(&children[0], variables);
            let two = eval_vars(&children[1], variables);
            one - two
        }
        Expr::Div(children) => {
            let one = eval_vars(&children[0], variables);
            let two = eval_vars(&children[1], variables);
            one / two
        }
        Expr::Pow(children) => {
            let base = eval_vars(&children[0], variables);
            let exp = eval_vars(&children[1], variables);

            pow(base, exp)
        }
        Expr::Null() => 0.0,
        Expr::Variable(n) => variables(*n).unwrap_or(0.0),
        Expr::Function(name, expre) => match name.as_str() {
            "sin" => sin(eval_vars(&expre, variables)),
            "cos" => cos(eval_vars(&expre, variables)),
            "tan" => tan(eval_vars(&expre, variables)),
            "ln" => log(eval_vars(&expre, variables)),
            "log" => log10(eval_vars(&expre, variables)),
            "csc" => 1.0 / sin(eval_vars(&expre, variables)),
            "sec" => 1.0 / cos(eval_vars(&expre, variables)),
            "cot" => 1.0 / tan(eval_vars(&expre, variables)),
            "exp" => exp(eval_vars(&expre, variables)),
            "sqrt" => sqrt(eval_vars(&expre, variables)),
            "abs" => eval_vars(&expre, variables).abs(),
            _ => 0.0,
        },
    }
}
