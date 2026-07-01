mod decimal;

use decimal::Decimal;

fn nthroot(x: &str, n: i32) -> Decimal {
    let f: f64 = x.parse().unwrap();
    let mut guess = Decimal::from_str(&f.powf(1.0 / n as f64).to_string());
    println!("guess = {}", guess);
    let n_dec = Decimal::from_i32(n);
    let mut last_correction = Decimal::new();
    let x_dec = Decimal::from_str(x);
    for i in 0..20 {
        let correction = (guess.pow(n) - x_dec) / (n_dec * guess.pow(n-1));
        if correction == last_correction {
            break;
        }
        last_correction = correction;
        guess = guess - correction;
    }
    guess
}

fn main() {
    let x = Decimal::from_str("1.4142135623730950488016887242095");
    let y = Decimal::from_str("9");
    println!("{}", nthroot("9",4));
}
