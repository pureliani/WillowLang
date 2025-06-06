use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::CheckedTypeKind,
        },
        IdentifierNode, Span,
    },
    check::{
        scope::{Scope, SymbolEntry},
        SemanticChecker, SemanticError, SemanticErrorKind,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_identifier_expr(&mut self, id: IdentifierNode, span: Span, scope: Rc<RefCell<Scope>>) -> CheckedExpr {
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);

        let ty = scope
            .borrow()
            .lookup(id.name)
            .map(|entry| match entry {
                SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl { decl, node_id },
                SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl { decl, node_id },
                SymbolEntry::EnumDecl(decl) => CheckedTypeKind::EnumDecl { decl, node_id },
                SymbolEntry::VarDecl(decl) => decl.constraint,
                SymbolEntry::GenericParam(_) => {
                    self.errors.push(SemanticError {
                        kind: SemanticErrorKind::CannotUseGenericParameterAsValue,
                        span,
                    });

                    CheckedTypeKind::Unknown { node_id }
                }
            })
            .unwrap_or_else(|| {
                self.errors.push(SemanticError {
                    kind: SemanticErrorKind::UndeclaredIdentifier(id),
                    span,
                });

                CheckedTypeKind::Unknown { node_id }
            });

        CheckedExpr {
            ty,
            span,
            kind: CheckedExprKind::Identifier(id),
        }
    }
}
