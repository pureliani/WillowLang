use crate::{
    ast::{
        decl::Param, expr::BlockContents, type_annotation::TypeAnnotation, IdentifierNode,
    },
    hir::{cfg::Value, FunctionBuilder, HIRContext},
};

impl FunctionBuilder {
    pub fn build_fn_expr(
        &mut self,
        ctx: &mut HIRContext,
        params: Vec<Param>,
        body: BlockContents,
        return_type: TypeAnnotation,
        name: IdentifierNode,
    ) -> Value {
        todo!()
    }
}
