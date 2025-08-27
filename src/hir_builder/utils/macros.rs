/// Checks a condition. If the condition is false, it pushes the provided
/// semantic error, creates and returns a new "poison" Value of type Unknown,
/// and exits the current function.
#[macro_export]
macro_rules! ensure {
    ($self:ident, $condition:expr, $error_expr:expr) => {
        // The macro takes the HIRBuilder (`self`), a boolean condition,
        // and an expression that creates the SemanticError.
        if !$condition {
            let error = $error_expr;
            let error_span = error.span();
            $self.errors.push(error);

            let unknown_result = $self.new_value_id();
            $self.cfg.value_types.insert(
                unknown_result,
                crate::hir_builder::types::checked_type::Type {
                    kind: crate::hir_builder::types::checked_type::TypeKind::Unknown,
                    span: error_span,
                },
            );
            return crate::cfg::Value::Use(unknown_result);
        }
    };
}
