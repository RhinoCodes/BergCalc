use crate::differentiate::differentiate;
use crate::evaluate::Evaluator;
use crate::expr::Expr;
use crate::simplify::simplify;
use alloc::vec::Vec;
use alloc::vec;

fn find_extrema(
    mut lo: f64,
    mut hi: f64,
    dx: impl Fn(f64) -> f64,
    ddx: impl Fn(f64) -> f64,
) -> f64 {
    let epsilon = 1e-12;
    let mut x = (lo + hi) / 2.0;
    let mut dx_lo = dx(lo);
    for _ in 0..100 {
        let d = dx(x);
        if d.abs() < epsilon {
            return x;
        };

        let dd = ddx(x);
        let x_new = if dd.abs() > 1e-8 {
            x - d / dd
        } else {
            (lo + hi) / 2.0
        };

        let x_new = if x_new < lo || x_new > hi {
            (lo + hi) / 2.0
        } else {
            x_new
        };
        if dx_lo * dx(x_new) < 0.0 {
            hi = x_new;
        } else {
            lo = x_new;
            dx_lo = dx(lo);
        };

        x = x_new;
    }
    x
}

pub fn newtons_method(guess: f64, fx: impl Fn(f64) -> f64, dx: impl Fn(f64) -> f64) -> f64 {
    let mut guess = guess;
    for _ in 1..100 {
        let fx_guess = fx(guess);
        if fx_guess == 0.0 {
            return guess;
        }
        let correction = fx_guess / dx(guess);
        guess -= correction;
        if correction.abs() < 1e-15 {
            break;
        }
    }
    guess
}

pub fn on_interval(expr: &Expr, open: f64, close: f64) -> Vec<f64> {
    intersections_on_interval(expr, &Expr::Number(0.0), open, close)
}

pub fn intersections_on_interval(expr_one: &Expr, expr_two: &Expr, open: f64, close: f64) -> Vec<f64> {
    let expr = &simplify(&Expr::Sub(vec![
        expr_one.clone(),
        expr_two.clone()
    ]));
    let evaluator = Evaluator::new();
    let fx = |x: f64| evaluator.eval_x(expr, x);
    let derivative = &simplify(&differentiate(&expr));
    let second_deriv = &simplify(&differentiate(derivative));
    let dx = |x: f64| evaluator.eval_x(derivative, x);
    let ddx = |x: f64| evaluator.eval_x(second_deriv, x);

    let mut at = open;
    let mut last = fx(open);
    let mut guesses = Vec::new();

    let mut zeroes = Vec::new();
    while at <= close {
        let new = fx(at);
        if new == 0.0 {
            zeroes.push(at);
        } else if new * last < 0.0 {
            guesses.push((2.0 * at - 0.0625) / 2.0);
        } else if dx(at) * dx(at - 0.0625) < 0.0 {
            let extrema = find_extrema(at - 0.0625, at, dx, ddx);
            if fx(extrema).abs() < 1e-6 {
                guesses.push(extrema);
            }
        }
        last = new;
        at += 0.0625;
    }

    for guess in guesses.iter() {
        let zero = newtons_method(*guess, fx, dx);
        if zero >= open && zero <= close {
            zeroes.push(zero);
        }
    }
    zeroes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    zeroes.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
    zeroes
}
