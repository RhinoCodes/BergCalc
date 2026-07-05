use std::iter::chain;

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
        Expr::Variable(_c) => Expr::Number(1.0),
        Expr::Pow(children) => {
            if let Expr::Number(n) = children[1].clone() {
                Expr::Mult(vec![
                    differentiate(&children[0]),
                    Expr::Mult(vec![
                        children[1].clone(),
                        Expr::Pow(vec![children[0].clone(), Expr::Number(n - 1.0)]),
                    ]),
                ])
            } else {
                panic!("Power function has non numeric exponent");
            }
        }
        Expr::Sub(children) => Expr::Sub(vec![
            differentiate(&children[0]),
            differentiate(&children[1]),
        ]),
        Expr::Div(children) => Expr::Div(vec![
            Expr::Sub(vec![
                Expr::Mult(vec![children[1].clone(), differentiate(&children[0])]),
                Expr::Mult(vec![children[0].clone(), differentiate(&children[1])]),
            ]),
            Expr::Pow(vec![children[1].clone(), Expr::Number(2.0)]),
        ]),
        Expr::Function(name, ex) => {
            let chain_rule = differentiate(ex);
            let function = match name.as_str() {
                // Trig Functions
                "sin" => Expr::Function("cos".to_string(), ex.clone()),
                "cos" => Expr::Negate(Box::from(Expr::Function("sin".to_string(), ex.clone()))),
                "tan" => Expr::Pow(vec![
                    Expr::Function("sec".to_string(), ex.clone()),
                    Expr::Number(2.0),
                ]),
                "sec" => Expr::Mult(vec![
                    Expr::Function("sec".to_string(), ex.clone()),
                    Expr::Function("tan".to_string(), ex.clone()),
                ]),
                "csc" => Expr::Negate(Box::from(Expr::Mult(vec![
                    Expr::Function("csc".to_string(), ex.clone()),
                    Expr::Function("cot".to_string(), ex.clone()),
                ]))),
                "cot" => Expr::Negate(Box::from(Expr::Pow(vec![
                    Expr::Function("csc".to_string(), ex.clone()),
                    Expr::Number(2.0),
                ]))),
                // Misc Functions
                "exp" => Expr::Function("exp".to_string(), (*ex).clone()),
                "ln" => Expr::Div(vec![Expr::Number(1.0), (**ex).clone()]),
                "log" => Expr::Div(vec![
                    Expr::Number(1.0),
                    Expr::Mult(vec![
                        Expr::Function("ln".to_string(), Box::from(Expr::Number(10.0))),
                        (**ex).clone(),
                    ]),
                ]),
                _ => Expr::Number(0.0),
            };
            Expr::Mult(vec![chain_rule, function])
        } // TODO
        _ => Expr::Null(),
    }
}
