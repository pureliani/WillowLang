use crate::{cfg::Value, hir_builder::HIRBuilder};

impl<'a> HIRBuilder<'a> {
    pub fn build_bool_literal(&mut self, value: bool) -> Value {
        Value::BoolLiteral(value)
    }
}
