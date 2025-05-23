use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{
            base_declaration::{GenericParam, Param},
            base_expression::BlockContents,
            base_type::TypeAnnotation,
        },
        checked::{
            checked_declaration::{CheckedParam, CheckedVarDecl},
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind, GenericFn},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        check_stmt::check_generic_params,
        check_stmts::check_stmts,
        scope::{Scope, ScopeKind, SymbolEntry},
        utils::{
            check_is_assignable::check_is_assignable, check_returns::check_returns,
            type_annotation_to_semantic::check_type, union_of::union_of,
        },
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_fn_expr(
    params: Vec<Param>,
    body: BlockContents,
    return_type: Option<TypeAnnotation>,
    generic_params: Vec<GenericParam>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let fn_scope = scope.borrow().child(ScopeKind::Function);

    let checked_params: Vec<CheckedParam> = params
        .iter()
        .map(|param| {
            let checked_constraint = check_type(&param.constraint, errors, fn_scope.clone());

            fn_scope.borrow_mut().insert(
                param.identifier.name.to_owned(),
                SymbolEntry::VarDecl(CheckedVarDecl {
                    documentation: None,
                    identifier: param.identifier.to_owned(),
                    constraint: checked_constraint.clone(),
                    value: None,
                }),
            );

            CheckedParam {
                constraint: checked_constraint,
                identifier: param.identifier.to_owned(),
            }
        })
        .collect();
    let checked_generic_params = check_generic_params(&generic_params, errors, fn_scope.clone());

    let checked_statements = check_stmts(body.statements, errors, fn_scope.clone());
    let checked_final_expr = body
        .final_expr
        .map(|fe| Box::new(check_expr(*fe, errors, fn_scope.clone())));

    let checked_body = CheckedBlockContents {
        statements: checked_statements.clone(),
        final_expr: checked_final_expr.clone(),
    };

    let mut return_exprs = check_returns(&checked_statements, errors, fn_scope.clone());
    if let Some(final_expr) = checked_final_expr {
        return_exprs.push(*final_expr);
    }
    let inferred_return_type = union_of(return_exprs.iter().map(|e| e.ty.clone()));

    let param_types: Vec<CheckedParam> = params
        .into_iter()
        .map(|p| CheckedParam {
            constraint: check_type(&p.constraint, errors, fn_scope.clone()),
            identifier: p.identifier,
        })
        .collect();

    let expected_return_type =
        return_type.map(|return_t| check_type(&return_t, errors, fn_scope.clone()));

    let actual_return_type = if let Some(explicit_return_type) = expected_return_type {
        for return_expr in return_exprs.iter() {
            let is_assignable = check_is_assignable(&return_expr.ty, &explicit_return_type);
            if !is_assignable {
                errors.push(SemanticError {
                    kind: SemanticErrorKind::ReturnTypeMismatch {
                        expected: explicit_return_type.clone(),
                        received: return_expr.ty.clone(),
                    },
                    span: return_expr.span,
                });
            }
        }

        explicit_return_type
    } else {
        inferred_return_type
    };

    if generic_params.is_empty() {
        let expr_type = CheckedType::FnType {
            params: param_types,
            return_type: Box::new(actual_return_type.clone()),
        };

        CheckedExpr {
            span: expr_span,
            ty: expr_type,
            kind: CheckedExprKind::Fn {
                params: checked_params,
                body: checked_body,
                return_type: actual_return_type,
            },
        }
    } else {
        let expr_type = CheckedType::GenericFnType {
            params: param_types,
            return_type: Box::new(actual_return_type.clone()),
            generic_params: checked_generic_params.clone(),
        };

        CheckedExpr {
            span: expr_span,
            ty: expr_type,
            kind: CheckedExprKind::GenericFn(GenericFn {
                params: checked_params,
                body: checked_body,
                return_type: actual_return_type,
                generic_params: checked_generic_params,
            }),
        }
    }
}
