use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        DefinitionId, IdentifierNode, Span,
    },
    check::{utils::scope::SymbolEntry, SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_identifier_expr(&mut self, id: IdentifierNode, span: Span) -> CheckedExpr {
        let entry = self.scope_lookup(id.name);

        let kind = match entry {
            Some(e) => match e {
                SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl.clone()),
                SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl.clone()),
                SymbolEntry::EnumDecl(decl) => CheckedTypeKind::EnumDecl(decl.clone()),
                SymbolEntry::VarDecl(decl) => {
                    let target = decl.borrow();
                    let mut constraint_kind: CheckedTypeKind = decl.borrow().constraint.kind.clone();

                    if let Some(ctx) = self.tfg_contexts.last() {
                        if let Some(current_node) = ctx.graph.get_node(ctx.current_node) {
                            if let Some(kind) = self.get_definition_id_type(current_node.id, target.id) {
                                constraint_kind = kind
                            }
                        }
                    }

                    constraint_kind
                }
                SymbolEntry::GenericParam(_) => {
                    self.errors.push(SemanticError::CannotUseGenericParameterAsValue { span });

                    CheckedTypeKind::Unknown
                }
            },
            None => {
                self.errors.push(SemanticError::UndeclaredIdentifier { id });

                CheckedTypeKind::Unknown
            }
        };

        CheckedExpr {
            ty: CheckedType { kind, span },
            kind: CheckedExprKind::Identifier(id),
        }
    }
}
