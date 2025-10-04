pub mod access;
pub mod and;
pub mod array_literal;
pub mod binary_op;
pub mod bool_literal;
pub mod borrow;
pub mod codeblock;
pub mod r#fn;
pub mod fn_call;
pub mod identifier;
pub mod r#if;
pub mod r#match;
pub mod number_literal;
pub mod or;
pub mod static_access;
pub mod string;
pub mod struct_init;
pub mod typecast;
pub mod unary_op;

use crate::{
    ast::{
        expr::{Expr, ExprKind},
        Span,
    },
    hir::{
        cfg::{BinaryOperationKind, UnaryOperationKind, Value},
        expressions::r#if::IfContext,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_expr(&mut self, ctx: &mut HIRContext, expr: Expr) -> Value {
        match expr.kind {
            ExprKind::Not { right } => self.build_unary_op_expr(ctx, UnaryOperationKind::Not, right),
            ExprKind::Neg { right } => self.build_unary_op_expr(ctx, UnaryOperationKind::Neg, right),
            ExprKind::Add { left, right } => self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::Add),
            ExprKind::Subtract { left, right } => self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::Subtract),
            ExprKind::Multiply { left, right } => self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::Multiply),
            ExprKind::Divide { left, right } => self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::Divide),
            ExprKind::Modulo { left, right } => self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::Modulo),
            ExprKind::LessThan { left, right } => self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::LessThan),
            ExprKind::LessThanOrEqual { left, right } => {
                self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::LessThanOrEqual)
            }
            ExprKind::GreaterThan { left, right } => {
                self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::GreaterThan)
            }
            ExprKind::GreaterThanOrEqual { left, right } => {
                self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::GreaterThanOrEqual)
            }
            ExprKind::Equal { left, right } => self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::Equal),
            ExprKind::NotEqual { left, right } => self.build_binary_op_expr(ctx, left, right, BinaryOperationKind::NotEqual),
            ExprKind::And { left, right } => self.build_and_expr(ctx, left, right),
            ExprKind::Or { left, right } => self.build_or_expr(ctx, left, right),
            ExprKind::Access { left, field } => self.build_access_expr(ctx, left, field),
            ExprKind::StaticAccess { left, field } => todo!(),
            ExprKind::TypeCast { left, target } => self.build_typecast_expr(ctx, left, target),
            ExprKind::FnCall { left, args } => self.build_fn_call_expr(ctx, left, args, expr.span),
            ExprKind::BoolLiteral(value) => self.build_bool_literal(value),
            ExprKind::Number(value) => self.build_number_literal(value),
            ExprKind::String(value) => todo!(),
            ExprKind::Identifier(identifier) => self.build_identifier_expr(ctx, identifier),
            ExprKind::Fn {
                params,
                body,
                return_type,
                name,
            } => todo!(),
            ExprKind::If { branches, else_branch } => self.build_if(ctx, branches, else_branch, IfContext::Expression),
            ExprKind::ArrayLiteral(items) => self.build_array_literal_expr(ctx, items),
            ExprKind::CodeBlock(block_contents) => self.build_codeblock_expr(ctx, block_contents),
            ExprKind::Match { conditions, arms } => self.build_match_expr(ctx, conditions, arms),
            ExprKind::StructInit(fields) => self.build_struct_init_expr(ctx, fields, expr.span),
            ExprKind::Borrow { kind, value } => self.build_borrow_expr(ctx, kind, value),
        }
    }
}
