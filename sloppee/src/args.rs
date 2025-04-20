use std::{path::Path, process::exit};

#[derive(Debug)]
pub struct Args {
    pub parser_opts: ParserArgs,
    pub compiler_opts: CodeGenArgs,
    pub path: Option<String>,
    pub dry_run: bool,
}

#[derive(Debug)]
pub struct ParserArgs {
    pub print_lexer_ouput: bool,
    pub print_ast_output: bool,
}

#[derive(Debug)]
pub struct CodeGenArgs {
    pub jit: bool,
    pub run: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            parser_opts: ParserArgs {
                print_lexer_ouput: false,
                print_ast_output: false,
            },
            compiler_opts: CodeGenArgs {
                jit: false,
                run: false,
            },
            path: None,
            dry_run: false,
        }
    }
}

const HELP_STRING: &str = r#"
Usage: sloppee [COMMAND] <file> [OPTIONS]

Commands:
    build                   Build the project
    run                     Run the project
    jit                     Run the project with LLVM's JIT

Options:
    --help, -h              Show this help message
    --print-lexer-output    Print the lexer output
    --print-ast-output      Print the ast output
    --dry-run               Run without invoking LLVM
"#;

fn show_help() {
    println!("{}", HELP_STRING);
    exit(0);
}

fn show_usage() {
    eprintln!("Usage: sloppee [COMMAND] <file> [OPTIONS]");
    exit(1);
}

pub fn parse_args(args: &Vec<String>) -> Args {
    let mut result = Args::default();

    match args.get(1) {
        Some(arg) => match arg.as_str() {
            "--help" | "-h" => show_help(),
            "build" => result.compiler_opts.run = false,
            "run" => result.compiler_opts.run = true,
            "jit" => result.compiler_opts.jit = true,
            _ => show_usage(),
        },
        None => show_usage(),
    }

    if let Some(path) = args.get(2) {
        if !Path::new(path).exists() {
            eprintln!("File does not exist");
            exit(1);
        }
        result.path = Some(path.to_string());
    } else {
        eprintln!("No file provided");
        exit(1);
    }

    if args.len() < 3 {
        return result;
    }
    for arg in args.get(2..).unwrap() {
        match arg.as_str() {
            "--help" | "-h" => show_help(),
            "--print-lexer-output" => result.parser_opts.print_lexer_ouput = true,
            "--print-ast-output" => result.parser_opts.print_ast_output = true,
            "--dry-run" => {
                if !result.compiler_opts.run {
                    result.dry_run = true
                } else {
                    eprintln!("--dry-run can only be used with build");
                    exit(1);
                }
            }
            _ => (),
        }
    }
    result
}
