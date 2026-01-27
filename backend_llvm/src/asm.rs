use inkwell::values::BasicValueEnum;

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_inline_asm(
        &self,
        asm: String,
        constraints: String,
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
            .build_indirect_call(void_fn_ty, inline_asm, &[], "");

        call.map(|c| c.try_as_basic_value().left())
            .map_err(|e| CodeGenError::from_llvm_err(e))
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
}
