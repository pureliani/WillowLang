use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
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
                SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl.clone()),
                SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl.clone()),
                SymbolEntry::EnumDecl(decl) => CheckedTypeKind::EnumDecl(decl.clone()),
                SymbolEntry::VarDecl(decl) => self.get_current_type_of_var(&decl),
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
