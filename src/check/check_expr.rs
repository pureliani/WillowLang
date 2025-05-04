use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        base::base_expression::{Expr, ExprKind},
        checked::{
            checked_declaration::{CheckedParam, CheckedStructDecl, CheckedVarDecl},
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind, TypeSpan},
        },
        Span,
    },
    tokenizer::NumberKind,
};

use super::{
    check_stmt::check_generic_params,
    check_stmts::check_stmts,
    expressions::{
        check_access_expr::check_access_expr, check_addition_expr::check_addition_expr,
        check_and_expr::check_and_expr,
        check_arithmetic_negation_expr::check_arithmetic_negation_expr,
        check_division_expr::check_division_expr, check_equality_expr::check_equality_expr,
        check_greater_than_expr::check_greater_than_expr,
        check_inequality_expr::check_inequality_expr, check_less_than_expr::check_less_than_expr,
        check_less_than_or_equal_expr::check_less_than_or_equal_expr,
        check_logical_negation_expr::check_logical_negation_expr,
        check_modulo_expr::check_modulo_expr, check_multiplication_expr::check_multiplication_expr,
        check_or_expr::check_or_expr, check_static_access_expr::check_static_access_expr,
        check_subtraction_expr::check_subtraction_expr, check_type_cast_expr::check_type_cast_expr,
    },
    scope::{Scope, ScopeKind, SymbolEntry},
    type_annotation_to_semantic::type_annotation_to_semantic,
    SemanticError, SemanticErrorKind,
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
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);
            let checked_op = check_binary_numeric_operation(&checked_left, &checked_right, errors);

            let type_kind = if checked_op.kind == TypeKind::Unknown {
                TypeKind::Unknown
            } else {
                TypeKind::Bool
            };

            let expr_type = Type {
                kind: type_kind,
                span: checked_op.span,
            };

            CheckedExpr {
                kind: CheckedExprKind::GreaterThanOrEqual {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::Equal { left, right } => check_equality_expr(left, field, errors, scope),
        ExprKind::NotEqual { left, right } => check_inequality_expr(left, right, errors, scope),
        ExprKind::And { left, right } => check_and_expr(left, right, errors, scope),
        ExprKind::Or { left, right } => check_or_expr(left, right, errors, scope),
        ExprKind::Access { left, field } => check_access_expr(left, field, errors, scope),
        ExprKind::StaticAccess { left, field } => {
            check_static_access_expr(left, field, errors, scope)
        }
        ExprKind::TypeCast { left, target } => check_type_cast_expr(left, target, errors, scope),
        ExprKind::IsType { left, target } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_target = type_annotation_to_semantic(&target, errors, scope);

            if !matches!(checked_left.expr_type.kind, TypeKind::Union { .. }) {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotUseIsTypeOnNonUnion,
                    expr.span,
                ));
            }

            CheckedExpr {
                kind: CheckedExprKind::IsType {
                    left: Box::new(checked_left),
                    target: checked_target,
                },
                expr_type: Type {
                    kind: TypeKind::Bool,
                    span: TypeSpan::Expr(expr.span),
                },
            }
        }
        ExprKind::GenericApply { left, args } => todo!(),
        ExprKind::FnCall { left, args } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_args: Vec<_> = args
                .into_iter()
                .map(|arg| check_expr(arg, errors, scope.clone()))
                .collect();

            let mut call_result_type = Type {
                kind: TypeKind::Unknown,
                span: TypeSpan::Expr(expr.span),
            };

            match &checked_left.expr_type.kind {
                TypeKind::FnType {
                    generic_params,
                    params,
                    return_type,
                } if !generic_params.is_empty() => {
                    // --- Call on a Generic Function without Explicit Type Arguments ---
                    // This requires type inference, which is complex.
                    // For now, let's require explicit arguments for generic functions.
                    todo!("Implement type inference and substitution")
                }
                TypeKind::FnType {
                    params,
                    return_type,
                    generic_params,
                } if generic_params.is_empty() => {
                    call_result_type = *return_type.clone();

                    if params.len() != checked_args.len() {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::ArgumentCountMismatch {
                                expected: params.len(),
                                received: checked_args.len(),
                            },
                            expr.span,
                        ));
                    } else {
                        for (param, arg) in params.iter().zip(checked_args.iter()) {
                            if !check_is_assignable(&arg.expr_type, &param.constraint) {
                                errors.push(SemanticError::new(
                                    SemanticErrorKind::TypeMismatch {
                                        expected: param.constraint.clone(),
                                        received: arg.expr_type.clone(),
                                    },
                                    arg.expr_type.unwrap_expr_span(),
                                ));
                            }
                        }
                    }
                }
                TypeKind::GenericApply { target, type_args } => {
                    if let TypeKind::FnType {
                        params,
                        return_type,
                        generic_params,
                    } = &target.kind
                    {
                        if generic_params.len() != type_args.len() {
                            errors.push(SemanticError::new(
                                SemanticErrorKind::GenericArgumentCountMismatch {
                                    expected: generic_params.len(),
                                    received: type_args.len(),
                                },
                                checked_left.expr_type.unwrap_expr_span(),
                            ));
                        } else {
                            // Build substitution map
                            let substitution: GenericSubstitutionMap = generic_params
                                .iter()
                                .map(|gp| gp.identifier.name.clone())
                                .zip(type_args.iter().cloned())
                                .collect();

                            // Substitute parameter and return types
                            let substituted_params: Vec<Type> = params
                                .iter()
                                .map(|p| substitute_generics(&p.constraint, &substitution, errors))
                                .collect();

                            let substituted_return_type =
                                substitute_generics(return_type, &substitution, errors);

                            call_result_type = substituted_return_type;

                            if substituted_params.len() != checked_args.len() {
                                errors.push(SemanticError::new(
                                    SemanticErrorKind::ArgumentCountMismatch {
                                        expected: substituted_params.len(),
                                        received: checked_args.len(),
                                    },
                                    expr.span,
                                ));
                            } else {
                                for (expected_type, arg) in
                                    substituted_params.iter().zip(checked_args.iter())
                                {
                                    if !check_is_assignable(&arg.expr_type, expected_type) {
                                        errors.push(SemanticError::new(
                                            SemanticErrorKind::TypeMismatch {
                                                expected: expected_type.clone(),
                                                received: arg.expr_type.clone(),
                                            },
                                            arg.expr_type.unwrap_expr_span(),
                                        ));
                                    }
                                }
                            }
                        }
                    } else {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::CannotCall(*target.clone()),
                            checked_left.expr_type.unwrap_expr_span(),
                        ));
                    }
                }
                non_callable_type => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::CannotCall(checked_left.expr_type.clone()),
                        checked_left.expr_type.unwrap_expr_span(),
                    ));
                }
            }

            CheckedExpr {
                expr_type: call_result_type,
                kind: CheckedExprKind::FnCall {
                    left: Box::new(checked_left),
                    args: checked_args,
                },
            }
        }
        ExprKind::StructInit { left, fields } => todo!(),
        ExprKind::Null => CheckedExpr {
            kind: CheckedExprKind::Null,
            expr_type: Type {
                kind: TypeKind::Null,
                span: TypeSpan::Expr(expr.span),
            },
        },
        ExprKind::BoolLiteral { value } => CheckedExpr {
            kind: CheckedExprKind::BoolLiteral { value },

            expr_type: Type {
                kind: TypeKind::Bool,
                span: TypeSpan::Expr(expr.span),
            },
        },
        ExprKind::Number { value } => {
            let type_kind = match value {
                NumberKind::I64(_) => TypeKind::I64,
                NumberKind::I32(_) => TypeKind::I32,
                NumberKind::I16(_) => TypeKind::I16,
                NumberKind::I8(_) => TypeKind::I8,
                NumberKind::F32(_) => TypeKind::F32,
                NumberKind::F64(_) => TypeKind::F64,
                NumberKind::U64(_) => TypeKind::U64,
                NumberKind::U32(_) => TypeKind::U32,
                NumberKind::U16(_) => TypeKind::U16,
                NumberKind::U8(_) => TypeKind::U8,
                NumberKind::USize(_) => TypeKind::USize,
                NumberKind::ISize(_) => TypeKind::ISize,
            };

            CheckedExpr {
                kind: CheckedExprKind::Number { value },
                expr_type: Type {
                    kind: type_kind,
                    span: TypeSpan::Expr(expr.span),
                },
            }
        }
        ExprKind::String(string_node) => CheckedExpr {
            expr_type: Type {
                kind: TypeKind::Array {
                    item_type: Box::new(Type {
                        kind: TypeKind::Char,
                        span: TypeSpan::None,
                    }),
                    size: string_node.value.len(),
                },
                span: TypeSpan::Expr(expr.span),
            },
            kind: CheckedExprKind::String(string_node),
        },
        ExprKind::Identifier(id) => {
            let expr_type = scope
                .borrow()
                .lookup(&id.name)
                .map(|entry| match entry {
                    SymbolEntry::StructDecl(decl) => Type {
                        kind: TypeKind::Struct(decl),
                        span: TypeSpan::Expr(expr.span),
                    },
                    SymbolEntry::TypeAliasDecl(decl) => Type {
                        kind: TypeKind::TypeAlias(decl),
                        span: TypeSpan::Expr(expr.span),
                    },
                    SymbolEntry::EnumDecl(decl) => Type {
                        kind: TypeKind::Enum(decl),
                        span: TypeSpan::Expr(expr.span),
                    },
                    SymbolEntry::GenericParam(_) => {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::CannotUseGenericParameterAsValue,
                            expr.span,
                        ));

                        Type {
                            kind: TypeKind::Unknown,
                            span: TypeSpan::Expr(expr.span),
                        }
                    }
                    SymbolEntry::VarDecl(CheckedVarDecl {
                        identifier,
                        documentation,
                        constraint,
                        value,
                    }) => Type {
                        kind: constraint.kind,
                        span: TypeSpan::Expr(expr.span),
                    },
                })
                .unwrap_or_else(|| {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                        expr.span,
                    ));

                    Type {
                        kind: TypeKind::Unknown,
                        span: TypeSpan::Expr(expr.span),
                    }
                });

            CheckedExpr {
                kind: CheckedExprKind::Identifier(id),
                expr_type,
            }
        }
        ExprKind::Fn {
            params,
            body,
            return_type,
            generic_params,
        } => {
            let fn_scope = scope.borrow().child(ScopeKind::Function);

            let checked_params: Vec<CheckedParam> = params
                .iter()
                .map(|param| {
                    let checked_constraint =
                        type_annotation_to_semantic(&param.constraint, errors, fn_scope.clone());

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
            let checked_generic_params =
                check_generic_params(&generic_params, errors, fn_scope.clone());

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

            let inferred_return_type = union_of(
                &return_exprs
                    .iter()
                    .map(|e| e.expr_type.clone())
                    .collect::<Vec<Type>>(),
            );

            let param_types: Vec<CheckedParam> = params
                .into_iter()
                .map(|p| CheckedParam {
                    constraint: type_annotation_to_semantic(
                        &p.constraint,
                        errors,
                        fn_scope.clone(),
                    ),
                    identifier: p.identifier,
                })
                .collect();

            let expected_return_type = return_type
                .map(|return_t| type_annotation_to_semantic(&return_t, errors, fn_scope.clone()));

            let actual_return_type = if let Some(explicit_return_type) = expected_return_type {
                for return_expr in return_exprs.iter() {
                    let is_assignable =
                        check_is_assignable(&return_expr.expr_type, &explicit_return_type);
                }

                explicit_return_type
            } else {
                inferred_return_type
            };

            let expr_type = Type {
                kind: TypeKind::FnType {
                    params: param_types,
                    return_type: Box::new(actual_return_type.clone()),
                    generic_params: checked_generic_params.clone(),
                },
                span: TypeSpan::Expr(expr.span),
            };

            CheckedExpr {
                expr_type,
                kind: CheckedExprKind::Fn {
                    params: checked_params,
                    body: checked_body,
                    return_type: actual_return_type,
                    generic_params: checked_generic_params,
                },
            }
        }
        ExprKind::If {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
        } => {
            let mut if_else_expr_type = Type {
                kind: TypeKind::Void,
                span: TypeSpan::Expr(expr.span),
            };

            let checked_condition = check_expr(*condition, errors, scope.clone());
            if checked_condition.expr_type.kind != TypeKind::Bool {
                errors.push(SemanticError::new(
                    SemanticErrorKind::TypeMismatch {
                        expected: Type {
                            kind: TypeKind::Bool,
                            span: checked_condition.expr_type.span,
                        },
                        received: checked_condition.expr_type.clone(),
                    },
                    checked_condition.expr_type.unwrap_expr_span(),
                ));
            }
            let then_branch_scope = scope.borrow().child(ScopeKind::CodeBlock);
            let checked_then_branch_statements =
                check_stmts(then_branch.statements, errors, then_branch_scope.clone());

            let checked_then_branch_final_expr = then_branch.final_expr.map(|fe| {
                let checked_final_expr = check_expr(*fe, errors, then_branch_scope.clone());

                if_else_expr_type = union_of(&[
                    if_else_expr_type.clone(),
                    checked_final_expr.expr_type.clone(),
                ]);

                Box::new(checked_final_expr)
            });

            let checked_then_branch = CheckedBlockContents {
                final_expr: checked_then_branch_final_expr,
                statements: checked_then_branch_statements,
            };

            let checked_else_if_branches: Vec<(Box<CheckedExpr>, CheckedBlockContents)> =
                else_if_branches
                    .into_iter()
                    .map(|ei| {
                        let checked_condition = check_expr(*ei.0, errors, scope.clone());

                        let else_if_scope = scope.borrow().child(ScopeKind::CodeBlock);
                        let checked_codeblock_statements =
                            check_stmts(ei.1.statements, errors, else_if_scope.clone());
                        let checked_codeblock_final_expr = ei.1.final_expr.map(|fe| {
                            let checked_final_expr = check_expr(*fe, errors, else_if_scope.clone());

                            if_else_expr_type = union_of(&[
                                if_else_expr_type.clone(),
                                checked_final_expr.expr_type.clone(),
                            ]);

                            Box::new(checked_final_expr)
                        });

                        (
                            Box::new(checked_condition),
                            CheckedBlockContents {
                                final_expr: checked_codeblock_final_expr,
                                statements: checked_codeblock_statements,
                            },
                        )
                    })
                    .collect();

            let checked_else_branch = else_branch.map(|br| {
                let else_scope = scope.borrow().child(ScopeKind::CodeBlock);
                let checked_statements = check_stmts(br.statements, errors, else_scope.clone());
                let checked_final_expr = br.final_expr.map(|fe| {
                    let checked_final_expr = check_expr(*fe, errors, else_scope);

                    if_else_expr_type = union_of(&[
                        if_else_expr_type.clone(),
                        checked_final_expr.expr_type.clone(),
                    ]);

                    Box::new(checked_final_expr)
                });

                CheckedBlockContents {
                    statements: checked_statements,
                    final_expr: checked_final_expr,
                }
            });

            CheckedExpr {
                expr_type: if_else_expr_type,
                kind: CheckedExprKind::If {
                    condition: Box::new(checked_condition),
                    then_branch: checked_then_branch,
                    else_if_branches: checked_else_if_branches,
                    else_branch: checked_else_branch,
                },
            }
        }
        ExprKind::ArrayLiteral { items } => {
            let checked_items: Vec<CheckedExpr> = items
                .iter()
                .map(|item| check_expr(*item.clone(), errors, scope.clone()))
                .collect();

            let unionized_types = union_of(
                &checked_items
                    .iter()
                    .map(|item| item.expr_type.clone())
                    .collect::<Vec<Type>>(),
            );

            CheckedExpr {
                expr_type: Type {
                    kind: TypeKind::Array {
                        item_type: Box::new(unionized_types),
                        size: items.len(),
                    },
                    span: TypeSpan::Expr(expr.span),
                },
                kind: CheckedExprKind::ArrayLiteral {
                    items: checked_items,
                },
            }
        }
        ExprKind::Block(block_contents) => {
            let block_scope = scope.borrow().child(ScopeKind::CodeBlock);

            let checked_codeblock_statements =
                check_stmts(block_contents.statements, errors, block_scope.clone());
            let checked_codeblock_final_expr = block_contents.final_expr.map(|fe| {
                let checked_final_expr = check_expr(*fe, errors, block_scope.clone());

                Box::new(checked_final_expr)
            });

            let expr_type = checked_codeblock_final_expr
                .clone()
                .map(|fe| fe.expr_type)
                .unwrap_or(Type {
                    kind: TypeKind::Void,
                    span: TypeSpan::Expr(expr.span),
                });

            CheckedExpr {
                kind: CheckedExprKind::Block(CheckedBlockContents {
                    final_expr: checked_codeblock_final_expr,
                    statements: checked_codeblock_statements,
                }),
                expr_type,
            }
        }
        ExprKind::Error(parsing_error) => todo!(),
    }
}
