use inkwell::{
    AddressSpace,
    types::BasicType,
    values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue},
};
use parser::nodes::FunctionCall;

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_inline_asm(
        &self,
        asm: String,
        constraints: String,
        args: &[BasicMetadataValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, CodeGenError> {
        let void_fn_ty = self.context.void_type().fn_type(&[], false);
        let inline_asm = self.context.create_inline_asm(
            void_fn_ty,
            asm,
            constraints,
            true,
            false,
            Some(inkwell::InlineAsmDialect::Intel),
            false,
        );

        let call = self
            .builder
            .build_indirect_call(void_fn_ty, inline_asm, &args, "");

        call.map(|c| c.try_as_basic_value().left())
            .map_err(|e| CodeGenError::from_llvm_err(e))
    }

    pub(crate) fn impl_inline_asm_call(
        &self,
        built_func: FunctionValue<'ctx>,
        func_call: &FunctionCall,
    ) -> Result<Option<BasicValueEnum<'ctx>>, CodeGenError> {
        let asm = self.impl_const_string(&func_call.args[0])?;
        let constraints = self.impl_const_string(&func_call.args[1])?;

        let mut args: Vec<BasicMetadataValueEnum<'ctx>> = Vec::new();
        for arg in &func_call.args[2..] {
            let expr = self
                .impl_expr(
                    arg,
                    built_func,
                    self.context
                        .ptr_type(AddressSpace::default())
                        .as_basic_type_enum(),
                )
                .unwrap();
            args.push(expr.into());
        }

        return self.impl_inline_asm(asm, constraints, &args);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_codegen_asm() {
        let data = "func main() { inline_asm(\"nop\", \"\") }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define void @main() {
entry:
  call void asm sideeffect inteldialect "nop", ""()
  ret void
}
"#
        )
    }

    #[test]
    fn test_codegen_asm_with_const() {
        let data = "
let string message = \"Test\"
func main() {
    inline_asm(\"mov rax, 1
    mov rdi, 1
    lea rsi, message[rip]
    mov rdx, 4
    syscall
    mov rax, 60
    xor rdi, rdi
    syscall\",
\"~{dirflag},~{fpsr},~{flags}\")

}";
        let result = crate::get_codegen_for_string(data).unwrap();

        println!("{}", result);

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

@message = global [4 x i8] c"Test"

define void @main() {
entry:
  call void asm sideeffect inteldialect "mov rax, 1\0A    mov rdi, 1\0A    lea rsi, message[rip]\0A    mov rdx, 4\0A    syscall\0A    mov rax, 60\0A    xor rdi, rdi\0A    syscall", "~{dirflag},~{fpsr},~{flags}"()
  ret void
}
"#
        )
    }

    #[test]
    fn test_codegen_asm_with_const_pointer() {
        let data = "
let string message = \"Test\"
func main() {
    inline_asm(\"mov rax, 1
    mov rdi, 1
    mov rsi, $0
    mov rdx, $1
    syscall
    mov rax, 60
    xor rdi, rdi
    syscall\",
    \"r,~{rax},~{rdi},~{rsi},~{rdx},~{rcx},~{r11},~{memory},~{dirflag},~{fpsr},~{flags}\", message, 4)

}";
        let result = crate::get_codegen_for_string(data).unwrap();

        println!("{}", result);

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

@message = global [4 x i8] c"Test"

define void @main() {
entry:
  call void asm sideeffect inteldialect "mov rax, 1\0A    mov rdi, 1\0A    mov rsi, $0\0A    mov rdx, $1\0A    syscall\0A    mov rax, 60\0A    xor rdi, rdi\0A    syscall", "r,~{rax},~{rdi},~{rsi},~{rdx},~{rcx},~{r11},~{memory},~{dirflag},~{fpsr},~{flags}"(ptr @message, i64 4)
  ret void
}
"#
        )
    }
}
