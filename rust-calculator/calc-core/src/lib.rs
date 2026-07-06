// calc-core/src/lib.rs (Claude generated)
#![no_std]
extern crate alloc;

pub mod expr;
pub mod parser;
pub mod evaluate;
pub mod differentiate;
pub mod simplify;
pub mod io; 
pub mod repl;

pub use repl::run_repl;