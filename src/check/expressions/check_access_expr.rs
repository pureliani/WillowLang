use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::CheckedStructDecl,
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        IdentifierNode, Span,
    },
    check::{scope::Scope, SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_access_expr(
        &mut self,
        left: Box<Expr>,
        field: IdentifierNode,
        span: Span,
        scope: Rc<RefCell<Scope>>,
    ) -> CheckedExpr {
        let checked_left = self.check_expr(*left, scope);

        let expr_type = match &checked_left.ty.kind {
            // TODO: Add enum declaration handler
            CheckedTypeKind::StructDecl(CheckedStructDecl { fields, .. }) => fields
                .into_iter()
                .find(|p| p.identifier == field)
                .map(|p| p.constraint.clone())
                .unwrap_or_else(|| {
                    self.errors.push(SemanticError::AccessToUndefinedField { field });

                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span,
                    }
                }),
            _ => {
                self.errors.push(SemanticError::CannotAccess {
                    target: checked_left.ty.clone(),
                });

                CheckedType {
                    kind: CheckedTypeKind::Unknown,
                    span,
                }
            }
        };

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::Access {
                left: Box::new(checked_left.clone()),
                field,
            },
        }
    }
}
