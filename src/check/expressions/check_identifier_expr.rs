use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        IdentifierNode, Span,
    },
    check::{
        scope::{Scope, SymbolEntry},
        SemanticChecker, SemanticError, SemanticErrorKind,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_identifier_expr(
        &mut self,
        id: IdentifierNode,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let ty = scope
            .borrow()
            .lookup(id.name)
            .map(|entry| match entry {
                SymbolEntry::StructDecl(decl) => CheckedType::StructDecl(decl),
                SymbolEntry::TypeAliasDecl(decl) => CheckedType::TypeAliasDecl(decl),
                SymbolEntry::EnumDecl(decl) => CheckedType::EnumDecl(decl),
                SymbolEntry::VarDecl(decl) => decl.constraint,
                SymbolEntry::GenericParam(_) => {
                    self.errors.push(SemanticError {
                        kind: SemanticErrorKind::CannotUseGenericParameterAsValue,
                        span,
                    });

                    CheckedType::Unknown
                }
            })
            .unwrap_or_else(|| {
                self.errors.push(SemanticError {
                    kind: SemanticErrorKind::UndeclaredIdentifier(id),
                    span,
                });

                CheckedType::Unknown
            });

        CheckedExpr {
            ty,
            span,
            kind: CheckedExprKind::Identifier(id),
        }
    }
}
