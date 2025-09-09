use crate::{cfg::Value, hir_builder::FunctionBuilder, tokenize::NumberKind};

impl FunctionBuilder {
    pub fn build_number_literal(&mut self, value: NumberKind) -> Value {
        Value::NumberLiteral(value)
    }
}
