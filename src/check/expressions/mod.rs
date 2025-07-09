pub mod check_access_expr;
pub mod check_and_expr;
pub mod check_arithmetic_negation_expr;
pub mod check_array_literal_expr;
pub mod check_binary_numeric_op;
pub mod check_codeblock_expr;
pub mod check_equality_expr;
pub mod check_fn_call_expr;
pub mod check_fn_expr;
pub mod check_generic_apply_expr;
pub mod check_identifier_expr;
pub mod check_if_expr;
pub mod check_inequality_expr;
pub mod check_logical_negation_expr;
pub mod check_numeric_expr;
pub mod check_or_expr;
pub mod check_static_access_expr;
pub mod check_struct_init_expr;
pub mod check_type_cast_expr;

use crate::{
    ast::{
        base::base_expression::{Expr, ExprKind},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind},
        },
    },
    check::SemanticChecker,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_expr(&mut self, expr: Expr) -> CheckedExpr {
        match expr.kind {
            ExprKind::Not { right } => self.check_logical_negation_expr(right, expr.span),
            ExprKind::Neg { right } => self.check_arithmetic_negation_expr(right, expr.span),
            ExprKind::Add { left, right } => {
                self.check_arithmetic_operation(left, right, expr.span, |left, right| CheckedExprKind::Add { left, right })
            }
            ExprKind::Subtract { left, right } => self.check_arithmetic_operation(left, right, expr.span, |left, right| {
                CheckedExprKind::Subtract { left, right }
            }),
            ExprKind::Multiply { left, right } => self.check_arithmetic_operation(left, right, expr.span, |left, right| {
                CheckedExprKind::Multiply { left, right }
            }),
            ExprKind::Divide { left, right } => {
                self.check_arithmetic_operation(left, right, expr.span, |left, right| CheckedExprKind::Divide { left, right })
            }
            ExprKind::Modulo { left, right } => {
                self.check_arithmetic_operation(left, right, expr.span, |left, right| CheckedExprKind::Modulo { left, right })
            }
            ExprKind::LessThan { left, right } => self.check_numeric_comparison(left, right, expr.span, |left, right| {
                CheckedExprKind::LessThan { left, right }
            }),
            ExprKind::LessThanOrEqual { left, right } => self.check_numeric_comparison(left, right, expr.span, |left, right| {
                CheckedExprKind::LessThanOrEqual { left, right }
            }),
            ExprKind::GreaterThan { left, right } => self.check_numeric_comparison(left, right, expr.span, |left, right| {
                CheckedExprKind::GreaterThan { left, right }
            }),
            ExprKind::GreaterThanOrEqual { left, right } => {
                self.check_numeric_comparison(left, right, expr.span, |left, right| CheckedExprKind::GreaterThanOrEqual {
                    left,
                    right,
                })
            }
            ExprKind::Equal { left, right } => self.check_equality_expr(left, right, expr.span),
            ExprKind::NotEqual { left, right } => self.check_inequality_expr(left, right, expr.span),
            ExprKind::And { left, right } => self.check_and_expr(left, right, expr.span),
            ExprKind::Or { left, right } => self.check_or_expr(left, right, expr.span),
            ExprKind::Access { left, field } => self.check_access_expr(left, field, expr.span),
            ExprKind::StaticAccess { left, field } => self.check_static_access_expr(left, field, expr.span),
            ExprKind::TypeCast { left, target } => self.check_type_cast_expr(left, target, expr.span),
            ExprKind::GenericApply { left, args } => self.check_generic_apply_expr(left, args, expr.span),
            ExprKind::FnCall { left, args } => self.check_fn_call_expr(left, args, expr.span),
            ExprKind::StructLiteral(fields) => self.check_struct_literal_expr(fields, expr.span),
            ExprKind::Null => CheckedExpr {
                kind: CheckedExprKind::Null,
                ty: Type {
                    kind: TypeKind::Null,
                    span: expr.span,
                },
            },
            ExprKind::BoolLiteral { value } => CheckedExpr {
                kind: CheckedExprKind::BoolLiteral { value },
                ty: Type {
                    kind: TypeKind::Bool,
                    span: expr.span,
                },
            },
            ExprKind::String(string_node) => CheckedExpr {
                kind: CheckedExprKind::String(string_node),
                ty: Type {
                    span: expr.span,
                    kind: TypeKind::Array {
                        item_type: Box::new(Type {
                            kind: TypeKind::Char,
                            span: expr.span, // TODO: come up with better span
                        }),
                        size: string_node.len,
                    },
                },
            },
            ExprKind::Number { value } => self.check_numeric_expr(value, expr.span),
            ExprKind::Identifier(id) => self.check_identifier_expr(id, expr.span),
            ExprKind::Fn {
                params,
                body,
                return_type,
                generic_params,
            } => self.check_fn_expr(params, body, return_type, generic_params, expr.span),
            ExprKind::If {
                condition,
                then_branch,
                else_if_branches,
                else_branch,
            } => self.check_if_expr(condition, then_branch, else_if_branches, else_branch, expr.span),
            ExprKind::ArrayLiteral { items } => self.check_array_literal_expr(items, expr.span),
            ExprKind::Block(codeblock) => {
                let (ty, checked_codeblock) = self.check_codeblock(codeblock);
                CheckedExpr {
                    ty,
                    kind: CheckedExprKind::Block(checked_codeblock),
                }
            }
        }
    }
}
