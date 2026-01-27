use std::{
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
};

use ext_defs::Resolver;
use inkwell::{
    OptimizationLevel,
    builder::{Builder, BuilderError},
    context::Context,
    execution_engine::{self, ExecutionEngine},
    module::Module,
    targets::InitializationConfig,
};
use parser::nodes::ASTNodes;
use stmt::Variables;
use structs::StructDefs;

mod asm;
mod block;
mod cond;
mod expr;
mod ext_defs;
mod func;
mod loops;
mod ops;
mod stmt;
mod structs;
mod utils;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub tokens: Vec<ASTNodes>,

    pub struct_defs: StructDefs<'ctx>,
    pub var_ptrs: Variables<'ctx>,
    pub import_resolver: Resolver<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, tokens: Vec<ASTNodes>, path: &str) -> Self {
        let builder = context.create_builder();
        let module = context.create_module(path);
        Self {
            context,
            builder,
            module,
            tokens,

            struct_defs: StructDefs::default(),
            var_ptrs: Variables::default(),
            import_resolver: Resolver::new(context),
        }
    }

    pub fn codegen(&self) -> Result<(), CodeGenError> {
        for node in self.tokens.iter() {
            match node {
                ASTNodes::Function(func) => {
                    self.impl_function_def(func)?;
                }
                ASTNodes::StructDef(st) => {
                    self.def_struct(st)?;
                }
                ASTNodes::ImportDef(imp) => {
                    self.import_resolver.resolve_import_def(imp)?;
                }
                ASTNodes::Extern(ext) => {
                    self.impl_extern_call(ext)?;
                }
                ASTNodes::LetStmt(let_stmt) => {
                    self.impl_global_let_stmt(let_stmt)?;
                }
                _ => unreachable!(),
            };
        }
        Ok(())
    }

    pub fn generate_ir_string(&self) -> String {
        return self.module.print_to_string().to_string();
    }

    pub fn generate_ir(&self, path: PathBuf) -> Result<(), CodeGenError> {
        self.module.print_to_file(&path).unwrap();
        Ok(())
    }

    pub fn generate_obj(&self, path: PathBuf, emit_llvm: bool) -> Result<PathBuf, CodeGenError> {
        let dir = PathBuf::from(".build/");
        if !dir.exists() {
            std::fs::create_dir(&dir).unwrap();
        }

        if emit_llvm {
            let ir_path = dir.join("ir/");
            if !ir_path.exists() {
                std::fs::create_dir(&ir_path).unwrap();
            }
            self.generate_ir(ir_path.join(path.file_stem().unwrap()).with_extension("ll"))
                .unwrap();
        }

        let mut obj_path = dir.join("obj/");
        if !obj_path.exists() {
            std::fs::create_dir_all(&obj_path).unwrap();
        }

        obj_path = obj_path.join(path.file_stem().unwrap()).with_extension("o");

        inkwell::targets::Target::initialize_all(&InitializationConfig::default());
        let triple = inkwell::targets::TargetMachine::get_default_triple();

        let target = inkwell::targets::Target::from_triple(&triple).unwrap();
        let machine = target
            .create_target_machine(
                &triple,
                "",
                "",
                inkwell::OptimizationLevel::Default,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .unwrap();

        machine
            .write_to_file(&self.module, inkwell::targets::FileType::Object, &obj_path)
            .unwrap();

        Ok(obj_path)
    }
}

pub fn run_with_jit(codegens: &[CodeGen]) -> Option<i32> {
    let modules = codegens.iter().map(|cg| &cg.module).collect::<Vec<_>>();
    let main_module = modules
        .iter()
        .find(|m| m.get_function("main").is_some())
        .unwrap_or_else(|| panic!("No main function found"));

    let execution_engine = main_module
        .create_jit_execution_engine(OptimizationLevel::Default)
        .unwrap();

    for codegen in codegens {
        if codegen.module.get_function("main").is_none() {
            execution_engine.add_module(&codegen.module).unwrap();
        }
    }

    let function = main_module.get_function("main").unwrap();
    let result = unsafe { execution_engine.run_function_as_main(function, &[]) };
    return Some(result);
}

#[derive(Debug)]
pub struct CodeGenError {
    msg: String,
}
impl Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeGenError: {}", self.msg)
    }
}
impl Error for CodeGenError {}

impl CodeGenError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    fn from_llvm_err(err: BuilderError) -> Self {
        Self {
            msg: err.to_string(),
        }
    }
}

#[cfg(test)]
pub(crate) fn get_codegen_for_string(code: &str) -> Result<String, CodeGenError> {
    use lexer::lexer::Lexer;
    use parser::Parser;

    let context = Context::create();
    let lexer = Lexer::new(code).tokenize();
    let parser = Parser::new(lexer).parse();
    if let Err(err) = parser {
        return Err(CodeGenError::new(&format!("Failed to parse: {}", err)));
    }
    let parser = parser.unwrap();
    let codegen = CodeGen::new(&context, parser, "main");
    codegen.codegen()?;
    Ok(codegen.generate_ir_string())
}
