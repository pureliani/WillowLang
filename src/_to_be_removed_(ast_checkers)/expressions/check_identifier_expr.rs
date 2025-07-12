use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind},
        },
        IdentifierNode, Span,
    },
    check::{utils::scope::SymbolEntry, SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_identifier_expr(&mut self, id: IdentifierNode, span: Span) -> CheckedExpr {
        let entry = self.scope_lookup(id.name);

        let kind = match entry {
            Some(e) => match e {
                SymbolEntry::TypeAliasDecl(decl) => TypeKind::TypeAliasDecl(decl.clone()),
                SymbolEntry::VarDecl(decl) => {
                    todo!()
                }
                SymbolEntry::GenericParam(_) => {
                    self.errors.push(SemanticError::CannotUseGenericParameterAsValue { span });

                    TypeKind::Unknown
                }
            },
            None => {
                self.errors.push(SemanticError::UndeclaredIdentifier { id });

                TypeKind::Unknown
            }
        };

        CheckedExpr {
            ty: Type { kind, span },
            kind: CheckedExprKind::Identifier(id),
        }
    }
}
