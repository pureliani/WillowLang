use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
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
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let type_kind = scope
        .borrow()
        .lookup(&id.name)
        .map(|entry| match entry {
            SymbolEntry::GenericStructDecl(decl) => CheckedTypeKind::GenericStructDecl(decl),
            SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl),
            SymbolEntry::GenericTypeAliasDecl(decl) => CheckedTypeKind::GenericTypeAliasDecl(decl),
            SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl),
            SymbolEntry::EnumDecl(decl) => CheckedTypeKind::Enum(decl),
            SymbolEntry::VarDecl(decl) => decl.constraint.kind,
            SymbolEntry::GenericParam(_) => {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotUseGenericParameterAsValue,
                    expr_span,
                ));

                CheckedTypeKind::Unknown
            }
        })
        .unwrap_or_else(|| {
            errors.push(SemanticError::new(
                SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                expr_span,
            ));

            CheckedTypeKind::Unknown
        });

    CheckedExpr {
        kind: CheckedExprKind::Identifier(id),
        expr_type: CheckedType {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
