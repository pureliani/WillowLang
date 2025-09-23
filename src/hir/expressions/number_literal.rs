use crate::{
    hir::{cfg::Value, FunctionBuilder},
    tokenize::NumberKind,
};

impl FunctionBuilder {
    pub fn build_number_literal(&mut self, value: NumberKind) -> Value {
        Value::NumberLiteral(value)
    }
}
