use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        IdentifierNode, Span,
    },
    check::{
        scope::{Scope, SymbolEntry},
        SemanticChecker, SemanticError,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_identifier_expr(&mut self, id: IdentifierNode, span: Span, scope: Rc<RefCell<Scope>>) -> CheckedExpr {
        let kind = scope
            .borrow()
            .lookup(id.name)
            .map(|entry| match entry {
                SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl),
                SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl),
                SymbolEntry::EnumDecl(decl) => CheckedTypeKind::EnumDecl(decl),
                SymbolEntry::VarDecl(decl) => decl.borrow().constraint.kind.clone(),
                SymbolEntry::GenericParam(_) => {
                    self.errors.push(SemanticError::CannotUseGenericParameterAsValue { span });

                    CheckedTypeKind::Unknown
                }
            })
            .unwrap_or_else(|| {
                self.errors.push(SemanticError::UndeclaredIdentifier { id });

                CheckedTypeKind::Unknown
            });

        CheckedExpr {
            ty: CheckedType { kind, span },
            kind: CheckedExprKind::Identifier(id),
        }
    }
}
