use crate::evaluate::eval;
use crate::expr::Expr;
use alloc::boxed::Box;
use alloc::vec; // brings in the vec! macro
use alloc::vec::Vec; // brings in the Vec type
//use alloc::string::String;
use alloc::string::ToString;

pub fn simplify(expr: &Expr) -> Expr {
    let expression = (*expr).clone();
    match expr {
        Expr::Number(n) => Expr::Number(*n),
        Expr::Negate(n) => {
            let inside = simplify(n);
            Expr::Negate(Box::from(inside))
        }
        Expr::Add(children) => {
            let mut simple = Vec::new();
            for child in children {
                let simple_child = simplify(child);
                if simple_child != Expr::Number(0.0) {
                    simple.push(simple_child.clone());
                }
            }
            if simple.len() == 1 {
                return simple[0].clone();
            }
            Expr::Add(simple)
        }
        Expr::Mult(children) => {
            let mut simple = Vec::new();
            for child in children {
                let simple_child = simplify(child);
                if simple_child == Expr::Number(0.0) {
                    return Expr::Number(0.0);
                } else if simple_child != Expr::Number(1.0) {
                    simple.push(simple_child.clone());
                }
            }
            if simple.len() == 1 {
                return simple[0].clone();
            }
            let simplified = Expr::Mult(simple);
            if simplified.is_evaluable() {
                return Expr::Number(eval(&simplified));
            } else {
                return simplified;
            }
        }
        Expr::Sub(children) => {
            let one = simplify(&children[0]);
            let two = simplify(&children[1]);
            if two == Expr::Number(0.0) {
                return one;
            } else if one == Expr::Number(0.0) {
                return Expr::Negate(Box::from(two));
            }
            Expr::Sub(vec![one, two])
        }
        Expr::Div(children) => {
            let one = simplify(&children[0]);
            let two = simplify(&children[1]);
            if one == Expr::Number(0.0) {
                return Expr::Number(0.0);
            } else if two == Expr::Number(1.0) {
                return one;
            }
            Expr::Div(vec![one, two])
        }
        Expr::Pow(children) => {
            let base = simplify(&children[0]);
            let exp = simplify(&children[1]);
            let mut abs = false;
            if let Expr::Pow(children) = base {
                if let Expr::Number(n) = exp {
                    if n % 2.0 == 0.0 {
                        abs = true;
                    }
                }
                if !abs && let Expr::Number(n) = children[1].clone() {
                    if n % 2.0 == 0.0 {
                        abs = true;
                    }
                }
                if !abs {
                    simplify(&Expr::Pow(vec![
                        children[0].clone(),
                        simplify(&Expr::Mult(vec![children[1].clone(), exp])),
                    ]))
                } else {
                    simplify(&Expr::Function(
                        "abs".to_string(),
                        Box::from(Expr::Pow(vec![
                            children[0].clone(),
                            simplify(&Expr::Mult(vec![children[1].clone(), exp])),
                        ])),
                    ))
                }
            } else {
                if exp == Expr::Number(0.0) {
                    return Expr::Number(1.0);
                } else if exp == Expr::Number(1.0) {
                    return base;
                }
                Expr::Pow(vec![base, exp])
            }
        }
        Expr::Function(name, exp) => Expr::Function(name.to_string(), Box::from(simplify(exp))),
        _ => expression,
    }
}
