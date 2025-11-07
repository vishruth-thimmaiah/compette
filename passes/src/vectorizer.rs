use parser::nodes::{ASTNodes, Datatype, Expression, ForLoop, Variable};

use crate::r#impl::{PassData, Variables};

const MAX_SIMD_SIZE: usize = 16;

pub(crate) fn vectorize(for_loop: &mut ForLoop, context: &PassData) {
    // TODO: for now, we only support for loops which use an increment.
    if for_loop.value.name != "_" || for_loop.increment.name == "_" {
        return;
    }

    let iter = if let Expression::Simple { left, .. } = &for_loop.iterator {
        if let ASTNodes::Variable(Variable { name }) = left.as_ref() {
            let var = context.get_vars(name).unwrap();
            let Datatype::NARRAY(ty, _) = var.get_dt() else {
                return;
            };
            if !ty.is_trivial() {
                return;
            }
            var
        } else {
            return;
        }
    } else {
        return;
    };

    let Datatype::NARRAY(dt, size) = iter.get_dt() else {
        unreachable!()
    };

    if dt.get_size() * size == MAX_SIMD_SIZE && size % 2 == 0 {
        try_vectorize_simple(for_loop, iter, context);
    }

    println!("vectorizing `for loop`");
}

fn try_vectorize_simple(
    for_loop: &mut ForLoop,
    expr: Variables,
    context: &PassData,
) -> Option<Variable> {
    let Datatype::NARRAY(dt, size) = expr.get_dt() else {
        unreachable!()
    };

    expr.set_dt(Datatype::SIMD(dt, size));
    return None;
}
