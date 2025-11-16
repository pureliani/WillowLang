use std::sync::{Arc, RwLock};

use crate::{
    ast::{decl::FnDecl, expr::BlockContents, IdentifierNode},
    hir::{
        cfg::{CheckedDeclaration, Terminator, Value},
        errors::{SemanticError, SemanticErrorKind},
        types::{
            checked_declaration::{
                CheckedClosureType, CheckedFnDecl, CheckedFnType, CheckedParam,
                CheckedVarDecl, VarStorage,
            },
            checked_type::{Type, TypeKind},
        },
        utils::{
            pack_struct::pack_struct, scope::ScopeKind,
            var_capture_analyzer::analyze_captures,
        },
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

        let captures_map = analyze_captures(ctx, &checked_params, &body);

        if captures_map.is_empty() {
            let new_function_id = ctx.program_builder.new_function_id();
            let checked_fn_decl = Arc::new(RwLock::new(CheckedFnDecl {
                id: ctx.program_builder.new_declaration_id(),
                function_id: new_function_id,
                identifier,
                params: checked_params.clone(),
                return_type: checked_return_type.clone(),
                body: None,
            }));

            ctx.module_builder
                .functions
                .insert(new_function_id, checked_fn_decl.clone());

            ctx.module_builder.scope_insert(
                identifier,
                CheckedDeclaration::Function(new_function_id),
                identifier.span,
            );

            ctx.module_builder.enter_scope(ScopeKind::Function);
            let mut new_fn_builder = FunctionBuilder::new(checked_return_type.clone());
            new_fn_builder.build_fn_body(ctx, &checked_params, body);
            checked_fn_decl.write().unwrap().body = Some(new_fn_builder.cfg);
            ctx.module_builder.exit_scope();

            let fn_type = Type {
                kind: TypeKind::FnType(CheckedFnType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                }),
                span: identifier.span,
            };

            Value::FunctionAddr {
                function_id: new_function_id,
                ty: fn_type,
            }
        } else {
            // 1. Find captured variables
            let mut captures: Vec<CheckedParam> = Vec::new();
            for (id_node, ty) in captures_map {
                captures.push(CheckedParam {
                    identifier: id_node,
                    ty,
                });
                let decl_to_update =
                    ctx.module_builder.scope_lookup_mut(id_node.name).unwrap();
                if let CheckedDeclaration::Var(var_decl) = decl_to_update {
                    var_decl.storage = VarStorage::Captured;
                } else {
                    panic!(
                        "INTERNAL COMPILER ERROR: Captured a non-variable declaration."
                    );
                }
            }
            pack_struct(ctx, &mut captures);

            // 2. Instantiate env struct on heap with captured variables
            let env_struct_type = Type {
                kind: TypeKind::Struct(captures),
                span: identifier.span,
            };
            let env_ptr = self
                .emit_heap_alloc(
                    ctx,
                    env_struct_type.clone(),
                    Value::NumberLiteral(NumberKind::USize(1)),
                )
                .unwrap();

            if let TypeKind::Struct(canonical_env_fields) = &env_struct_type.kind {
                for captured_var in canonical_env_fields {
                    let field_ptr = self
                        .emit_get_field_ptr(ctx, env_ptr, captured_var.identifier)
                        .unwrap();
                    let captured_value =
                        self.build_identifier_expr(ctx, captured_var.identifier);
                    self.emit_store(ctx, field_ptr, captured_value);
                }
            }

            // 3. Create the new function with additional env parameter
            let env_param_id = IdentifierNode {
                name: ctx.program_builder.env_ptr_field_name,
                span: identifier.span,
            };
            let mut closure_params = vec![CheckedParam {
                identifier: env_param_id,
                ty: Type {
                    kind: TypeKind::Pointer(Box::new(env_struct_type.clone())),
                    span: identifier.span,
                },
            }];
            closure_params.extend_from_slice(&checked_params);

            let new_function_id = ctx.program_builder.new_function_id();
            let checked_fn_decl = Arc::new(RwLock::new(CheckedFnDecl {
                id: ctx.program_builder.new_declaration_id(),
                function_id: new_function_id,
                identifier,
                params: closure_params.clone(),
                return_type: checked_return_type.clone(),
                body: None,
            }));
            ctx.module_builder
                .functions
                .insert(new_function_id, checked_fn_decl.clone());

            ctx.module_builder.enter_scope(ScopeKind::Function);
            let mut fn_builder = FunctionBuilder::new(checked_return_type.clone());
            fn_builder.build_fn_body(ctx, &closure_params, body);
            checked_fn_decl.write().unwrap().body = Some(fn_builder.cfg);
            ctx.module_builder.exit_scope();

            // 4. Create the closure object { __fn_ptr, __env_ptr } on stack
            let fn_ptr_id = IdentifierNode {
                name: ctx.program_builder.fn_ptr_field_name,
                span: identifier.span,
            };
            let mut closure_obj_fields = vec![
                CheckedParam {
                    identifier: fn_ptr_id,
                    ty: Type {
                        kind: TypeKind::Pointer(Box::new(Type {
                            kind: TypeKind::Void,
                            span: identifier.span,
                        })),
                        span: identifier.span,
                    },
                },
                CheckedParam {
                    identifier: env_param_id,
                    ty: Type {
                        kind: TypeKind::Pointer(Box::new(env_struct_type.clone())),
                        span: identifier.span,
                    },
                },
            ];
            pack_struct(ctx, &mut closure_obj_fields);

            let closure_obj_type = Type {
                kind: TypeKind::Struct(closure_obj_fields),
                span: identifier.span,
            };

            let closure_obj_ptr = self.emit_stack_alloc(ctx, closure_obj_type, 1);

            // Populate the fields of the closure object.
            let fn_ptr_field = self
                .emit_get_field_ptr(ctx, closure_obj_ptr, fn_ptr_id)
                .unwrap();
            let fn_addr_val = Value::FunctionAddr {
                function_id: new_function_id,
                ty: Type {
                    kind: TypeKind::Void,
                    span: identifier.span,
                },
            };
            self.emit_store(ctx, fn_ptr_field, fn_addr_val);

            let env_ptr_field = self
                .emit_get_field_ptr(ctx, closure_obj_ptr, env_param_id)
                .unwrap();
            self.emit_store(ctx, env_ptr_field, Value::Use(env_ptr));

            let closure_function_type = Type {
                kind: TypeKind::Closure(CheckedClosureType {
                    params: checked_params,
                    return_type: Box::new(checked_return_type),
                    env_struct_type: Box::new(env_struct_type),
                }),
                span: identifier.span,
            };

            // 5. Declare variable with the same name as user-declared function but make it hold the closure object
            ctx.program_builder
                .value_types
                .insert(closure_obj_ptr, closure_function_type.clone());

            let checked_var_decl = CheckedVarDecl {
                id: ctx.program_builder.new_declaration_id(),
                storage: VarStorage::Stack(closure_obj_ptr),
                identifier,
                documentation: None,
                constraint: closure_function_type,
            };

            ctx.module_builder.scope_insert(
                identifier,
                CheckedDeclaration::Var(checked_var_decl),
                identifier.span,
            );

            Value::Use(closure_obj_ptr)
        }
    }
}
