use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        IdentifierNode, Span,
    },
    check::{SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_access_expr(&mut self, left: Box<Expr>, field: IdentifierNode, span: Span) -> CheckedExpr {
        let checked_left = self.check_expr(*left);

        let expr_type = match &checked_left.ty.kind {
            CheckedTypeKind::Struct(fields) => fields
                .iter()
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
