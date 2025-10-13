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
                kind: SemanticErrorKind::FromStatementMustBeDeclaredAtTopLevel,
                span,
            });
            return;
        }

        let mut target_path = ctx.module_builder.module.path.clone();
        target_path.pop();
        target_path.push(path.value);

        let canonical_path = target_path.canonicalize().unwrap();

        let module = ctx.program_builder.modules.get(&canonical_path);

        match module {
            Some(m) => {
                let target_module_exports = m.module.exports.get(todo!());
            }
            None => {
                ctx.module_builder.errors.push(SemanticError {
                    kind: SemanticErrorKind::ModuleNotFound(target_path),
                    span: path.span,
                });
                return;
            }
        }
    }
}
