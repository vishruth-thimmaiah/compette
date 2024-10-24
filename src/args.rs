use std::process::exit;

pub struct Args {
    pub print_lexer_ouput: bool,
    pub print_ast_output: bool,
    pub use_jit: bool,
    pub dry_run: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            print_lexer_ouput: false,
            print_ast_output: false,
            use_jit: false,
            dry_run: false,
        }
    }
}

const HELP_STRING: &str = r#"
Usage: sloppee <file> [OPTIONS]

Options:
    --help, -h              Show this help message
    --print-lexer-output    Print the lexer output
    --print-ast-output      Print the ast output
    --use-jit               Use LLVM's JIT
    --dry-run               Run without invoking LLVM
    "#;

fn show_help() {
    println!("{}", HELP_STRING);
    exit(0);
}

pub fn parse_args(args: &Vec<String>) -> Args {
    let mut result = Args::default();
    let first_arg = args.get(1);
    if  first_arg == Some(&"--help".to_string()) || first_arg == Some(&"-h".to_string()) {
        show_help();
    }
    if args.len() < 3 {
        return result;
    }
    for arg in args.get(2..).unwrap() {
        match arg.as_str() {
            "--help" | "-h" => show_help(),
            "--print-lexer-output" => result.print_lexer_ouput = true,
            "--print-ast-output" => result.print_ast_output = true,
            "--use-jit" => result.use_jit = true,
            "--dry-run" => result.dry_run = true,
            _ => (),
        }
    }
    result
}
