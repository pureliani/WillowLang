use ariadne::{Cache, Color, Label, Report, ReportKind, Source};
use std::{cell::RefCell, collections::HashMap, fmt::format, rc::Rc, vec};

use crate::{
    check::{
        check_stmts::check_stmts,
        scope::{Scope, ScopeKind},
        SemanticError, SemanticErrorKind,
    },
    parse::{Parser, ParsingErrorKind},
    tokenizer::{TokenizationErrorKind, Tokenizer},
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

pub fn compile_file(file_path: &String, source_code: &String) {
    let mut reports: Vec<Report<'_, (String, std::ops::Range<usize>)>> = vec![];

    let (tokens, tokenization_errors) = Tokenizer::tokenize(source_code);
    let (ast, parsing_errors) = Parser::parse(tokens);
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
                    Label::new(error_span.clone())
                        .with_message("This string is not terminated")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::UnknownToken => report_builder
                .with_message("Unknown token")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("This token is not recognized")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::UnknownEscapeSequence => report_builder
                .with_message("Unknown escape sequence")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("The escape sequence here is invalid")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::InvalidFloatingNumber => report_builder
                .with_message("Invalid floating-point number")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("This is not a valid floating-point number")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::InvalidIntegerNumber => report_builder
                .with_message("Invalid integer number")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("This is not a valid integer number")
                        .with_color(Color::Red),
                )
                .finish(),
            TokenizationErrorKind::UnterminatedDoc => report_builder
                .with_message("Unterminated documentation")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("This documentation block is not terminated")
                        .with_color(Color::Red),
                )
                .finish(),
        };

        reports.push(report)
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
                    Label::new(error_span.clone())
                        .with_message("This documentation must be followed by a declaration of either struct, type alias, enum or a variable")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAnExpressionButFound(token) => {
                report_builder
                .with_message("Expected an expression")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message(format!("Expected an expression but instead found token: {}", token.kind.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedATypeButFound(token) => {
                report_builder
                .with_message("Expected a type")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message(format!("Expected a type but instead found token: {}", token.kind.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::InvalidSuffixOperator(token) => {
                report_builder
                .with_message("Invalid suffix operator")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message(format!("Invalid token as expression suffix operator: {}", token.kind.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::UnexpectedEndOfInput => {
                report_builder
                .with_message("Unexpected end of input")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("Unexpected end of input")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAnIdentifier => {
                report_builder
                .with_message("Expected an identifier")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("Expected an identifier")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAPunctuationMark(punctuation_kind) => {
                report_builder
                .with_message("Expected a punctuation mark")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message(format!("Expected the \"{}\" punctuation mark", punctuation_kind.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAKeyword(keyword_kind) => {
                report_builder
                .with_message("Expected a keyword")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message(format!("Expected the \"{}\" keyword", keyword_kind.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedAStringValue => {
                report_builder
                .with_message("Expected a string literal")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("Expected a string literal")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedANumericValue => {
                report_builder
                .with_message("Expected a numeric literal")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("Expected a numeric literal")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::UnknownStaticMethod(identifier_node) => {
                report_builder
                .with_message("Unknown static method")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message(format!("Static method with name \"{}\" doesn't exist", identifier_node.name))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::UnexpectedStatementAfterFinalExpression => {
                report_builder
                .with_message("Unexpected statement after final expression")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message("Final expression of a codeblock must not be followed by another statement")
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::ExpectedStatementOrExpression { found } => {
                report_builder
                .with_message("Expected a statement or an expression")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message(format!("Expected a statement or an expression but instead found token: {}", found.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
            ParsingErrorKind::UnexpectedTokenAfterFinalExpression { found } => {
                report_builder
                .with_message("Unexpected token after final expression")
                .with_label(
                    Label::new(error_span.clone())
                        .with_message(format!("Unexpected token after final expression: {}", found.to_string()))
                        .with_color(Color::Red),
                )
                .finish()
            }
        };

        reports.push(report)
    });

    semantic_errors.into_iter().for_each(|e| {
        let error_span = (
            file_path.clone(),
            e.span.start.byte_offset..e.span.end.byte_offset,
        );

        let report_builder = Report::build(ReportKind::Error, error_span.clone())
            .with_code(format!("S{}", e.kind.code()));

        let report = match &e.kind {
            SemanticErrorKind::NonNumericOperand => {
                todo!();
            }
            SemanticErrorKind::MixedSignedAndUnsigned => {
                todo!();
            }
            SemanticErrorKind::MixedFloatAndInteger => {
                todo!();
            }
            SemanticErrorKind::CannotCompareType { of, to } => {
                todo!();
            }
            SemanticErrorKind::UndeclaredIdentifier(_) => {
                todo!();
            }
            SemanticErrorKind::UndeclaredType(_) => {
                todo!();
            }
            SemanticErrorKind::ReturnKeywordOutsideFunction => {
                todo!();
            }
            SemanticErrorKind::BreakKeywordOutsideLoop => {
                todo!();
            }
            SemanticErrorKind::ContinueKeywordOutsideLoop => {
                todo!();
            }
            SemanticErrorKind::InvalidAssignmentTarget => {
                todo!();
            }
            SemanticErrorKind::TypeMismatch { expected, received } => {
                todo!();
            }
            SemanticErrorKind::InvalidArraySizeValue(number_kind) => {
                todo!();
            }
            SemanticErrorKind::ReturnNotLastStatement => {
                todo!();
            }
            SemanticErrorKind::ReturnTypeMismatch { expected, received } => {
                todo!();
            }
            SemanticErrorKind::CannotAccess(checked_type) => {
                todo!();
            }
            SemanticErrorKind::CannotCall(checked_type) => {
                todo!();
            }
            SemanticErrorKind::FnArgumentCountMismatch { expected, received } => {
                todo!();
            }
            SemanticErrorKind::GenericArgumentCountMismatch { expected, received } => {
                todo!();
            }
            SemanticErrorKind::CannotUseGenericParameterAsValue => {
                todo!();
            }
            SemanticErrorKind::CannotUseVariableDeclarationAsType => {
                todo!();
            }
            SemanticErrorKind::VarDeclWithNoConstraintOrInitializer => {
                todo!();
            }
            SemanticErrorKind::UndefinedProperty(identifier_node) => {
                todo!();
            }
            SemanticErrorKind::UnresolvedGenericParam(_) => {
                todo!();
            }
            SemanticErrorKind::CannotUseIsTypeOnNonUnion => {
                todo!();
            }
            SemanticErrorKind::ConflictingGenericBinding { existing, new } => {
                todo!();
            }
            SemanticErrorKind::TypeAliasMustBeDeclaredAtTopLevel => {
                todo!();
            }
            SemanticErrorKind::StructMustBeDeclaredAtTopLevel => {
                todo!();
            }
            SemanticErrorKind::CannotApplyTypeArguments { to } => {
                todo!();
            }
        };

        reports.push(report)
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
