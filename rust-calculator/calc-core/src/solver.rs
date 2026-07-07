use crate::expr::Expr; 
use crate::differentiate::differentiate;
use crate::evaluate::eval_x;
use crate::simplify::simplify;
use alloc::vec::Vec; 

pub fn newtons_method(expr: &Expr, guess: f64) -> f64 {
    let fx = |x: f64| eval_x(expr, x);
    let dx = |x: f64| eval_x(&simplify(&differentiate(&expr)), x);
    let mut guess = guess;
    for i in 1..100 {
        if (dx(guess) == 0.0) {
            break;
        }
        let correction = fx(guess) / dx(guess);
        guess -= correction;
        if correction < 1e-15 {
            break;
        }
    }
    guess
}

pub fn on_interval(expr: &Expr, open: f64, close: f64) -> Vec<f64> {
    let fx = |x: f64| eval_x(expr, x);
    let dx = |x: f64| eval_x(&simplify(&differentiate(&expr)), x);
    let mut at = open;
    let mut last = fx(open);
    let mut guesses = Vec::new();

    let mut zeroes = Vec::new();
    while at <= close {
        let new = fx(at);
        if new == 0.0 {
            zeroes.push(at);
            at += 0.0625;
            continue;
        }
        if new * last < 0.0 {
            guesses.push((2.0 * at - 0.0625) / 2.0);
        } else if dx(at) * dx(at - 0.0625) < 0.0 {
            let extrema = newtons_method(&simplify(&differentiate(&expr)), (2.0 * at - 0.0625) / 2.0);
            if extrema < 1e-6 {
                guesses.push(extrema);
            }
        }
        last = new;
        at += 0.0625;
    }
    for guess in guesses.iter() { 
        zeroes.push(newtons_method(expr, *guess));
    }
    zeroes
}