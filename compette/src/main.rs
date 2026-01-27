use std::{env::args, fs, path::PathBuf};

use backend_llvm::{CodeGen, run_with_jit};
use build::{build, run};
use inkwell::context::Context;
use lexer::lexer::Lexer;
use parser::Parser;
use passes::r#impl::PassManager;

mod args;
mod build;

enum Output<'a> {
    Asm(Vec<PathBuf>),
    Jit(Vec<CodeGen<'a>>),
}

fn main() {
    let args: Vec<String> = args().collect();
    let parsed_args = args::parse_args(&args);

    if args.len() > 1 {
        let context = Context::create();
        let mut cgns = if parsed_args.compiler_opts.jit {
            Output::Jit(vec![])
        } else {
            Output::Asm(vec![])
        };

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

            let codegen = CodeGen::new(&context, parser, path.to_str().unwrap());
            codegen.codegen().unwrap();
            match cgns {
                Output::Asm(ref mut codegens) => {
                    codegens.push(
                        codegen
                            .generate_obj(path.into(), parsed_args.emit_llvm)
                            .unwrap(),
                    );
                }
                Output::Jit(ref mut codegens) => {
                    codegens.push(codegen);
                }
            }
        }

        match cgns {
            Output::Asm(codegens) => {
                let output = build(&codegens).unwrap();
                parsed_args.compiler_opts.run.then(|| run(output));
            }
            Output::Jit(codegens) => {
                let exit_code = run_with_jit(&codegens);
                println!("Exit Code: {}", exit_code.unwrap());
            }
        }
    }
}
