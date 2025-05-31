use ariadne::{Cache, Color, Label, Report, ReportKind, Source};
use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};
use string_interner::StringInterner;
pub mod string_interner;

use crate::{
    check::{
        check_stmts::check_stmts,
        scope::{Scope, ScopeKind},
        utils::type_to_string::type_to_string,
        SemanticError, SemanticErrorKind,
    },
    parse::{Parser, ParsingErrorKind},
    tokenize::{token_kind_to_string, TokenizationErrorKind, Tokenizer},
};

struct FileSourceCache {
    sources: HashMap<String, Source>,
}

impl FileSourceCache {
    fn new() -> Self {
        FileSourceCache {
            sources: HashMap::new(),
        }
    }
    fn add(&mut self, id: String, source_str: String) {
        self.sources.insert(id, Source::from(source_str));
    }
}

impl Cache<String> for FileSourceCache {
    type Storage = String;

    fn fetch(&mut self, id: &String) -> Result<&Source<Self::Storage>, impl std::fmt::Debug> {
        self.sources
            .get(id)
            .ok_or_else(|| format!("Source not found: {}", id))
    }

    fn display<'a>(&self, id: &'a String) -> Option<impl std::fmt::Display + 'a> {
        Some(Box::new(id.clone()))
    }
}

pub fn compile_file<'a, 'b>(
    file_path: &String,
    source_code: &'a String,
    string_interner: &'b mut StringInterner<'a>,
) {
    let mut reports: Vec<Report<'_, (String, std::ops::Range<usize>)>> = vec![];
    let (tokens, tokenization_errors) = Tokenizer::tokenize(source_code);
    let (ast, parsing_errors) = Parser::parse(tokens, string_interner);
    let mut semantic_errors: Vec<SemanticError> = vec![];
    let scope = Rc::new(RefCell::new(Scope::new(ScopeKind::File)));
    let analyzed_tree = check_stmts(ast, &mut semantic_errors, scope);

    tokenization_errors.iter().for_each(|e| {
        let error_span = (
            file_path.clone(),
            e.span.start.byte_offset..e.span.end.byte_offset,
        );
        let report_builder = Report::build(ReportKind::Error, error_span.clone())
            .with_code(format!("T{}", e.kind.code()));

        let report = match &e.kind {
            TokenizationErrorKind::UnterminatedString => report_builder
                .with_message("Unterminated string")
                .with_label(
                    Label::new(error_span)
                        .with_message("This string is not terminated")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::UnknownToken => report_builder
                .with_message("Unknown token")
                .with_label(
                    Label::new(error_span)
                        .with_message("This token is not recognized")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::UnknownEscapeSequence => report_builder
                .with_message("Unknown escape sequence")
                .with_label(
                    Label::new(error_span)
                        .with_message("The escape sequence here is invalid")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::InvalidFloatingNumber => report_builder
                .with_message("Invalid floating-point number")
                .with_label(
                    Label::new(error_span)
                        .with_message("This is not a valid floating-point number")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::InvalidIntegerNumber => report_builder
                .with_message("Invalid integer number")
                .with_label(
                    Label::new(error_span)
                        .with_message("This is not a valid integer number")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::UnterminatedDoc => report_builder
                .with_message("Unterminated documentation")
                .with_label(
                    Label::new(error_span)
                        .with_message("This documentation block is not terminated")
                        .with_color(Color::Red),
                )
                .finish(),
        };

        reports.push(report);
    });

    parsing_errors.iter().for_each(|e| {
        let error_span = (
            file_path.clone(),
            e.span.start.byte_offset..e.span.end.byte_offset,
        );

        let report_builder = Report::build(ReportKind::Error, error_span.clone())
            .with_code(format!("P{}", e.kind.code()));

        let report = match &e.kind {
            ParsingErrorKind::DocMustBeFollowedByDeclaration => {
                report_builder
                .with_message("Documentation must be followed by a declaration of ")
                .with_label(
                    Label::new(error_span)
                        .with_message("This documentation must be followed by a declaration of either struct, type alias, enum or a variable")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAnExpressionButFound(token) => {
                report_builder
                .with_message("Expected an expression")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Expected an expression but instead found token \"{}\"", token_kind_to_string(&token.kind)))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedATypeButFound(token) => {
                report_builder
                .with_message("Expected a type")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Expected a type but instead found token \"{}\"", token_kind_to_string(&token.kind)))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::InvalidSuffixOperator(token) => {
                report_builder
                .with_message("Invalid suffix operator")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Invalid token as expression suffix operator \"{}\"", token_kind_to_string(&token.kind)))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::UnexpectedEndOfInput => {
                report_builder
                .with_message("Unexpected end of input")
                .with_label(
                    Label::new(error_span)
                        .with_message("Unexpected end of input")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAnIdentifier => {
                report_builder
                .with_message("Expected an identifier")
                .with_label(
                    Label::new(error_span)
                        .with_message("Expected an identifier")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAPunctuationMark(punctuation_kind) => {
                report_builder
                .with_message("Expected a punctuation mark")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Expected the \"{}\" punctuation mark", punctuation_kind.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAKeyword(keyword_kind) => {
                report_builder
                .with_message("Expected a keyword")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Expected the \"{}\" keyword", keyword_kind.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAStringValue => {
                report_builder
                .with_message("Expected a string literal")
                .with_label(
                    Label::new(error_span)
                        .with_message("Expected a string literal")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedANumericValue => {
                report_builder
                .with_message("Expected a numeric literal")
                .with_label(
                    Label::new(error_span)
                        .with_message("Expected a numeric literal")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::UnknownStaticMethod(identifier_node) => {
                let name = string_interner.resolve(identifier_node.name).unwrap();
                report_builder
                .with_message("Unknown static method")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Static method with name \"{}\" doesn't exist", name))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::UnexpectedStatementAfterFinalExpression => {
                report_builder
                .with_message("Unexpected statement after final expression")
                .with_label(
                    Label::new(error_span)
                        .with_message("Final expression of a codeblock must not be followed by another statement")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedStatementOrExpression { found } => {
                report_builder
                .with_message("Expected a statement or an expression")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Expected a statement or an expression but instead found token \"{}\"", token_kind_to_string(found)))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::UnexpectedTokenAfterFinalExpression { found } => {
                report_builder
                .with_message("Unexpected token after final expression")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Unexpected token after final expression \"{}\"", token_kind_to_string(found)))
                        .with_color(Color::Red),
                )
                .finish()
            }
        };

        reports.push(report);
    });

    semantic_errors.into_iter().for_each(|e| {
        let error_span = (
            file_path.clone(),
            e.span.start.byte_offset..e.span.end.byte_offset,
        );

        let report_builder = Report::build(ReportKind::Error, error_span.clone())
            .with_code(format!("S{}", e.kind.code()));

        let report = match &e.kind {
            SemanticErrorKind::ExpectedANumericOperand => report_builder
                .with_message("Expected a numeric operand")
                .with_label(
                    Label::new(error_span)
                        .with_message("Expected this value to have a numeric type")
                        .with_color(Color::Red),
                )
                .finish(),
            SemanticErrorKind::MixedSignedAndUnsigned => report_builder
                .with_message("Mixed signed and unsigned operands")
                .with_label(
                    Label::new(error_span)
                        .with_message(
                            "Mixing signed and unsigned operands in an arithmetic operation is not allowed",
                        )
                        .with_color(Color::Red),
                )
                .finish(),
            SemanticErrorKind::MixedFloatAndInteger => {
                report_builder
                .with_message("Mixed float and integer operands")
                .with_label(
                    Label::new(error_span)
                        .with_message(
                            "Mixing integer and floating-point numbers in an arithmetic operation is not allowed",
                        )
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::CannotCompareType { of, to } => {
                report_builder
                .with_message("Cannot compare types")
                .with_label(
                    Label::new(error_span)
                        .with_message(
                            format!("Cannot compare type \"{}\" to type \"{}\"", 
                              type_to_string(of, string_interner),
                              type_to_string(to, string_interner)
                            ),
                        )
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::UndeclaredIdentifier(id) => {
                let name = string_interner.resolve(id.name).unwrap();

                report_builder
                .with_message("Undeclared identifier")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Undeclared identifier \"{}\"", name))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::UndeclaredType(id) => {
                let name = string_interner.resolve(id.name).unwrap();

                report_builder
                .with_message("Undeclared type")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Undeclared type \"{}\"", name))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::ReturnKeywordOutsideFunction => {
                report_builder
                .with_message("Keyword \"return\" used outside of a function scope")
                .with_label(
                    Label::new(error_span)
                        .with_message("Cannot use the \"return\" keyword outside of a function scope")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::BreakKeywordOutsideLoop => {
                report_builder
                .with_message("Keyword \"break\" used outside of a loop scope")
                .with_label(
                    Label::new(error_span)
                        .with_message("Cannot use the \"break\" keyword outside of a loop scope")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::ContinueKeywordOutsideLoop => {
                report_builder
                .with_message("Keyword \"continue\" used outside of a loop scope")
                .with_label(
                    Label::new(error_span)
                        .with_message("Cannot use the \"continue\" keyword outside of a loop scope")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::InvalidAssignmentTarget => {
                report_builder
                .with_message("Invalid assignment target")
                .with_label(
                    Label::new(error_span)
                        .with_message("Invalid assignment target")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::TypeMismatch { expected, received } => {
                report_builder
                .with_message("Type mismatch")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Type mismatch, expected {} but instead found {}", 
                          type_to_string(expected, string_interner),
                          type_to_string(received, string_interner)
                        ))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::InvalidArraySizeValue(number_kind) => {
                report_builder
                .with_message("Invalid array size")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Invalid array size: {}", number_kind.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::ReturnNotLastStatement => {
                report_builder
                .with_message("Expected the return statement to be the last statement in the function")
                .with_label(
                    Label::new(error_span)
                        .with_message("Expected the return statement to be the last statement in the function")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::ReturnTypeMismatch { expected, received } => {
                report_builder
                .with_message("Return type mismatch")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Expected the return value to be assignable to {}, found {}",
                          type_to_string(expected, string_interner),
                          type_to_string(received, string_interner)
                        ))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::CannotAccess(checked_type) => {
                report_builder
                .with_message("Cannot access")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Cannot use the access operator on the type \"{}\"", type_to_string(checked_type, string_interner)))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::CannotCall(checked_type) => {
                report_builder
                .with_message("Cannot call")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Cannot use the call operator on the type \"{}\"", type_to_string(checked_type, string_interner)))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::FnArgumentCountMismatch { expected, received } => {
                report_builder
                .with_message("Function argument count mismatch")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("This function expects {} arguments, but instead received {}", expected.to_string(), received.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::GenericArgumentCountMismatch { expected, received } => {
                report_builder
                .with_message("Generic argument count mismatch")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Expected {} type arguments, but instead received {}", expected.to_string(), received.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::CannotUseGenericParameterAsValue => {
                report_builder
                .with_message("Cannot use generic parameters as values")
                .with_label(
                    Label::new(error_span)
                        .with_message("Cannot use generic parameters where an expression is expected")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::CannotUseVariableDeclarationAsType => {
                report_builder
                .with_message("Cannot use variable declaration as a type")
                .with_label(
                    Label::new(error_span)
                        .with_message("Cannot use variable declaration as a type")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::VarDeclWithNoConstraintOrInitializer => {
                report_builder
                .with_message("Variable declarations must have a initializer or constraint or both")
                .with_label(
                    Label::new(error_span)
                        .with_message("Variable declarations must have a initializer or constraint or both")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::AccessToUndefinedProperty(id) => {
                let name = string_interner.resolve(id.name).unwrap();
                report_builder
                .with_message("Access to an undefined property")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Property {} is not defined", name))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::UnresolvedGenericParam(_) => {
                todo!();
            }
            SemanticErrorKind::CannotUseIsTypeOnNonUnion => {
                report_builder
                .with_message("Cannot use the \"::is(T)\" method on a non-union type")
                .with_label(
                    Label::new(error_span)
                        .with_message("Cannot use the \"::is(T)\" method on a non-union type")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::ConflictingGenericBinding { identifier, existing, new } => {
                let name = string_interner.resolve(identifier.name).unwrap();

                report_builder
                .with_message("Conflicting generic binding")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Generic parameter identifier {} is already bound to type {}, cannot re-bind it to {}", 
                          name, 
                          type_to_string(existing, string_interner), 
                          type_to_string(new, string_interner)
                        ))
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::TypeAliasMustBeDeclaredAtTopLevel => {
                report_builder
                .with_message("Type aliases must be declared in the file scope")
                .with_label(
                    Label::new(error_span)
                        .with_message("Type aliases must be declared in the file scope")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::StructMustBeDeclaredAtTopLevel => {
               report_builder
                .with_message("Structs must be declared in the file scope")
                .with_label(
                    Label::new(error_span)
                        .with_message("Structs must be declared in the file scope")
                        .with_color(Color::Red),
                )
                .finish()
            }
            SemanticErrorKind::CannotApplyTypeArguments { to } => {
                report_builder
                .with_message("Cannot apply type arguments")
                .with_label(
                    Label::new(error_span)
                        .with_message(format!("Cannot apply type arguments to non-generic type {}", type_to_string(to, string_interner)))
                        .with_color(Color::Red),
                )
                .finish()
            }
        };

        reports.push(report);
    });

    if !reports.is_empty() {
        let mut cache = FileSourceCache::new();
        cache.add(file_path.clone(), source_code.clone());

        for (index, report) in reports.into_iter().enumerate() {
            println!("\n=============== {} ===============\n", index + 1);
            report.eprint(&mut cache).unwrap();
            println!();
        }
    } else {
        println!(
            "Compilation successful for {} (no errors found).",
            file_path
        );
    }
}
