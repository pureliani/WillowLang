use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedTypeX, CheckedType, TypeSpan},
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
            SymbolEntry::GenericStructDecl(decl) => CheckedType::GenericStructDecl(decl),
            SymbolEntry::StructDecl(decl) => CheckedType::StructDecl(decl),
            SymbolEntry::GenericTypeAliasDecl(decl) => CheckedType::GenericTypeAliasDecl(decl),
            SymbolEntry::TypeAliasDecl(decl) => CheckedType::TypeAliasDecl(decl),
            SymbolEntry::EnumDecl(decl) => CheckedType::Enum(decl),
            SymbolEntry::VarDecl(decl) => decl.constraint.kind,
            SymbolEntry::GenericParam(_) => {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotUseGenericParameterAsValue,
                    expr_span,
                ));

                CheckedType::Unknown
            }
        })
        .unwrap_or_else(|| {
            errors.push(SemanticError::new(
                SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                expr_span,
            ));

            CheckedType::Unknown
        });

    CheckedExpr {
        kind: CheckedExprKind::Identifier(id),
        ty: CheckedTypeX {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
    }
}
