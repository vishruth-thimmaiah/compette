pub struct Args {
    pub print_lexer_ouput: bool,
    pub print_ast_output: bool,
    pub use_jit: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            print_lexer_ouput: false,
            print_ast_output: false,
            use_jit: false,
        }
    }
}

pub fn parse_args(args: Vec<String>) -> Args {
    let mut result = Args::default();
    if args.len() < 3 {
        return result;
    }
    for arg in args.get(2..).unwrap() {
        match arg.as_str() {
            "--print-lexer-output" => result.print_lexer_ouput = true,
            "--print-ast-output" => result.print_ast_output = true,
            "--use-jit" => result.use_jit = true,
            _ => (),
        }

    }
    result
}
