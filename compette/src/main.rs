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
        let context = Context::create();
        let mut cgns = Vec::new();
        let mut obj_paths = Vec::new();

        for path in parsed_args.paths {
            let contents = fs::read_to_string(&path).unwrap();

            let lexer = Lexer::new(&contents).tokenize();
            if parsed_args.parser_opts.print_lexer_ouput {
                println!("{:#?}", lexer);
            }

            let mut parser = Parser::new(lexer).parse().unwrap();

            let mut pass_manager = PassManager::new(&mut parser);
            if parsed_args.opt {
                pass_manager.invoke();
            }

            if parsed_args.parser_opts.print_ast_output {
                println!("{:#?}", parser);
            }

            cgns.push(CodeGen::new(&context, parser));
            let codegen = cgns.last_mut().unwrap();
            codegen.codegen().unwrap();

            if !parsed_args.compiler_opts.jit {
                let obj_path = codegen.generate_obj(path.into(), parsed_args.emit_llvm);
                obj_paths.push(obj_path.unwrap());
            }
        }

        parsed_args
            .compiler_opts
            .jit
            .then(|| {
                let exit_code = run_with_jit(&cgns);
                println!("Exit Code: {}", exit_code.unwrap());
            })
            .or_else(|| {
                let output = build(&obj_paths).unwrap();
                parsed_args.compiler_opts.run.then(|| run(output));
                Some(())
            });
    }
}
