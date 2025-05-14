use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    base::base_expression::{Expr, ExprKind},
    checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_type::CheckedType,
    },
};

use super::{
    expressions::{
        check_access_expr::check_access_expr, check_addition_expr::check_addition_expr,
        check_and_expr::check_and_expr,
        check_arithmetic_negation_expr::check_arithmetic_negation_expr,
        check_array_literal_expr::check_array_literal_expr,
        check_codeblock_expr::check_codeblock_expr, check_division_expr::check_division_expr,
        check_equality_expr::check_equality_expr, check_fn_call_expr::check_fn_call_expr,
        check_fn_expr::check_fn_expr, check_generic_apply_expr::check_generic_apply_expr,
        check_greater_than_expr::check_greater_than_expr,
        check_greater_than_or_equal_expr::check_greater_than_or_equal_expr,
        check_identifier_expr::check_identifier_expr, check_if_expr::check_if_expr,
        check_inequality_expr::check_inequality_expr, check_is_type_expr::check_is_type_expr,
        check_less_than_expr::check_less_than_expr,
        check_less_than_or_equal_expr::check_less_than_or_equal_expr,
        check_logical_negation_expr::check_logical_negation_expr,
        check_modulo_expr::check_modulo_expr, check_multiplication_expr::check_multiplication_expr,
        check_numeric_expr::check_numeric_expr, check_or_expr::check_or_expr,
        check_static_access_expr::check_static_access_expr,
        check_struct_init_expr::check_struct_init_expr,
        check_subtraction_expr::check_subtraction_expr, check_type_cast_expr::check_type_cast_expr,
    },
    scope::Scope,
    SemanticError,
};

pub fn check_expr(
    expr: Expr,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    match expr.kind {
        ExprKind::Not { right } => check_logical_negation_expr(right, expr.span, errors, scope),
        ExprKind::Neg { right } => check_arithmetic_negation_expr(right, expr.span, errors, scope),
        ExprKind::Add { left, right } => check_addition_expr(left, right, errors, scope),
        ExprKind::Subtract { left, right } => check_subtraction_expr(left, right, errors, scope),
        ExprKind::Multiply { left, right } => check_multiplication_expr(left, right, errors, scope),
        ExprKind::Divide { left, right } => check_division_expr(left, right, errors, scope),
        ExprKind::Modulo { left, right } => check_modulo_expr(left, right, errors, scope),
        ExprKind::LessThan { left, right } => check_less_than_expr(left, right, errors, scope),
        ExprKind::LessThanOrEqual { left, right } => {
            check_less_than_or_equal_expr(left, right, errors, scope)
        }
        ExprKind::GreaterThan { left, right } => {
            check_greater_than_expr(left, right, errors, scope)
        }
        ExprKind::GreaterThanOrEqual { left, right } => {
            check_greater_than_or_equal_expr(left, right, errors, scope)
        }
        ExprKind::Equal { left, right } => check_equality_expr(left, right, errors, scope),
        ExprKind::NotEqual { left, right } => check_inequality_expr(left, right, errors, scope),
        ExprKind::And { left, right } => check_and_expr(left, right, errors, scope),
        ExprKind::Or { left, right } => check_or_expr(left, right, errors, scope),
        ExprKind::Access { left, field } => check_access_expr(left, field, errors, scope),
        ExprKind::StaticAccess { left, field } => {
            check_static_access_expr(left, field, errors, scope)
        }
        ExprKind::TypeCast { left, target } => check_type_cast_expr(left, target, errors, scope),
        ExprKind::IsType { left, target } => {
            check_is_type_expr(left, target, expr.span, errors, scope)
        }
        ExprKind::GenericApply { left, args } => {
            check_generic_apply_expr(left, args, expr.span, errors, scope)
        }
        ExprKind::FnCall { left, args } => check_fn_call_expr(left, args, expr.span, errors, scope),
        ExprKind::StructInit { left, fields } => {
            check_struct_init_expr(left, fields, errors, scope)
        }
        ExprKind::Null => CheckedExpr {
            span: expr.span,
            kind: CheckedExprKind::Null,
            ty: CheckedType::Null,
        },
        ExprKind::BoolLiteral { value } => CheckedExpr {
            span: expr.span,
            kind: CheckedExprKind::BoolLiteral { value },
            ty: CheckedType::Bool,
        },
        ExprKind::String(string_node) => {
            let size = string_node.value.len();

            CheckedExpr {
                span: expr.span,
                kind: CheckedExprKind::String(string_node),
                ty: CheckedType::Array {
                    item_type: Box::new(CheckedType::Char),
                    size,
                },
            }
        }
        ExprKind::Number { value } => check_numeric_expr(value, expr.span),
        ExprKind::Identifier(id) => check_identifier_expr(id, expr.span, errors, scope),
        ExprKind::Fn {
            params,
            body,
            return_type,
            generic_params,
        } => check_fn_expr(
            params,
            body,
            return_type,
            generic_params,
            expr.span,
            errors,
            scope,
        ),
        ExprKind::If {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
        } => check_if_expr(
            condition,
            then_branch,
            else_if_branches,
            else_branch,
            expr.span,
            errors,
            scope,
        ),
        ExprKind::ArrayLiteral { items } => {
            check_array_literal_expr(items, expr.span, errors, scope)
        }
        ExprKind::Block(block_contents) => {
            check_codeblock_expr(block_contents, expr.span, errors, scope)
        }
        ExprKind::Error(parsing_error) => todo!(),
    }
}
