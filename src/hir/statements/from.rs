use crate::{
    ast::{IdentifierNode, Span, StringNode},
    hir::{
        errors::{SemanticError, SemanticErrorKind},
        HIRContext,
    },
};

pub fn build_from_stmt(
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

    let canonical_path = match target_path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            ctx.module_builder.errors.push(SemanticError {
                kind: SemanticErrorKind::ModuleNotFound(target_path),
                span: path.span,
            });
            return;
        }
    };

    let target_module = ctx.program_builder.modules.get(&canonical_path);

    match target_module {
        Some(m) => {
            for (imported_ident, alias) in identifiers {
                if let Some(decl_id) = m.resolve_export(imported_ident.name) {
                    let name_in_current_scope = alias.unwrap_or(imported_ident);

                    ctx.module_builder.scope_map(name_in_current_scope, decl_id);
                } else {
                    ctx.module_builder.errors.push(SemanticError {
                        kind: SemanticErrorKind::SymbolNotExported {
                            module_path: canonical_path.clone(),
                            symbol: imported_ident,
                        },
                        span: imported_ident.span,
                    });
                    continue;
                }
            }
        }
        None => {
            ctx.module_builder.errors.push(SemanticError {
                kind: SemanticErrorKind::ModuleNotFound(canonical_path),
                span: path.span,
            });
        }
    }
}
