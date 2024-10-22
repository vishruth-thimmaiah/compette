use crate::{lexer::lexer::Lexer, llvm::func::CodeGen, parser::parser::Parser};
use inkwell::context::Context;

#[allow(dead_code)]
pub fn generate_result(contents: &str) -> Option<u32> {
    let lexer = Lexer::new(&contents).tokenize();

    let parser = Parser::new(lexer.clone()).parse();

    let context = Context::create();
    let codegen = CodeGen::new(&context, parser);
    codegen.jit_compile(false)
}

#[cfg(test)]
mod tests {
    use crate::llvm::tests::general::generate_result;

    #[test]
    fn check_main_func() {
        let contents = r#"
        func main() u32 {
            let u32 a = 6 * 3 - 1
            return a
        }
        "#;

        assert_eq!(12, generate_result(contents).unwrap());
    }

    #[test]
    fn check_mut() {
        let contents = r#"
        func main() u32 {
            let u32! a = 2
            if 5 < 6 {
                a = 4
            }
            return a
        }
        "#;

        assert_eq!(4, generate_result(contents).unwrap());
    }
}
