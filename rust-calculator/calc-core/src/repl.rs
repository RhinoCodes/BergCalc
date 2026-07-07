// calc-core/src/repl.rs (Claude generated)
use alloc::format;

use crate::io::LineIo;
use crate::parser::{parse, tree};
use crate::differentiate::differentiate;
use crate::simplify::simplify;
//use crate::evaluate::eval;

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
        io.write_line(&format!("{}", simplify(&differentiate(&tree(&result)))));
    }
}