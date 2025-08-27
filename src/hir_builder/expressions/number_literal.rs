use crate::{cfg::Value, hir_builder::HIRBuilder, tokenize::NumberKind};

impl<'a> HIRBuilder<'a> {
    pub fn build_number_literal(&mut self, value: NumberKind) -> Value {
        Value::NumberLiteral(value)
    }
}
