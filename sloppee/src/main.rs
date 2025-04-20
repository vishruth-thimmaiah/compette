use std::{env::args, fs};

use backend_llvm::CodeGen;
use build::{build, run};
use inkwell::context::Context;
use lexer::lexer::Lexer;
use parser::Parser;

mod args;
mod build;

fn main() {
    let args: Vec<String> = args().collect();
    let parsed_args = args::parse_args(&args);

    if args.len() > 1 {
        let path = parsed_args.path.unwrap();
        let contents = fs::read_to_string(&path).unwrap();

        let lexer = Lexer::new(&contents).tokenize();
        if parsed_args.parser_opts.print_lexer_ouput {
            println!("{:#?}", lexer);
        }

        let context = Context::create();

        let parser = Parser::new(lexer.clone()).parse().unwrap();

        if parsed_args.parser_opts.print_ast_output {
            println!("{:#?}", parser);
        }

        let codegen = CodeGen::new(&context, parser, parsed_args.compiler_opts.jit);
        codegen.codegen().unwrap();

        parsed_args
            .compiler_opts
            .jit
            .then(|| {
                let exit_code = codegen.run_with_jit();
                println!("Exit Code: {}", exit_code.unwrap());
            })
            .or_else(|| {
                let ir = codegen.ir_as_string();
                let output = build(path.into(), ir).unwrap();
                parsed_args.compiler_opts.run.then(|| run(output))
            });
    }
}
