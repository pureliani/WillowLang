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
    ast::checked::checked_type::CheckedTypeKind,
    check::{utils::type_to_string::type_to_string, SemanticChecker, SemanticError},
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
    let (analyzed_tree, semantic_errors) = SemanticChecker::check(ast);

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
                .with_label(label.with_message(
                    "This documentation must be followed by a declaration of either struct, type alias, enum or a variable",
                )),
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
                    "Unexpected token after final expression \"{}\"",
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
                err.with_message("Type mismatch").with_label(label.with_message(format!(
                    "Type mismatch, expected {} but instead found {}",
                    type_to_string(&expected.kind, string_interner),
                    type_to_string(&received.kind, string_interner)
                )))
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
            SemanticError::CannotCall { target } => err.with_message("Cannot call").with_label(label.with_message(format!(
                "Cannot use the call operator on the type \"{}\"",
                type_to_string(&target.kind, string_interner)
            ))),
            SemanticError::FnArgumentCountMismatch { expected, received, .. } => err
                .with_message("Function argument count mismatch")
                .with_label(label.with_message(format!(
                    "This function expects {} arguments, but instead received {}",
                    expected.to_string(),
                    received.to_string()
                ))),
            SemanticError::GenericArgumentCountMismatch { expected, received, .. } => err
                .with_message("Generic argument count mismatch")
                .with_label(label.with_message(format!(
                    "Expected {} type arguments, but instead received {}",
                    expected.to_string(),
                    received.to_string()
                ))),
            SemanticError::CannotUseGenericParameterAsValue { .. } => err
                .with_message("Cannot use generic parameters as values")
                .with_label(label.with_message("Cannot use generic parameter where an expression is expected")),
            SemanticError::CannotUseVariableDeclarationAsType { .. } => err
                .with_message("Cannot use variable declaration as a type")
                .with_label(label.with_message("Cannot use variable declaration as a type")),
            SemanticError::AccessToUndefinedField { field } => {
                let name = string_interner.resolve(field.name).unwrap();
                err.with_message("Access to an undefined field")
                    .with_label(label.with_message(format!("Field {} is not defined", name)))
            }
            SemanticError::UnresolvedGenericParam { param } => {
                let name = string_interner.resolve(param.name).unwrap();
                err.with_message("Unresolved generic parameter")
                    .with_label(label.with_message(format!("Could not resolve generic parameter with name \"{}\"", name)))
            }
            SemanticError::CannotUseIsTypeOnNonUnion { .. } => err
                .with_message("Cannot use the \"::is(T)\" method on a non-union type")
                .with_label(label.with_message("Cannot use the \"::is(T)\" method on a non-union type")),
            SemanticError::ConflictingGenericBinding {
                generic_param,
                existing,
                new,
            } => {
                let name = string_interner.resolve(generic_param.identifier.name).unwrap();

                err.with_message("Conflicting generic binding")
                    .with_label(label.with_message(format!(
                        "Generic parameter identifier {} is already bound to type {}, cannot re-bind it to {}",
                        name,
                        type_to_string(&existing.kind, string_interner),
                        type_to_string(&new.kind, string_interner)
                    )))
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
            SemanticError::CouldNotSubstituteGenericParam {
                generic_param,
                with_type,
            } => {
                let gp_name = string_interner.resolve(generic_param.identifier.name).unwrap();

                let gp_string = match &generic_param.constraint {
                    Some(c) => {
                        format!("{}: {}", gp_name, type_to_string(&c.kind, string_interner))
                    }
                    None => {
                        format!("{}", gp_name)
                    }
                };

                let gp_span = generic_param.identifier.span.start.byte_offset..generic_param.identifier.span.end.byte_offset;
                let argument_span = with_type.span.start.byte_offset..with_type.span.end.byte_offset;

                err.with_message("Could not substitute generic param").with_labels(vec![
                    Label::primary(interned_fp, gp_span).with_message(format!(
                        "Could not substitute generic param \"{}\" with type \"{}\"",
                        gp_string,
                        type_to_string(&with_type.kind, string_interner)
                    )),
                    Label::primary(interned_fp, argument_span).with_message("type argument provided here"),
                ])
            }
            SemanticError::AmbiguousGenericInferenceForUnion { received, expected } => err
                .with_message("Ambiguous generic inference for union")
                .with_label(label.with_message(format!(
                    "There are multiple ways to infer generic parameters in \"{}\" from \"{}\"",
                    type_to_string(&CheckedTypeKind::Union(expected.clone()), string_interner),
                    type_to_string(&received.kind, string_interner)
                ))),
            SemanticError::FailedToInferGenericsInUnion {
                expected_union,
                received,
            } => err
                .with_message("Failed to infer generic parameters in union")
                .with_label(label.with_message(format!(
                    "Failed to infer generic parameters in \"{}\" from {}",
                    type_to_string(&CheckedTypeKind::Union(expected_union.clone()), string_interner),
                    type_to_string(&received.kind, string_interner)
                ))),
        };

        errors.push(diagnostic);
    });

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    if !errors.is_empty() {
        println!();
        for (index, diagnostic) in errors.into_iter().enumerate() {
            let _ = term::emit(&mut writer.lock(), &config, files, &diagnostic);
        }
    } else {
        println!("Compilation successful for {} (no errors found).", file_path);
    }
}
