#[cfg(test)]
mod tests {
    use inkwell::context::Context;

    use crate::{lexer::lexer::Lexer, llvm::func::CodeGen, parser::parser::Parser};

    #[test]
    fn check_main_func() {
        let contents = r#"
        func main() u32 {
            let u32 a = 6 * 3 - 1
            return a
        }
        "#;

        let lexer = Lexer::new(&contents).tokenize();

        let parser = Parser::new(lexer.clone()).parse();

        let context = Context::create();
        let codegen = CodeGen::new(&context, parser);
        let result = codegen.jit_compile(false);
        assert_eq!(12, result.unwrap());
    }
}
