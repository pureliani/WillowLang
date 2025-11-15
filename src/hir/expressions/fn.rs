use std::sync::{Arc, RwLock};

use crate::{
    ast::{decl::FnDecl, expr::BlockContents},
    hir::{
        cfg::{CheckedDeclaration, Terminator, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::{
                CallingConvention, CheckedFnDecl, CheckedFnType, CheckedParam,
                CheckedVarDecl, VarStorage,
            },
            checked_type::{Type, TypeKind},
        },
        utils::{scope::ScopeKind, var_capture_analyzer::analyze_captures},
        FunctionBuilder, HIRContext,
    },
    tokenize::NumberKind,
};

impl FunctionBuilder {
    pub fn build_fn_body(
        &mut self,
        ctx: &mut HIRContext,
        params: &[CheckedParam],
        body: BlockContents,
    ) {
        for param in params {
            let variable_stack_ptr = self.emit_stack_alloc(ctx, param.ty.clone(), 1);

            // The calling convention will be responsible for storing the actual argument value
            // into this variable's stack slot before the function body begins execution.

            let checked_var_decl = CheckedVarDecl {
                id: ctx.program_builder.new_declaration_id(),
                storage: VarStorage::Stack(variable_stack_ptr),
                identifier: param.identifier,
                documentation: None,
                constraint: param.ty.clone(),
            };

            ctx.module_builder.scope_insert(
                param.identifier,
                CheckedDeclaration::Var(checked_var_decl),
                param.identifier.span,
            );
        }

        let final_value = self.build_codeblock_expr(ctx, body);
        let final_value_type = ctx.program_builder.get_value_type(&final_value);

        if !self.check_is_assignable(&final_value_type, &self.return_type) {
            ctx.module_builder.errors.push(SemanticError {
                span: final_value_type.span,
                kind: SemanticErrorKind::ReturnTypeMismatch {
                    expected: self.return_type.clone(),
                    received: final_value_type,
                },
            });

            self.set_basic_block_terminator(Terminator::Unreachable);
            return;
        }

        self.set_basic_block_terminator(Terminator::Return {
            value: Some(final_value),
        });
    }

