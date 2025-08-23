use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use string_interner::StringInterner;

pub mod string_interner;

use crate::{
    hir_builder::{errors::SemanticError, types::checked_type::TypeKind, utils::type_to_string::type_to_string, HIRBuilder},
    parse::{Parser, ParsingErrorKind},
    tokenize::{token_kind_to_string, TokenizationErrorKind, Tokenizer},
};

pub fn compile_file<'a, 'b>(
    file_path: &'a str,
    source_code: &'a str,
    string_interner: &'b mut StringInterner<'a>,
    files: &mut SimpleFiles<usize, String>,
) {
    let interned_fp = string_interner.intern(file_path).0;
    files.add(interned_fp, source_code.to_string());

    let (tokens, tokenization_errors) = Tokenizer::tokenize(source_code);
    let (ast, parsing_errors) = Parser::parse(tokens, string_interner);
    let (analyzed_tree, semantic_errors) = HIRBuilder::build(ast);

    let mut errors: Vec<Diagnostic<usize>> = vec![];

    tokenization_errors.iter().for_each(|e| {
        let span = e.span.start.byte_offset..e.span.end.byte_offset;
        let label = Label::primary(interned_fp, span);
        let err = Diagnostic::error().with_code(format!("T{}", e.kind.code()));

        let diagnostic = match &e.kind {
            TokenizationErrorKind::UnterminatedString => err
                .with_message("Unterminated string")
                .with_label(label.with_message("This string is not terminated")),
            TokenizationErrorKind::UnknownToken => err
                .with_message("Unknown token")
                .with_label(label.with_message("This token is not recognized")),
            TokenizationErrorKind::UnknownEscapeSequence => err
                .with_message("Unknown escape sequence")
                .with_label(label.with_message("The escape sequence here is invalid")),
            TokenizationErrorKind::InvalidFloatingNumber => err
                .with_message("Invalid floating-point number")
                .with_label(label.with_message("This is not a valid floating-point number")),
            TokenizationErrorKind::InvalidIntegerNumber => err
                .with_message("Invalid integer number")
                .with_label(label.with_message("This is not a valid integer number")),
            TokenizationErrorKind::UnterminatedDoc => err
                .with_message("Unterminated documentation")
                .with_label(label.with_message("This documentation block is not terminated")),
        };

        errors.push(diagnostic);
    });

    parsing_errors.iter().for_each(|e| {
        let span = e.span.start.byte_offset..e.span.end.byte_offset;
        let label = Label::primary(interned_fp, span);
        let err = Diagnostic::error().with_code(format!("P{}", e.kind.code()));

        let diagnostic = match &e.kind {
            ParsingErrorKind::DocMustBeFollowedByDeclaration => err
                .with_message("Documentation must be followed by a declaration of ")
                .with_label(
                    label.with_message("This documentation must be followed by a declaration of a type alias or a variable"),
                ),
            ParsingErrorKind::ExpectedAnExpressionButFound(token) => {
                err.with_message("Expected an expression")
                    .with_label(label.with_message(format!(
                        "Expected an expression but instead found token \"{}\"",
                        token_kind_to_string(&token.kind)
                    )))
            }
            ParsingErrorKind::ExpectedATypeButFound(token) => {
                err.with_message("Expected a type").with_label(label.with_message(format!(
                    "Expected a type but instead found token \"{}\"",
                    token_kind_to_string(&token.kind)
                )))
            }
            ParsingErrorKind::InvalidSuffixOperator(token) => {
                err.with_message("Invalid suffix operator")
                    .with_label(label.with_message(format!(
                        "Invalid token as expression suffix operator \"{}\"",
                        token_kind_to_string(&token.kind)
                    )))
            }
            ParsingErrorKind::UnexpectedEndOfInput => err
                .with_message("Unexpected end of input")
                .with_label(label.with_message("Unexpected end of input")),
            ParsingErrorKind::ExpectedAnIdentifier => err
                .with_message("Expected an identifier")
                .with_label(label.with_message("Expected an identifier")),
            ParsingErrorKind::ExpectedAPunctuationMark(punctuation_kind) => err
                .with_message("Expected a punctuation mark")
                .with_label(label.with_message(format!("Expected the \"{}\" punctuation mark", punctuation_kind.to_string()))),
            ParsingErrorKind::ExpectedAKeyword(keyword_kind) => err
                .with_message("Expected a keyword")
                .with_label(label.with_message(format!("Expected the \"{}\" keyword", keyword_kind.to_string()))),
            ParsingErrorKind::ExpectedAStringValue => err
                .with_message("Expected a string literal")
                .with_label(label.with_message("Expected a string literal")),
            ParsingErrorKind::ExpectedANumericValue => err
                .with_message("Expected a numeric literal")
                .with_label(label.with_message("Expected a numeric literal")),
            ParsingErrorKind::UnknownStaticMethod(identifier_node) => {
                let name = string_interner.resolve(identifier_node.name).unwrap();
                err.with_message("Unknown static method")
                    .with_label(label.with_message(format!("Static method with name \"{}\" doesn't exist", name)))
            }
            ParsingErrorKind::UnexpectedStatementAfterFinalExpression => err
                .with_message("Unexpected statement after final expression")
                .with_label(label.with_message("Final expression of a codeblock must not be followed by another statement")),
            ParsingErrorKind::ExpectedStatementOrExpression { found } => err
                .with_message("Expected a statement or an expression")
                .with_label(label.with_message(format!(
                    "Expected a statement or an expression but instead found token \"{}\"",
                    token_kind_to_string(found)
                ))),
            ParsingErrorKind::UnexpectedTokenAfterFinalExpression { found } => err
                .with_message("Unexpected token after final expression")
                .with_label(label.with_message(format!(
                    "Unexpected token \"{}\" after final expression",
                    token_kind_to_string(found)
                ))),
        };

        errors.push(diagnostic);
    });

    semantic_errors.into_iter().for_each(|e| {
        let span = e.span();
        let error_span = span.start.byte_offset..span.end.byte_offset;
        let label = Label::primary(interned_fp, error_span);
        let err = Diagnostic::error().with_code(format!("S{}", e.code()));

        let diagnostic = match &e {
            SemanticError::ExpectedANumericOperand { .. } => err
                .with_message("Expected a numeric operand")
                .with_label(label.with_message("Expected this value to have a numeric type")),
            SemanticError::MixedSignedAndUnsigned { .. } => err
                .with_message("Mixed signed and unsigned operands")
                .with_label(label.with_message("Mixing signed and unsigned operands in an arithmetic operation is not allowed")),
            SemanticError::MixedFloatAndInteger { .. } => err.with_message("Mixed float and integer operands").with_label(
                label.with_message("Mixing integer and floating-point numbers in an arithmetic operation is not allowed"),
            ),
            SemanticError::CannotCompareType { of, to } => {
                err.with_message("Cannot compare types")
                    .with_label(label.with_message(format!(
                        "Cannot compare type \"{}\" to type \"{}\"",
                        type_to_string(&of.kind, string_interner),
                        type_to_string(&to.kind, string_interner)
                    )))
            }
            SemanticError::UndeclaredIdentifier { id } => {
                let name = string_interner.resolve(id.name).unwrap();

                err.with_message("Undeclared identifier")
                    .with_label(label.with_message(format!("Undeclared identifier \"{}\"", name)))
            }
            SemanticError::UndeclaredType { id } => {
                let name = string_interner.resolve(id.name).unwrap();

                err.with_message("Undeclared type")
                    .with_label(label.with_message(format!("Undeclared type \"{}\"", name)))
            }
            SemanticError::ReturnKeywordOutsideFunction { .. } => err
                .with_message("Keyword \"return\" used outside of a function scope")
                .with_label(label.with_message("Cannot use the \"return\" keyword outside of a function scope")),
            SemanticError::BreakKeywordOutsideLoop { .. } => err
                .with_message("Keyword \"break\" used outside of a loop scope")
                .with_label(label.with_message("Cannot use the \"break\" keyword outside of a loop scope")),
            SemanticError::ContinueKeywordOutsideLoop { .. } => err
                .with_message("Keyword \"continue\" used outside of a loop scope")
                .with_label(label.with_message("Cannot use the \"continue\" keyword outside of a loop scope")),
            SemanticError::InvalidAssignmentTarget { .. } => err
                .with_message("Invalid assignment target")
                .with_label(label.with_message("Invalid assignment target")),
            SemanticError::TypeMismatch { expected, received } => {
                let constraint_str = type_to_string(&expected.kind, string_interner);
                let declaration_of_expected = expected.span.start.byte_offset..expected.span.end.byte_offset;

                err.with_message("Type mismatch").with_labels(vec![
                    label.with_message(format!(
                        "Type mismatch, expected `{}`, instead found `{}`",
                        constraint_str,
                        type_to_string(&received.kind, string_interner)
                    )),
                    Label::secondary(interned_fp, declaration_of_expected)
                        .with_message(format!("expected type \"{}\" originated here", constraint_str)),
                ])
            }
            SemanticError::InvalidArraySizeValue { value, .. } => err
                .with_message("Invalid array size")
                .with_label(label.with_message(format!("Invalid array size: {}", value.to_string()))),
            SemanticError::ReturnNotLastStatement { .. } => err
                .with_message("Expected the return statement to be the last statement in the function")
                .with_label(label.with_message("Expected the return statement to be the last statement in the function")),
            SemanticError::ReturnTypeMismatch { expected, received } => {
                err.with_message("Return type mismatch")
                    .with_label(label.with_message(format!(
                        "Expected the return value to be assignable to {}, found {}",
                        type_to_string(&expected.kind, string_interner),
                        type_to_string(&received.kind, string_interner)
                    )))
            }
            SemanticError::CannotAccess { target } => {
                err.with_message("Cannot access field").with_label(label.with_message(format!(
                    "Cannot use the access operator on the type \"{}\"",
                    type_to_string(&target.kind, string_interner)
                )))
            }
            SemanticError::CannotCall { target } => {
                err.with_message("Cannot use the function call operator")
                    .with_label(label.with_message(format!(
                        "Cannot use the function-call operator on type \"{}\"",
                        type_to_string(&target.kind, string_interner)
                    )))
            }
            SemanticError::FnArgumentCountMismatch { expected, received, .. } => {
                let s = if *expected > 1 { "s" } else { "" };
                err.with_message("Function argument count mismatch")
                    .with_label(label.with_message(format!(
                        "This function expects {} argument{}, but instead received {}",
                        expected.to_string(),
                        s,
                        received.to_string()
                    )))
            }

            SemanticError::CannotUseVariableDeclarationAsType { .. } => err
                .with_message("Cannot use variable declaration as a type")
                .with_label(label.with_message("Cannot use variable declaration as a type")),
            SemanticError::AccessToUndefinedField { field } => {
                let name = string_interner.resolve(field.name).unwrap();
                err.with_message("Access to an undefined field")
                    .with_label(label.with_message(format!("Field {} is not defined", name)))
            }
            SemanticError::TypeAliasMustBeDeclaredAtTopLevel { .. } => err
                .with_message("Type aliases must be declared in the file scope")
                .with_label(label.with_message("Type aliases must be declared in the file scope")),
            SemanticError::StructMustBeDeclaredAtTopLevel { .. } => err
                .with_message("Structs must be declared in the file scope")
                .with_label(label.with_message("Structs must be declared in the file scope")),
            SemanticError::CannotApplyTypeArguments { to } => {
                err.with_message("Cannot apply type arguments")
                    .with_label(label.with_message(format!(
                        "Cannot apply type arguments to non-generic type {}",
                        type_to_string(&to.kind, string_interner)
                    )))
            }
            SemanticError::DuplicateStructFieldInitializer { id } => {
                let name = string_interner.resolve(id.name).unwrap();
                err.with_message("Duplicate initializer for a struct field")
                    .with_label(label.with_message(format!("Struct field \"{}\" cannot be initialized multiple times", name)))
            }
            SemanticError::UnknownStructFieldInitializer { id } => {
                let name = string_interner.resolve(id.name).unwrap();
                err.with_message("Unknown field in the struct initializer")
                    .with_label(label.with_message(format!("Unknown struct field \"{}\"", name)))
            }
            SemanticError::MissingStructFieldInitializer { missing_fields, .. } => {
                let field_names: Vec<&'a str> = missing_fields
                    .into_iter()
                    .map(|f| string_interner.resolve(*f).unwrap())
                    .collect();
                let joined = field_names
                    .iter()
                    .map(|n| format!("\"{}\"", n))
                    .collect::<Vec<String>>()
                    .join(", ");
                err.with_message("Missing field initializers")
                    .with_label(label.with_message(format!("Missing initializers for the following struct fields {}", joined)))
            }
            SemanticError::CannotApplyStructInitializer { .. } => err
                .with_message("Cannot apply struct initializer")
                .with_label(label.with_message("Cannot apply struct initializer to this expression")),
            SemanticError::VarDeclWithoutInitializer { .. } => err
                .with_message("Variable declarations must have an initializer")
                .with_label(label.with_message("This variable declaration must have an initializer")),
            SemanticError::DuplicateIdentifier { id } => {
                let identifier_name = string_interner.resolve(id.name).unwrap();
                err.with_message("Duplicate identifier")
                    .with_label(label.with_message(format!("Duplicate identifier declaration \"{}\"", identifier_name)))
            }
            SemanticError::ExpectedTypeArguments { .. } => err
                .with_message("Expected type argument")
                .with_label(label.with_message("Expected type arguments to be applied to this type")),
        };

        errors.push(diagnostic);
    });

    let writer = StandardStream::stderr(ColorChoice::Always);
    let mut config = codespan_reporting::term::Config::default();

    config.start_context_lines = 8;
    config.end_context_lines = 8;
    config.before_label_lines = 8;
    config.after_label_lines = 8;

    if !errors.is_empty() {
        println!();
        for diagnostic in errors {
            let _ = term::emit(&mut writer.lock(), &config, files, &diagnostic);
        }
    } else {
        println!("Compilation successful for {} (no errors found).", file_path);
    }
}
