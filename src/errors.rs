use std::{backtrace::Backtrace, process::exit};

use crate::parser::parser::Parser;

pub fn parser_error(parser: &Parser, msg: &str) -> ! {
    eprintln!("ParserError");
    let current_token = parser.get_current_token();
    let final_msg = if msg.contains("invalid token") {
        format!("{} => {:?}", msg, current_token.r#type)
    } else {
        msg.to_string()
    };
    eprintln!(
        "Error at line: {}, column: {}\nError type: {}\n",
        current_token.line + 1,
        current_token.column + 1,
        final_msg
    );
    eprintln!("{}", Backtrace::capture());
    exit(1)
}

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

pub fn compiler_error(msg: &str) -> ! {
    eprintln!("CompilerError");
    eprintln!("{}", msg);
    eprintln!("{}", Backtrace::capture());
    exit(1)
}

