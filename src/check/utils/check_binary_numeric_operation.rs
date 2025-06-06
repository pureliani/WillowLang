use crate::{
    ast::{
        checked::{
            checked_expression::CheckedExpr,
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{SemanticChecker, SemanticError},
};

use super::{get_numeric_type_rank::get_numeric_type_rank, is_float::is_float, is_integer::is_integer, is_signed::is_signed};

impl<'a> SemanticChecker<'a> {
    pub fn check_binary_numeric_operation(&mut self, left: &CheckedExpr, right: &CheckedExpr, span: Span) -> CheckedType {
        let node_id = self.get_node_id();
        self.span_registry.insert_span(node_id, span);

        let left_type = if is_float(&left.ty.kind) || is_integer(&left.ty.kind) {
            &left.ty
        } else {
            self.errors
                .push(SemanticError::ExpectedANumericOperand { span: left.ty.span });

            return CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: left.ty.span,
            };
        };

        let right_type = if is_float(&right.ty.kind) || is_integer(&right.ty.kind) {
            &right.ty
        } else {
            self.errors
                .push(SemanticError::ExpectedANumericOperand { span: right.ty.span });

            return CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: right.ty.span,
            };
        };

        if (is_float(&left_type.kind) && is_integer(&right_type.kind))
            || (is_integer(&left_type.kind) && is_float(&right_type.kind))
        {
            self.errors.push(SemanticError::MixedFloatAndInteger { span });

            return CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: span,
            };
        }

        if is_signed(&left_type.kind) != is_signed(&right_type.kind) {
            self.errors.push(SemanticError::MixedSignedAndUnsigned { span });

            return CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: span,
            };
        }

        if right_type == left_type {
            return left_type.clone();
        }

        let left_rank = get_numeric_type_rank(&left_type.kind);
        let right_rank = get_numeric_type_rank(&right_type.kind);

        if left_rank > right_rank {
            CheckedType {
                kind: left_type.kind.clone(),
                span,
            }
        } else {
            CheckedType {
                kind: right_type.kind.clone(),
                span,
            }
        }
    }
}
