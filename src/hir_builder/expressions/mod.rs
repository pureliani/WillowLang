pub mod access;
pub mod and;
pub mod arithmetic;
pub mod bool_literal;
pub mod code_block;
pub mod comparison;
pub mod equality;
pub mod r#fn;
pub mod fn_call;
pub mod generic_apply;
pub mod identifier;
pub mod r#if;
pub mod list_literal;
pub mod r#match;
pub mod neg;
pub mod not;
pub mod number_literal;
pub mod or;
pub mod static_access;
pub mod string;
pub mod struct_literal;
pub mod type_cast;

use crate::{
    ast::expr::{Expr, ExprKind},
    cfg::{BinaryOperationKind, Value},
    hir_builder::HIRBuilder,
};

impl<'a> HIRBuilder<'a> {
    pub fn build_expr(&mut self, expr: Expr) -> Value {
        match expr.kind {
            ExprKind::Not { right } => self.build_not_expr(right),
            ExprKind::Neg { right } => todo!(),
            ExprKind::Add { left, right } => todo!(),
            ExprKind::Subtract { left, right } => todo!(),
            ExprKind::Multiply { left, right } => todo!(),
            ExprKind::Divide { left, right } => todo!(),
            ExprKind::Modulo { left, right } => todo!(),
            ExprKind::LessThan { left, right } => self.build_comparison_expr(left, right, BinaryOperationKind::LessThan),
            ExprKind::LessThanOrEqual { left, right } => {
                self.build_comparison_expr(left, right, BinaryOperationKind::LessThanOrEqual)
            }
            ExprKind::GreaterThan { left, right } => self.build_comparison_expr(left, right, BinaryOperationKind::GreaterThan),
            ExprKind::GreaterThanOrEqual { left, right } => {
                self.build_comparison_expr(left, right, BinaryOperationKind::GreaterThanOrEqual)
            }
            ExprKind::Equal { left, right } => self.build_equality_expr(left, right, BinaryOperationKind::Equal),
            ExprKind::NotEqual { left, right } => self.build_equality_expr(left, right, BinaryOperationKind::NotEqual),
            ExprKind::And { left, right } => self.build_and_expr(left, right),
            ExprKind::Or { left, right } => self.build_or_expr(left, right),
            ExprKind::Access { left, field } => todo!(),
            ExprKind::StaticAccess { left, field } => todo!(),
            ExprKind::TypeCast { left, target } => todo!(),
            ExprKind::FnCall { left, args } => todo!(),
            ExprKind::StructLiteral { fields } => todo!(),
            ExprKind::BoolLiteral { value } => self.build_bool_literal(value),
            ExprKind::Number { value } => self.build_number_literal(value),
            ExprKind::String { value } => todo!(),
            ExprKind::Identifier { identifier } => todo!(),
            ExprKind::Fn {
                params,
                body,
                return_type,
                name,
            } => todo!(),
            ExprKind::If {
                condition,
                then_branch,
                else_if_branches,
                else_branch,
            } => todo!(),
            ExprKind::ListLiteral { items } => todo!(),
            ExprKind::CodeBlock(block_contents) => todo!(),
            ExprKind::Tag { identifier, value } => todo!(),
            ExprKind::Match { condition, arms } => todo!(),
        }
    }
}
