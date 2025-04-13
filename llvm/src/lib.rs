use std::{backtrace::Backtrace, process::exit};

mod builder;
pub mod codegen;
mod expr;
mod flow_control;
mod func;
mod helpers;
mod operations;
mod stdlib_defs;
mod structs;
mod types;
mod variables;

pub fn compiler_error(msg: &str) -> ! {
    eprintln!("CompilerError");
    eprintln!("{}", msg);
    eprintln!("{}", Backtrace::capture());
    exit(1)
}
