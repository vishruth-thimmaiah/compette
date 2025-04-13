use std::{backtrace::Backtrace, process::exit};

pub mod lexer;
mod tests;
pub mod types;

pub fn lexer_error(char: u8, msg: &str, line: usize, column: usize) -> ! {
    eprintln!("LexerError");
    eprintln!(
        "Error at line: {}, column: {}\nError type: {} => '{}' code: {}\n",
        line + 1,
        column + 1,
        msg,
        std::str::from_utf8(&[char]).unwrap_or("??"),
        char,
    );
    eprintln!("{}", Backtrace::capture());
    exit(1)
}

