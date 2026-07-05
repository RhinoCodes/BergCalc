use crate::expr::Expr; 

pub fn eval(expr: &Expr) -> f64 {
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

            base.powf(exp)
        }
        Expr::Null() => 0.0,
        Expr::Variable(_n) => 0.0,
        Expr::Function(name, exp) => {
            match name.as_str() {
                "sin" => eval(&exp).sin(),
                "cos" => eval(&exp).cos(),
                "tan" => eval(&exp).tan(),
                "ln" => eval(&exp).ln(),
                "log" => eval(&exp).log10(),
                "csc" => 1.0 / eval(&exp).sin(),
                "sec" => 1.0 / eval(&exp).cos(),
                "cot" => 1.0 / eval(&exp).tan(),
                "exp" => eval(&exp).exp(),
                _ => 0.0,
            }
        },
    }
}