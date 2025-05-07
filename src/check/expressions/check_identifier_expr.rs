use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind, TypeSpan},
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
            SymbolEntry::GenericStructDecl(decl) => TypeKind::GenericStructDecl(decl),
            SymbolEntry::StructDecl(decl) => TypeKind::StructDecl(decl),
            SymbolEntry::GenericTypeAliasDecl(decl) => TypeKind::GenericTypeAliasDecl(decl),
            SymbolEntry::TypeAliasDecl(decl) => TypeKind::TypeAliasDecl(decl),
            SymbolEntry::EnumDecl(decl) => TypeKind::Enum(decl),
            SymbolEntry::VarDecl(decl) => decl.constraint.kind,
            SymbolEntry::GenericParam(_) => {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotUseGenericParameterAsValue,
                    expr_span,
                ));

                TypeKind::Unknown
            }
        })
        .unwrap_or_else(|| {
            errors.push(SemanticError::new(
                SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                expr_span,
            ));

            TypeKind::Unknown
        });

    CheckedExpr {
        kind: CheckedExprKind::Identifier(id),
        expr_type: Type {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
