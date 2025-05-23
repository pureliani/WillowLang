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
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_identifier_expr(
    id: IdentifierNode,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let ty = scope
        .borrow()
        .lookup(&id.name)
        .map(|entry| match entry {
            SymbolEntry::GenericStructDecl(decl) => CheckedType::GenericStructDecl(decl),
            SymbolEntry::StructDecl(decl) => CheckedType::StructDecl(decl),
            SymbolEntry::GenericTypeAliasDecl(decl) => CheckedType::GenericTypeAliasDecl(decl),
            SymbolEntry::TypeAliasDecl(decl) => CheckedType::TypeAliasDecl(decl),
            SymbolEntry::EnumDecl(decl) => CheckedType::EnumDecl(decl),
            SymbolEntry::VarDecl(decl) => decl.constraint,
            SymbolEntry::GenericParam(_) => {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotUseGenericParameterAsValue,
                    span,
                ));

                CheckedType::Unknown
            }
        })
        .unwrap_or_else(|| {
            errors.push(SemanticError::new(
                SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                span,
            ));

            CheckedType::Unknown
        });

    CheckedExpr {
        ty,
        span,
        kind: CheckedExprKind::Identifier(id),
    }
}
