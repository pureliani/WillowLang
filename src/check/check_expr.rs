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
    check_is_assignable::check_is_assignable,
    check_returns::check_returns,
    check_stmt::check_generic_params,
    check_stmts::check_stmts,
    scope::{Scope, ScopeKind, SymbolEntry},
    type_annotation_to_semantic::type_annotation_to_semantic,
    SemanticError, SemanticErrorKind,
};

pub fn union_of(types: &[Type]) -> Type {
    let mut union_items: Vec<Type> = vec![];

    for t in types {
        match &t.kind {
            TypeKind::Union(items) => {
                union_items.extend(items.clone());
            }
            _ => union_items.push(t.clone()),
        }
    }

    // TODO: somehow deduplicate union items

    Type {
        kind: TypeKind::Union(union_items),
        span: TypeSpan::None,
    }
}

fn numeric_type_rank(ty: &Type) -> i32 {
    use TypeKind::*;
    match &ty.kind {
        I8 | U8 => 1,
        I16 | U16 => 2,
        I32 | U32 | ISize | USize => 3,
        I64 | U64 => 4,
        F32 => 5,
        F64 => 6,
        _ => 0,
    }
}

fn is_signed(ty: &Type) -> bool {
    use TypeKind::*;
    matches!(ty.kind, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}

fn is_integer(ty: &Type) -> bool {
    use TypeKind::*;
    matches!(
        ty.kind,
        I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | ISize | USize
    )
}

fn is_float(ty: &Type) -> bool {
    use TypeKind::*;
    matches!(ty.kind, F32 | F64)
}

pub fn check_binary_numeric_operation(
    left: &CheckedExpr,
    right: &CheckedExpr,
    errors: &mut Vec<SemanticError>,
) -> Type {
    let combined_span = Span {
        start: left.expr_type.unwrap_expr_span().start,
        end: right.expr_type.unwrap_expr_span().start,
    };

    let left_type = if is_float(&left.expr_type) || is_integer(&left.expr_type) {
        &left.expr_type
    } else {
        errors.push(SemanticError::new(
            SemanticErrorKind::NonNumericOperand,
            left.expr_type.unwrap_expr_span(),
        ));

        return Type {
            kind: TypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    };

    let right_type = if is_float(&right.expr_type) || is_integer(&right.expr_type) {
        &right.expr_type
    } else {
        errors.push(SemanticError::new(
            SemanticErrorKind::NonNumericOperand,
            right.expr_type.unwrap_expr_span(),
        ));

        return Type {
            kind: TypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    };

    if (is_float(&left_type) && is_integer(&right_type))
        || (is_integer(&left_type) && is_float(&right_type))
    {
        errors.push(SemanticError::new(
            SemanticErrorKind::MixedFloatAndInteger,
            combined_span,
        ));
        return Type {
            kind: TypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    }

    if is_signed(&left_type) != is_signed(&right_type) {
        errors.push(SemanticError::new(
            SemanticErrorKind::MixedSignedAndUnsigned,
            combined_span,
        ));
        return Type {
            kind: TypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    }

    if right_type == left_type {
        return left_type.clone();
    }

    let left_rank = numeric_type_rank(&left_type);
    let right_rank = numeric_type_rank(&right_type);

    if left_rank > right_rank {
        left_type.clone()
    } else {
        right_type.clone()
    }
}

type Substitution = HashMap<String, Type>;

fn substitute_generics(
    ty: &Type,
    substitution: &Substitution,
    errors: &mut Vec<SemanticError>,
) -> Type {
    match &ty.kind {
        TypeKind::GenericParam(gp) => substitution
            .get(&gp.identifier.name)
            .cloned()
            .unwrap_or_else(|| {
                let span = ty.unwrap_annotation_span();

                errors.push(SemanticError::new(
                    SemanticErrorKind::UnresolvedGenericParam(gp.identifier.name.clone()),
                    span,
                ));

                Type {
                    kind: TypeKind::Unknown,
                    span: ty.span,
                }
            }),
        TypeKind::FnType {
            params,
            return_type,
            generic_params,
        } => {
            // IMPORTANT: When substituting within a function type, we generally DON'T
            // substitute its *own* generic parameters. Those are bound locally.
            // We only substitute types that came from an outer scope's substitution.
            let substituted_params = params
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            let substituted_return_type = substitute_generics(return_type, substitution, errors);

            Type {
                kind: TypeKind::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                    generic_params: generic_params.clone(), // Keep original generic params
                },
                span: ty.span,
            }
        }
        TypeKind::Struct(decl) => {
            // Similar to FnType, a struct definition's generic params are local.
            // We substitute types *within* its properties if those types refer
            // to generics from the *outer* substitution context.
            let substituted_props = decl
                .properties
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            Type {
                kind: TypeKind::Struct(CheckedStructDecl {
                    properties: substituted_props,
                    ..decl.clone()
                }),
                span: ty.span,
            }
        }
        TypeKind::Array { item_type, size } => Type {
            kind: TypeKind::Array {
                item_type: Box::new(substitute_generics(item_type, substitution, errors)),
                size: *size,
            },
            span: ty.span,
        },
        TypeKind::Union(items) => {
            let substituted_items: Vec<Type> = items
                .iter()
                .map(|t| substitute_generics(t, substitution, errors))
                .collect();
            // Re-apply union_of logic to simplify the result
            union_of(&substituted_items)
        }
        // Base types and others don't contain substitutable generics directly
        TypeKind::I8
        | TypeKind::I16
        | TypeKind::I32
        | TypeKind::I64
        | TypeKind::ISize
        | TypeKind::U8
        | TypeKind::U16
        | TypeKind::U32
        | TypeKind::U64
        | TypeKind::USize
        | TypeKind::F32
        | TypeKind::F64
        | TypeKind::Bool
        | TypeKind::Char
        | TypeKind::Void
        | TypeKind::Null
        | TypeKind::Unknown
        | TypeKind::TypeAlias(_)
        | TypeKind::Enum(_) => ty.clone(),
        // Note: TypeAlias/Enum might contain generics *internally*, but the substitution
        // should happen when the alias/enum is *resolved* to its underlying type,
        // not on the alias/enum type itself.
    }
}

pub fn infer_generics(
    expected: &Type,
    received: &Type,
    substitution: &mut Substitution,
    errors: &mut Vec<SemanticError>,
) {
    match (&expected.kind, &received.kind) {
        // Handle generics
        (TypeKind::GenericParam(gp), received_kind) => {
            let name = &gp.identifier.name;
            if let Some(existing) = substitution.get(name) {
                if &existing.kind != received_kind {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::ConflictingGenericBinding {
                            existing: existing.clone(),
                            new: received.clone(),
                        },
                        received.unwrap_annotation_span(),
                    ));
                }
            } else {
                substitution.insert(name.clone(), received.clone());
            }
        }
        // Recursively check components (arrays, structs, etc.)
        (
            TypeKind::Array {
                item_type: maybe_generic,
                ..
            },
            TypeKind::Array {
                item_type: concrete,
                ..
            },
        ) => {
            infer_generics(maybe_generic, concrete, substitution, errors);
        }
        (TypeKind::Struct(maybe_generic), TypeKind::Struct(concrete)) => {
            for (maybe_generic_prop, concrete_prop) in maybe_generic
                .properties
                .iter()
                .zip(concrete.properties.iter())
            {
                infer_generics(
                    &maybe_generic_prop.constraint,
                    &concrete_prop.constraint,
                    substitution,
                    errors,
                );
            }
        }
        (
            TypeKind::FnType {
                params: maybe_generic_params,
                return_type: maybe_generic_return_type,
                generic_params: _,
            },
            TypeKind::FnType {
                params: concrete_params,
                return_type: concrete_return_type,
                generic_params: _,
            },
        ) => {
            todo!("Implement inferring types for functions")
        }
        (TypeKind::Union(maybe_generic), TypeKind::Union(concrete)) => {
            todo!("Implement inferring types for unions")
        }
        _ => {}
    }
}

pub fn check_expr(
    expr: Expr,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    match expr.kind {
        ExprKind::Not { right } => {
            let checked_right = check_expr(*right, errors, scope);

            let mut expr_type = Type {
                kind: TypeKind::Bool,
                span: TypeSpan::Expr(expr.span),
            };
            if checked_right.expr_type.kind != TypeKind::Bool {
                errors.push(SemanticError::new(
                    SemanticErrorKind::TypeMismatch {
                        expected: Type {
                            kind: TypeKind::Bool,
                            span: TypeSpan::Expr(expr.span),
                        },
                        received: checked_right.expr_type.clone(),
                    },
                    checked_right.expr_type.unwrap_expr_span(),
                ));
                expr_type.kind = TypeKind::Unknown
            }

            CheckedExpr {
                kind: CheckedExprKind::Not {
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::Neg { right } => {
            let checked_right = check_expr(*right, errors, scope);

            let expr_type = match &checked_right.expr_type {
                t if is_signed(&t) || is_float(&t) => t.clone(),
                unexpected_type => {
                    let expected = vec![
                        Type {
                            kind: TypeKind::I8,
                            span: TypeSpan::Expr(checked_right.expr_type.unwrap_expr_span()),
                        },
                        Type {
                            kind: TypeKind::I16,
                            span: TypeSpan::Expr(checked_right.expr_type.unwrap_expr_span()),
                        },
                        Type {
                            kind: TypeKind::I32,
                            span: TypeSpan::Expr(checked_right.expr_type.unwrap_expr_span()),
                        },
                        Type {
                            kind: TypeKind::I64,
                            span: TypeSpan::Expr(checked_right.expr_type.unwrap_expr_span()),
                        },
                        Type {
                            kind: TypeKind::ISize,
                            span: TypeSpan::Expr(checked_right.expr_type.unwrap_expr_span()),
                        },
                        Type {
                            kind: TypeKind::F32,
                            span: TypeSpan::Expr(checked_right.expr_type.unwrap_expr_span()),
                        },
                        Type {
                            kind: TypeKind::F64,
                            span: TypeSpan::Expr(checked_right.expr_type.unwrap_expr_span()),
                        },
                    ];

                    errors.push(SemanticError::new(
                        SemanticErrorKind::TypeMismatch {
                            expected: Type {
                                kind: TypeKind::Union(expected),
                                span: checked_right.expr_type.span,
                            },
                            received: unexpected_type.clone(),
                        },
                        checked_right.expr_type.unwrap_expr_span(),
                    ));

                    Type {
                        kind: TypeKind::Unknown,
                        span: TypeSpan::Expr(expr.span),
                    }
                }
            };

            CheckedExpr {
                expr_type,
                kind: CheckedExprKind::Neg {
                    right: Box::new(checked_right),
                },
            }
        }
        ExprKind::Add { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);
            let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

            CheckedExpr {
                kind: CheckedExprKind::Add {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::Subtract { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);
            let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

            CheckedExpr {
                kind: CheckedExprKind::Subtract {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::Multiply { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);
            let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

            CheckedExpr {
                kind: CheckedExprKind::Multiply {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::Divide { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);
            let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

            CheckedExpr {
                kind: CheckedExprKind::Divide {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::Modulo { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);
            let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

            CheckedExpr {
                kind: CheckedExprKind::Modulo {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::LessThan { left, right } => {
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
                kind: CheckedExprKind::LessThan {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::LessThanOrEqual { left, right } => {
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
                kind: CheckedExprKind::LessThanOrEqual {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::GreaterThan { left, right } => {
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
                kind: CheckedExprKind::GreaterThan {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
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
        ExprKind::Equal { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);

            let mut expr_type = Type {
                kind: TypeKind::Bool,
                span: TypeSpan::Expr(expr.span),
            };

            if checked_left.expr_type != checked_right.expr_type {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotCompareType {
                        of: checked_left.expr_type.clone(),
                        to: checked_right.expr_type.clone(),
                    },
                    expr.span,
                ));

                expr_type.kind = TypeKind::Unknown;
            }

            CheckedExpr {
                kind: CheckedExprKind::Equal {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::NotEqual { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);

            let mut expr_type = Type {
                kind: TypeKind::Bool,
                span: TypeSpan::Expr(expr.span),
            };
            if checked_left.expr_type != checked_right.expr_type {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotCompareType {
                        of: checked_left.expr_type.clone(),
                        to: checked_right.expr_type.clone(),
                    },
                    Span {
                        start: checked_left.expr_type.unwrap_expr_span().start,
                        end: checked_right.expr_type.unwrap_expr_span().end,
                    },
                ));

                expr_type.kind = TypeKind::Unknown
            }

            CheckedExpr {
                kind: CheckedExprKind::NotEqual {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::And { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);

            let mut expr_type = Type {
                kind: TypeKind::Bool,
                span: TypeSpan::Expr(expr.span),
            };

            if checked_left.expr_type.kind != TypeKind::Bool {
                errors.push(SemanticError::new(
                    SemanticErrorKind::TypeMismatch {
                        expected: Type {
                            kind: TypeKind::Bool,
                            span: checked_left.expr_type.span,
                        },
                        received: checked_left.expr_type.clone(),
                    },
                    checked_left.expr_type.unwrap_expr_span(),
                ));
                expr_type.kind = TypeKind::Unknown;
            }

            if checked_right.expr_type.kind != TypeKind::Bool {
                errors.push(SemanticError::new(
                    SemanticErrorKind::TypeMismatch {
                        expected: Type {
                            kind: TypeKind::Bool,
                            span: checked_right.expr_type.span,
                        },
                        received: checked_right.expr_type.clone(),
                    },
                    checked_right.expr_type.unwrap_expr_span(),
                ));
                expr_type.kind = TypeKind::Unknown;
            }

            CheckedExpr {
                kind: CheckedExprKind::And {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::Or { left, right } => {
            let checked_left = check_expr(*left, errors, scope.clone());
            let checked_right = check_expr(*right, errors, scope);

            let mut expr_type = Type {
                kind: TypeKind::Bool,
                span: TypeSpan::Expr(expr.span),
            };

            if checked_left.expr_type.kind != TypeKind::Bool {
                errors.push(SemanticError::new(
                    SemanticErrorKind::TypeMismatch {
                        expected: Type {
                            kind: TypeKind::Bool,
                            span: checked_left.expr_type.span,
                        },
                        received: checked_left.expr_type.clone(),
                    },
                    checked_left.expr_type.unwrap_expr_span(),
                ));
                expr_type.kind = TypeKind::Unknown;
            }

            if checked_right.expr_type.kind != TypeKind::Bool {
                errors.push(SemanticError::new(
                    SemanticErrorKind::TypeMismatch {
                        expected: Type {
                            kind: TypeKind::Bool,
                            span: checked_right.expr_type.span,
                        },
                        received: checked_right.expr_type.clone(),
                    },
                    checked_right.expr_type.unwrap_expr_span(),
                ));
                expr_type.kind = TypeKind::Unknown;
            }

            CheckedExpr {
                kind: CheckedExprKind::Or {
                    left: Box::new(checked_left),
                    right: Box::new(checked_right),
                },
                expr_type,
            }
        }
        ExprKind::Access { left, field } => {
            let checked_left = check_expr(*left, errors, scope);

            let expr_type = match &checked_left.expr_type.kind {
                TypeKind::Struct(StructTypeKind::Specialized(SpecializedStructDecl {
                    properties,
                    ..
                })) => properties
                    .iter()
                    .find(|p| p.identifier.name == field.name)
                    .map(|p| p.constraint.clone())
                    .unwrap_or_else(|| {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::UndefinedProperty(field.clone()),
                            field.span,
                        ));

                        Type {
                            kind: TypeKind::Unknown,
                            span: TypeSpan::Expr(expr.span),
                        }
                    }),
                TypeKind::GenericApply { target, type_args } => match &target.kind {
                    TypeKind::Struct(CheckedStructDecl {
                        properties,
                        generic_params,
                        ..
                    }) => {
                        if generic_params.len() != type_args.len() {
                            errors.push(SemanticError::new(
                                SemanticErrorKind::GenericArgumentCountMismatch {
                                    expected: generic_params.len(),
                                    received: type_args.len(),
                                },
                                checked_left.expr_type.unwrap_expr_span(),
                            ));

                            Type {
                                kind: TypeKind::Unknown,
                                span: TypeSpan::Expr(expr.span),
                            }
                        } else {
                            let prop_type = properties
                                .iter()
                                .find(|p| p.identifier.name == field.name)
                                .map(|p| p.constraint.clone())
                                .unwrap_or_else(|| {
                                    errors.push(SemanticError::new(
                                        SemanticErrorKind::UndefinedProperty(field.clone()),
                                        field.span,
                                    ));

                                    Type {
                                        kind: TypeKind::Unknown,
                                        span: TypeSpan::Expr(expr.span),
                                    }
                                });

                            let substitution: Substitution = generic_params
                                .iter()
                                .map(|gp| gp.identifier.name.clone())
                                .zip(type_args.iter().cloned())
                                .collect();

                            let substituted_prop_type =
                                substitute_generics(&prop_type, &substitution, errors);

                            Type {
                                kind: substituted_prop_type.kind,
                                span: TypeSpan::Expr(expr.span),
                            }
                        }
                    }
                    _ => {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::CannotAccess(checked_left.expr_type.clone()),
                            field.span,
                        ));

                        Type {
                            kind: TypeKind::Unknown,
                            span: TypeSpan::Expr(expr.span),
                        }
                    }
                },
                _ => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::CannotAccess(checked_left.expr_type.clone()),
                        field.span,
                    ));

                    Type {
                        kind: TypeKind::Unknown,
                        span: TypeSpan::Expr(expr.span),
                    }
                }
            };

            CheckedExpr {
                kind: CheckedExprKind::Access {
                    left: Box::new(checked_left.clone()),
                    field: field.clone(),
                },
                expr_type,
            }
        }
        ExprKind::StaticAccess { left, field } => todo!(),
        ExprKind::TypeCast { left, target } => todo!(),
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
                            let substitution: Substitution = generic_params
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
