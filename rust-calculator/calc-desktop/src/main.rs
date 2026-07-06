// (Claude generated)
use std::io::{self, Write};
struct StdIo;
impl calc_core::io::LineIo for StdIo {
    fn read_line(&mut self) -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s
    }
    fn write_line(&mut self, s: &str) {
        println!("{}", s);
    }
    fn write_str(&mut self, s: &str) {
        print!("{}", s);
        std::io::stdout().flush().unwrap(); // needed since stdout is line-buffered
    }
}

fn main() {
    let mut io = StdIo;
    calc_core::run_repl(&mut io);
}