    pub fn build_fn_expr(&mut self, ctx: &mut HIRContext, fn_decl: FnDecl) -> Value {
        let FnDecl {
            identifier,
            params,
            return_type,
            body,
        } = fn_decl;

        let checked_params: Vec<CheckedParam> = params
            .iter()
            .map(|p| CheckedParam {
                identifier: p.identifier,
                ty: self.check_type_annotation(ctx, &p.constraint),
            })
            .collect();
        let checked_return_type = self.check_type_annotation(ctx, &return_type);

        // STAGE 1: ANALYZE - Run the read-only capture analysis pass.
        let captures_map = analyze_captures(ctx, &checked_params, &body);

        if captures_map.is_empty() {
            // --- PATH A: NO CAPTURES (A regular function) ---
            let new_function_id = ctx.program_builder.new_function_id();
            let checked_fn_decl = Arc::new(RwLock::new(CheckedFnDecl {
                id: ctx.program_builder.new_declaration_id(),
                identifier,
                params: checked_params.clone(),
                return_type: checked_return_type.clone(),
                body: None,
            }));

            ctx.module_builder.scope_insert(
                identifier,
                CheckedDeclaration::Function(checked_fn_decl.clone()),
                identifier.span,
            );

            let mut new_fn_builder = FunctionBuilder::new(checked_return_type.clone());
            ctx.module_builder
                .enter_scope(ScopeKind::Function(Box::new(new_fn_builder)));

            let active_builder = ctx.module_builder.get_active_function_builder();
            active_builder.build_fn_body(ctx, &checked_params, body);

            let scope = ctx.module_builder.exit_scope();
            let final_fn_builder = match scope.kind {
                ScopeKind::Function(builder) => builder,
                _ => panic!("INTERNAL COMPILER ERROR: Expected to pop a function scope."),
            };

            checked_fn_decl.write().unwrap().body = Some(final_fn_builder.cfg);
            let fn_type = Type {
                kind: TypeKind::FnType(CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                    convention: CallingConvention::Native,
                }),
                span: identifier.span,
            };
            Value::FunctionAddr {
                function_id: new_function_id,
                ty: fn_type,
            }
        } else {
            // --- PATH B: HAS CAPTURES (A closure) ---

            // STAGE 2: MUTATE - Update the storage of captured variables in the current scope.
            let mut captures: Vec<CheckedParam> = Vec::new();
            for (id_node, ty) in captures_map {
                captures.push(CheckedParam {
                    identifier: id_node,
                    ty,
                });
                let decl_to_update = ctx.module_builder.scope_lookup_mut(id_node.name)
                        .expect("INTERNAL COMPILER ERROR: Captured variable not found in scope during mutation phase.");
                if let CheckedDeclaration::Var(var_decl) = decl_to_update {
                    var_decl.storage = VarStorage::Captured;
                } else {
                    panic!(
                        "INTERNAL COMPILER ERROR: Captured a non-variable declaration."
                    );
                }
            }
            captures.sort_by_key(|p| p.identifier.name.0);

            // STAGE 3: GENERATE - Build the closure object and its wrapped function.
            let env_struct_type = Type {
                kind: TypeKind::Struct(captures.clone()),
                span: identifier.span,
            };
            let env_ptr = self
                .emit_heap_alloc(
                    ctx,
                    env_struct_type.clone(),
                    Value::NumberLiteral(NumberKind::USize(1)),
                )
                .expect("Failed to allocate closure environment");

            for (i, captured_var) in captures.iter().enumerate() {
                let field_ptr = self.emit_get_field_ptr_by_index(
                    ctx,
                    env_ptr,
                    i,
                    captured_var.ty.clone(),
                );
                let captured_value =
                    self.build_identifier_expr(ctx, captured_var.identifier);
                self.emit_store(ctx, field_ptr, captured_value);
            }

            let mut closure_params = vec![CheckedParam {
                identifier,
                ty: Type {
                    kind: TypeKind::Pointer(Box::new(env_struct_type.clone())),
                    span: identifier.span,
                },
            }];
            closure_params.extend_from_slice(&checked_params);

            let new_function_id = ctx.program_builder.new_function_id();
            let checked_fn_decl = Arc::new(RwLock::new(CheckedFnDecl {
                id: ctx.program_builder.new_declaration_id(),
                identifier,
                params: closure_params.clone(),
                return_type: checked_return_type.clone(),
                body: None,
            }));
            // TODO: Store this `checked_fn_decl` in a program-wide function list.

            let mut new_fn_builder = FunctionBuilder::new(checked_return_type.clone());
            ctx.module_builder
                .enter_scope(ScopeKind::Function(Box::new(new_fn_builder)));

            // TODO: Modify `build_identifier_expr` to handle `VarStorage::Captured`.
            // It should know to load from the environment pointer (the first param).
            let active_builder = ctx.module_builder.get_active_function_builder();
            active_builder.build_fn_body(ctx, &closure_params, body);

            let scope = ctx.module_builder.exit_scope();
            let final_fn_builder = match scope.kind {
                ScopeKind::Function(builder) => builder,
                _ => panic!("INTERNAL COMPILER ERROR: Expected to pop a function scope."),
            };
            checked_fn_decl.write().unwrap().body = Some(final_fn_builder.cfg);

            let closure_obj_type = Type {
                kind: TypeKind::Struct(vec![
                    CheckedParam {
                        identifier,
                        ty: Type {
                            kind: TypeKind::Pointer(Box::new(Type {
                                kind: TypeKind::Void,
                                span: identifier.span,
                            })),
                            span: identifier.span,
                        },
                    },
                    CheckedParam {
                        identifier,
                        ty: Type {
                            kind: TypeKind::Pointer(Box::new(env_struct_type)),
                            span: identifier.span,
                        },
                    },
                ]),
                span: identifier.span,
            };
            let closure_obj_ptr = self.emit_stack_alloc(ctx, closure_obj_type, 1);
            let fn_ptr_field = self.emit_get_field_ptr_by_index(
                ctx,
                closure_obj_ptr,
                0,
                Type {
                    kind: TypeKind::Void,
                    span: identifier.span,
                },
            );
            let fn_addr_val = Value::FunctionAddr {
                function_id: new_function_id,
                ty: Type {
                    kind: TypeKind::Void,
                    span: identifier.span,
                },
            };
            self.emit_store(ctx, fn_ptr_field, fn_addr_val);
            let env_ptr_field = self.emit_get_field_ptr_by_index(
                ctx,
                closure_obj_ptr,
                1,
                Type {
                    kind: TypeKind::Void,
                    span: identifier.span,
                },
            );
            self.emit_store(ctx, env_ptr_field, Value::Use(env_ptr));

            let closure_type = Type {
                kind: TypeKind::FnType(CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                    convention: CallingConvention::Closure,
                }),
                span: identifier.span,
            };
            ctx.program_builder
                .value_types
                .insert(closure_obj_ptr, closure_type);

            Value::Use(closure_obj_ptr)
        }
    }
}
