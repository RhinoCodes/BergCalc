// calc-core/src/lib.rs (Claude generated)
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod expr;
pub mod parser;
pub mod evaluate;
pub mod differentiate;
pub mod simplify;
pub mod io; 
pub mod repl;
pub mod solver;
pub mod gauss_kronrod;

pub use repl::run_repl;