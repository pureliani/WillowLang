pub mod check_access_expr;
pub mod check_addition_expr;
pub mod check_and_expr;
pub mod check_arithmetic_negation_expr;
pub mod check_array_literal_expr;
pub mod check_codeblock_expr;
pub mod check_division_expr;
pub mod check_equality_expr;
pub mod check_fn_call_expr;
pub mod check_fn_expr;
pub mod check_generic_apply_expr;
pub mod check_greater_than_expr;
pub mod check_greater_than_or_equal_expr;
pub mod check_identifier_expr;
pub mod check_if_expr;
pub mod check_inequality_expr;
pub mod check_is_type_expr;
pub mod check_less_than_expr;
pub mod check_less_than_or_equal_expr;
pub mod check_logical_negation_expr;
pub mod check_modulo_expr;
pub mod check_multiplication_expr;
pub mod check_numeric_expr;
pub mod check_or_expr;
pub mod check_static_access_expr;
pub mod check_struct_init_expr;
pub mod check_subtraction_expr;
pub mod check_type_cast_expr;

use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::{Expr, ExprKind},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::SemanticChecker,
};

use super::scope::Scope;

impl<'a> SemanticChecker<'a> {
    pub fn check_expr(&mut self, expr: Expr, scope: Rc<RefCell<Scope>>) -> CheckedExpr {
        match expr.kind {
            ExprKind::Not { right } => self.check_logical_negation_expr(right, expr.span, scope),
            ExprKind::Neg { right } => self.check_arithmetic_negation_expr(right, expr.span, scope),
            ExprKind::Add { left, right } => self.check_addition_expr(left, right, expr.span, scope),
            ExprKind::Subtract { left, right } => self.check_subtraction_expr(left, right, expr.span, scope),
            ExprKind::Multiply { left, right } => self.check_multiplication_expr(left, right, expr.span, scope),
            ExprKind::Divide { left, right } => self.check_division_expr(left, right, expr.span, scope),
            ExprKind::Modulo { left, right } => self.check_modulo_expr(left, right, expr.span, scope),
            ExprKind::LessThan { left, right } => self.check_less_than_expr(left, right, expr.span, scope),
            ExprKind::LessThanOrEqual { left, right } => self.check_less_than_or_equal_expr(left, right, expr.span, scope),
            ExprKind::GreaterThan { left, right } => self.check_greater_than_expr(left, right, expr.span, scope),
            ExprKind::GreaterThanOrEqual { left, right } => self.check_greater_than_or_equal_expr(left, right, expr.span, scope),
            ExprKind::Equal { left, right } => self.check_equality_expr(left, right, expr.span, scope),
            ExprKind::NotEqual { left, right } => self.check_inequality_expr(left, right, expr.span, scope),
            ExprKind::And { left, right } => self.check_and_expr(left, right, expr.span, scope),
            ExprKind::Or { left, right } => self.check_or_expr(left, right, expr.span, scope),
            ExprKind::Access { left, field } => self.check_access_expr(left, field, expr.span, scope),
            ExprKind::StaticAccess { left, field } => self.check_static_access_expr(left, field, expr.span, scope),
            ExprKind::TypeCast { left, target } => self.check_type_cast_expr(left, target, expr.span, scope),
            ExprKind::IsType { left, target } => self.check_is_type_expr(left, target, expr.span, scope),
            ExprKind::GenericApply { left, args } => self.check_generic_apply_expr(left, args, expr.span, scope),
            ExprKind::FnCall { left, args } => self.check_fn_call_expr(left, args, expr.span, scope),
            ExprKind::StructInit { left, fields } => self.check_struct_init_expr(left, fields, expr.span, scope),
            ExprKind::Null => CheckedExpr {
                kind: CheckedExprKind::Null,
                ty: CheckedType {
                    kind: CheckedTypeKind::Null,
                    span: expr.span,
                },
            },
            ExprKind::BoolLiteral { value } => CheckedExpr {
                kind: CheckedExprKind::BoolLiteral { value },
                ty: CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: expr.span,
                },
            },
            ExprKind::String(string_node) => CheckedExpr {
                kind: CheckedExprKind::String(string_node),
                ty: CheckedType {
                    span: expr.span,
                    kind: CheckedTypeKind::Array {
                        item_type: Box::new(CheckedType {
                            kind: CheckedTypeKind::Char,
                            span: expr.span, // TODO: come up with better span
                        }),
                        size: string_node.len,
                    },
                },
            },
            ExprKind::Number { value } => self.check_numeric_expr(value, expr.span),
            ExprKind::Identifier(id) => self.check_identifier_expr(id, expr.span, scope),
            ExprKind::Fn {
                params,
                body,
                return_type,
                generic_params,
            } => self.check_fn_expr(params, body, return_type, generic_params, expr.span, scope),
            ExprKind::If {
                condition,
                then_branch,
                else_if_branches,
                else_branch,
            } => self.check_if_expr(condition, then_branch, else_if_branches, else_branch, expr.span, scope),
            ExprKind::ArrayLiteral { items } => self.check_array_literal_expr(items, expr.span, scope),
            ExprKind::Block(block_contents) => self.check_codeblock_expr(block_contents, expr.span, scope),
        }
    }
}
