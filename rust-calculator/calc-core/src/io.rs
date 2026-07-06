// calc-core/src/io.rs (Claude generated)
use alloc::string::String;

pub trait LineIo {
    fn read_line(&mut self) -> String;
    fn write_line(&mut self, s: &str);
    fn write_str(&mut self, s: &str); // no trailing newline — for prompts
}