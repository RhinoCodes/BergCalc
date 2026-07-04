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
    }
}