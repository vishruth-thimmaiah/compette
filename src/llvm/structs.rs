use crate::parser::nodes::StructParserNode;

use super::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn def_struct(&self, node: &StructParserNode) {
        let a = self.context.opaque_struct_type(&node.struct_name);
        let fields = node
            .fields
            .values()
            .map(|dt| self.def_expr(dt).unwrap())
            .collect::<Vec<_>>();
        a.set_body(&fields, false);
    }
}
