use crate::{cfg::Value, hir_builder::FunctionBuilder};

impl<'a> FunctionBuilder<'a> {
    pub fn build_bool_literal(&mut self, value: bool) -> Value {
        Value::BoolLiteral(value)
    }
}
