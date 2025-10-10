pub mod assignment;
pub mod enum_decl;
pub mod from;
pub mod r#return;
pub mod type_alias_decl;
pub mod var_decl;
pub mod r#while;

use crate::{
    ast::{
        expr::ExprKind,
        stmt::{Stmt, StmtKind},
    },
    hir::{
        cfg::Terminator,
        errors::{SemanticError, SemanticErrorKind},
        expressions::r#if::IfContext,
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_statements(&mut self, ctx: &mut HIRContext, statements: Vec<Stmt>) {
        for statement in statements {
            if self.get_current_basic_block().terminator.is_some() {
                ctx.module_builder.errors.push(SemanticError {
                    kind: SemanticErrorKind::UnreachableCode,
                    span: statement.span,
                });
                break;
            }

            match statement.kind {
                StmtKind::Expression(expr) => {
                    if let ExprKind::If {
                        branches,
                        else_branch,
                    } = expr.kind
                    {
                        self.build_if(ctx, branches, else_branch, IfContext::Statement);
                    } else {
                        self.build_expr(ctx, expr);
                    }
                }
                StmtKind::TypeAliasDecl(type_alias_decl) => {
                    self.build_type_alias_decl(ctx, type_alias_decl, statement.span);
                }
                StmtKind::VarDecl(var_decl) => {
                    self.build_var_decl(ctx, var_decl, statement.span)
                }
                StmtKind::Return { value } => todo!(),
                StmtKind::Assignment { target, value } => {
                    self.build_assignment_stmt(ctx, target, value)
                }
                StmtKind::From { path, identifiers } => todo!(),
                StmtKind::While { condition, body } => {
                    self.build_while_stmt(ctx, condition, body);
                }
                StmtKind::Break => {
                    if let Some((_, break_target)) =
                        ctx.module_builder.within_loop_scope()
                    {
                        self.set_basic_block_terminator(Terminator::Jump {
                            target: break_target,
                        });
                    } else {
                        ctx.module_builder.errors.push(SemanticError {
                            kind: SemanticErrorKind::BreakKeywordOutsideLoop,
                            span: statement.span,
                        });
                    }
                }
                StmtKind::Continue => {
                    if let Some((continue_target, _)) =
                        ctx.module_builder.within_loop_scope()
                    {
                        self.set_basic_block_terminator(Terminator::Jump {
                            target: continue_target,
                        });
                    } else {
                        ctx.module_builder.errors.push(SemanticError {
                            kind: SemanticErrorKind::ContinueKeywordOutsideLoop,
                            span: statement.span,
                        });
                    }
                }
                StmtKind::EnumDecl(enum_decl) => todo!(),
            }
        }
    }
}
