use crate::hir::{cfg::Value, FunctionBuilder};

impl FunctionBuilder {
    pub fn build_bool_literal(&mut self, value: bool) -> Value {
        Value::BoolLiteral(value)
    }
}
