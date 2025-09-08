use crate::{cfg::Value, hir_builder::FunctionBuilder, tokenize::NumberKind};

impl<'a> FunctionBuilder<'a> {
    pub fn build_number_literal(&mut self, value: NumberKind) -> Value {
        Value::NumberLiteral(value)
    }
}
