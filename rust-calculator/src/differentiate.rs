use crate::expr::Expr;

pub fn differentiate(expr: &Expr) -> Expr {
    match expr {
        Expr::Number(n) => Expr::Number(0.0),
        Expr::Negate(n) => Expr::Negate(Box::from(differentiate(n))),
        Expr::Add(children) => Expr::Add(vec![
            differentiate(&children[0]),
            differentiate(&children[1]),
        ]),
        Expr::Mult(children) => Expr::Add(vec![
            Expr::Mult(vec![differentiate(&children[0]), children[1].clone()]),
            Expr::Mult(vec![differentiate(&children[1]), children[0].clone()]),
        ]),
        Expr::Variable(c) => Expr::Number(1.0),
        Expr::Pow(children) => {
            if let Expr::Number(n) = children[1].clone() {
                Expr::Mult(vec![
                    children[1].clone(),
                    Expr::Pow(vec![
                        children[0].clone(),
                        Expr::Number(n - 1.0)
                    ])
                ])
            } else {
                panic!("Power function has non numeric exponent");
            }
        },
        Expr::Sub(children) => Expr::Sub(vec![
            differentiate(&children[0]),
            differentiate(&children[1]),
        ]),
        Expr::Div(children) => Expr::Div(vec![
            Expr::Sub(vec![
                Expr::Mult(vec![
                    children[1].clone(),
                    differentiate(&children[0])
                ]),
                Expr::Mult(vec![
                    children[0].clone(),
                    differentiate(&children[1])
                ])
            ]),
            Expr::Pow(vec![
                children[1].clone(),
                Expr::Number(2.0)
            ])
        ]),
        Expr::Function(name, ex) => Expr::Null(), // TODO
        _ => Expr::Null(),
    }
}
