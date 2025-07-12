pub mod access;
pub mod and;
pub mod arithmetic;
pub mod array_literal;
pub mod bool_literal;
pub mod code_block;
pub mod comparison;
pub mod equality;
pub mod r#fn;
pub mod fn_call;
pub mod generic_apply;
pub mod identifier;
pub mod r#if;
pub mod neg;
pub mod not;
pub mod or;
pub mod static_access;
pub mod string;
pub mod struct_literal;
pub mod type_cast;

use crate::{
    ast::expr::{Expr, ExprKind},
    cfg::Value,
    hir_builder::HIRBuilder,
};

impl<'a> HIRBuilder<'a> {
    pub fn build_expr(&mut self, expr: Expr) -> Value {
        match expr.kind {
            ExprKind::Not { right } => todo!(),
            ExprKind::Neg { right } => todo!(),

            ExprKind::Add { left, right } => todo!(), // TODO: implement in arithmetic.rs
            ExprKind::Subtract { left, right } => todo!(), // TODO: implement in arithmetic.rs
            ExprKind::Multiply { left, right } => todo!(), // TODO: implement in arithmetic.rs
            ExprKind::Divide { left, right } => todo!(), // TODO: implement in arithmetic.rs
            ExprKind::Modulo { left, right } => todo!(), // TODO: implement in arithmetic.rs

            ExprKind::LessThan { left, right } => todo!(), // TODO: implement in comparison.rs
            ExprKind::LessThanOrEqual { left, right } => todo!(), // TODO: implement in comparison.rs
            ExprKind::GreaterThan { left, right } => todo!(), // TODO: implement in comparison.rs
            ExprKind::GreaterThanOrEqual { left, right } => todo!(), // TODO: implement in comparison.rs

            ExprKind::Equal { left, right } => todo!(), // TODO: implement in equality.rs
            ExprKind::NotEqual { left, right } => todo!(), // TODO: implement in equality.rs

            ExprKind::And { left, right } => todo!(),
            ExprKind::Or { left, right } => todo!(),
            ExprKind::Access { left, field } => todo!(),
            ExprKind::StaticAccess { left, field } => todo!(),
            ExprKind::TypeCast { left, target } => todo!(),
            ExprKind::GenericApply { left, args } => todo!(),
            ExprKind::FnCall { left, args } => todo!(),
            ExprKind::StructLiteral(items) => todo!(),
            ExprKind::BoolLiteral { value } => todo!(),
            ExprKind::Number { value } => todo!(),
            ExprKind::String(string_node) => todo!(),
            ExprKind::Identifier(identifier_node) => todo!(),
            ExprKind::Fn {
                params,
                body,
                return_type,
                generic_params,
            } => todo!(),
            ExprKind::If {
                condition,
                then_branch,
                else_if_branches,
                else_branch,
            } => todo!(),
            ExprKind::ArrayLiteral { items } => todo!(),
            ExprKind::CodeBlock(block_contents) => todo!(),
        }
    }
}
