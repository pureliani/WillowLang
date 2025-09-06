use crate::{
    cfg::Value,
    hir_builder::{
        errors::SemanticError,
        types::checked_type::{Type, TypeKind},
        HIRBuilder,
    },
};

impl<'a> HIRBuilder<'a> {
    /// Records a semantic error and returns a new "poison" Value of type Unknown.
    /// The caller is responsible for immediately returning the poison Value.
    pub fn report_error_and_get_poison(&mut self, error: SemanticError) -> Value {
        let error_span = error.span;
        self.errors.push(error);
        let unknown_result_id = self.new_value_id();
        self.cfg.value_types.insert(
            unknown_result_id,
            Type {
                kind: TypeKind::Unknown,
                span: error_span,
            },
        );
        Value::Use(unknown_result_id)
    }
}
