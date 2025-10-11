use crate::{
    ast::{IdentifierNode, Span, StringNode},
    hir::{
        errors::{SemanticError, SemanticErrorKind},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_from_stmt(
        &mut self,
        ctx: &mut HIRContext,
        path: StringNode,
        identifiers: Vec<(IdentifierNode, Option<IdentifierNode>)>,
        span: Span,
    ) {
        if !ctx.module_builder.is_file_scope() {
            ctx.module_builder.errors.push(SemanticError {
                kind: SemanticErrorKind::FromStatementMustBeDeclaredAtFileLevel,
                span,
            });
            return;
        }

        let current_module_path_str =
            ctx.program_builder.string_interner.resolve(path.value);
        let target_module_path =
            ctx.module_builder.module.path.join(current_module_path_str);

        let module = ctx.program_builder.modules.get(&target_module_path);

        match module {
            Some(m) => {
                let target_module_exports = m.module.exports.get(todo!());
            }
            None => {
                ctx.module_builder.errors.push(SemanticError {
                    kind: SemanticErrorKind::ModuleNotFound(target_module_path.clone()),
                    span: path.span,
                });
                return;
            }
        }
    }
}
