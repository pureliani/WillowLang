use crate::{
    ast::expr::{Expr, MatchArm},
    cfg::{BasicBlockId, Value},
    hir_builder::{
        errors::{SemanticError, SemanticErrorKind},
        types::checked_type::{Type, TypeKind},
        FunctionBuilder, HIRContext,
    },
};

impl FunctionBuilder {
    pub fn build_match_expr(&mut self, ctx: &mut HIRContext, condition: Box<Expr>, arms: Vec<MatchArm>) -> Value {
        let condition_value = self.build_expr(ctx, *condition);
        let condition_type = self.get_value_type(&condition_value);

        let union_decl = if let TypeKind::Union(decl) = condition_type.kind {
            decl
        } else {
            return Value::Use(self.report_error_and_get_poison(
                ctx,
                SemanticError {
                    kind: SemanticErrorKind::ExpectedUnionType,
                    span: condition_type.span,
                },
            ));
        };

        let merge_block_id = self.new_basic_block();
        let mut phi_sources: Vec<(BasicBlockId, Value, Type)> = Vec::new();

        todo!()
    }
}
