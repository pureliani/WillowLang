use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_declaration::CheckedVarDecl,
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
    let expr_type = scope
        .borrow()
        .lookup(&id.name)
        .map(|entry| match entry {
            SymbolEntry::StructDecl(decl) => Type {
                kind: TypeKind::Struct(decl),
                span: TypeSpan::Expr(expr_span),
            },
            SymbolEntry::TypeAliasDecl(decl) => Type {
                kind: TypeKind::TypeAlias(decl),
                span: TypeSpan::Expr(expr_span),
            },
            SymbolEntry::EnumDecl(decl) => Type {
                kind: TypeKind::Enum(decl),
                span: TypeSpan::Expr(expr_span),
            },
            SymbolEntry::GenericParam(_) => {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotUseGenericParameterAsValue,
                    expr_span,
                ));

                Type {
                    kind: TypeKind::Unknown,
                    span: TypeSpan::Expr(expr_span),
                }
            }
            SymbolEntry::VarDecl(CheckedVarDecl {
                identifier,
                documentation,
                constraint,
                value,
            }) => Type {
                kind: constraint.kind,
                span: TypeSpan::Expr(expr_span),
            },
        })
        .unwrap_or_else(|| {
            errors.push(SemanticError::new(
                SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                expr_span,
            ));

            Type {
                kind: TypeKind::Unknown,
                span: TypeSpan::Expr(expr_span),
            }
        });

    CheckedExpr {
        kind: CheckedExprKind::Identifier(id),
        expr_type,
    }
}
