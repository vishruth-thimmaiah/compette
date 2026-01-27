use std::{env::args, fs};

use backend_llvm::{CodeGen, run_with_jit};
use build::{build, run};
use inkwell::context::Context;
use lexer::lexer::Lexer;
use parser::Parser;
use passes::r#impl::PassManager;

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

        let mut parser = Parser::new(lexer).parse().unwrap();

        let mut pass_manager = PassManager::new(&mut parser);
        if parsed_args.opt {
            pass_manager.invoke();
        }

        if parsed_args.parser_opts.print_ast_output {
            println!("{:#?}", parser);
        }

        let codegen = CodeGen::new(&context, parser);
        codegen.codegen().unwrap();

        parsed_args
            .compiler_opts
            .jit
            .then(|| {
                // let exit_code = codegen.run_with_jit();
                let exit_code = run_with_jit(&codegen.module);
                println!("Exit Code: {}", exit_code.unwrap());
            })
            .or_else(|| {
                let obj_path = codegen.generate_obj(path.into(), parsed_args.emit_llvm);
                let output = build(obj_path.unwrap()).unwrap();
                parsed_args.compiler_opts.run.then(|| run(output));
                Some(())
            });
    }
}
