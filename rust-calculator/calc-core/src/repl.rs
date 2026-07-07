// calc-core/src/repl.rs (Claude generated)
use alloc::format;

use crate::gauss_kronrod::integrate;
use crate::io::LineIo;
use crate::parser::{parse, tree};
use alloc::string::String;
/*
use crate::differentiate::differentiate;
use crate::simplify::simplify;*/
use crate::evaluate::eval;
use crate::solver::on_interval;

// ai generated because I didn't want to bother with a display function over logic
fn format_sig(v: f64, digits: usize) -> String {
    if v == 0.0 || !v.is_finite() {
        return format!("{}", v);
    }
    // snap near-zero artifacts to 0, like a calculator display
    if v.abs() < 1e-10 {
        return "0".to_string();
    }
    let magnitude = libm::floor(libm::log10(v.abs())) as i32;

    // switch to scientific notation outside a comfortable fixed range
    if magnitude >= digits as i32 || magnitude < -4 {
        let s = format!("{:.*e}", digits - 1, v);
        // Rust prints "1.234e-16"; trim zeros in the mantissa
        if let Some(epos) = s.find('e') {
            let (mant, exp) = s.split_at(epos);
            let mant = mant.trim_end_matches('0').trim_end_matches('.');
            return format!("{}{}", mant, exp);
        }
        return s;
    }

    let decimals = (digits as i32 - 1 - magnitude).max(0) as usize;
    let rounded = format!("{:.*}", decimals, v);
    rounded
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub fn run_repl<IO: LineIo>(io: &mut IO) -> ! {
    loop {
        io.write_str("Enter expression: "); // no newline here now

        let calc = io.read_line();
        let calc = calc.trim();

        if calc.is_empty() {
            continue;
        }

        let result = parse(calc);
        io.write_line(&format!("{:?}", result));
        io.write_line(&format!("{:#?}", tree(&result)));
        io.write_line(&format!("{}",
            integrate(&tree(&result), -2.0, 2.0)
        ));
    }
}
