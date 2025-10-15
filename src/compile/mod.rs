use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::{Path, PathBuf},
};

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

pub mod string_interner;

use crate::{
    ast::{
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
    },
    compile::string_interner::StringInterner,
    hir::{
        cfg::CheckedDeclaration,
        errors::{SemanticError, SemanticErrorKind},
        utils::type_to_string::type_to_string,
        ProgramBuilder,
    },
    parse::{Parser, ParsingError, ParsingErrorKind},
    tokenize::{
        token_kind_to_string, TokenizationError, TokenizationErrorKind, Tokenizer,
    },
};

pub fn compile_file<'a, 'b>(
    file_path: &'a str,
    source_code: &'a str,
    program_builder: &'b mut ProgramBuilder<'b>,
    files: &mut SimpleFiles<usize, String>,
) {
    let interned_fp = program_builder.string_interner.intern(file_path).0;
    files.add(interned_fp, source_code.to_string());

    let (tokens, tokenization_errors) = Tokenizer::tokenize(source_code);
    let (ast, parsing_errors) = Parser::parse(tokens, program_builder.string_interner);
    let path_buf = PathBuf::from(file_path);
    program_builder.build_module(path_buf, ast);

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
                .with_label(
                    label.with_message("This is not a valid floating-point number"),
                ),
            TokenizationErrorKind::InvalidIntegerNumber => err
                .with_message("Invalid integer number")
                .with_label(label.with_message("This is not a valid integer number")),
            TokenizationErrorKind::UnterminatedDoc => {
                err.with_message("Unterminated documentation").with_label(
                    label.with_message("This documentation block is not terminated"),
                )
            }
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
                let name = string_interner.resolve(identifier_node.name);
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
            ParsingErrorKind::ExpectedATagTypeButFound(type_annotation) => todo!(),
        };

        errors.push(diagnostic);
    });

    semantic_errors.into_iter().for_each(|e| {
        let span = e.span();
        let error_span = span.start.byte_offset..span.end.byte_offset;
        let label = Label::primary(interned_fp, error_span);
        let err = Diagnostic::error().with_code(format!("S{}", e.code()));

        let diagnostic = match &e {
            SemanticErrorKind::ExpectedANumericOperand { .. } => err
                .with_message("Expected a numeric operand")
                .with_label(label.with_message("Expected this value to have a numeric type")),
            SemanticErrorKind::MixedSignedAndUnsigned { .. } => err
                .with_message("Mixed signed and unsigned operands")
                .with_label(label.with_message("Mixing signed and unsigned operands in an arithmetic operation is not allowed")),
            SemanticErrorKind::MixedFloatAndInteger { .. } => err.with_message("Mixed float and integer operands").with_label(
                label.with_message("Mixing integer and floating-point numbers in an arithmetic operation is not allowed"),
            ),
            SemanticErrorKind::CannotCompareType { of, to } => {
                err.with_message("Cannot compare types")
                    .with_label(label.with_message(format!(
                        "Cannot compare type \"{}\" to type \"{}\"",
                        type_to_string(&of.kind, string_interner),
                        type_to_string(&to.kind, string_interner)
                    )))
            }
            SemanticErrorKind::UndeclaredIdentifier(id) => {
                let name = string_interner.resolve(id.name);

                err.with_message("Undeclared identifier")
                    .with_label(label.with_message(format!("Undeclared identifier \"{}\"", name)))
            }
            SemanticErrorKind::UndeclaredType(id) => {
                let name = string_interner.resolve(id.name);

                err.with_message("Undeclared type")
                    .with_label(label.with_message(format!("Undeclared type \"{}\"", name)))
            }
            SemanticErrorKind::ReturnKeywordOutsideFunction { .. } => err
                .with_message("Keyword \"return\" used outside of a function scope")
                .with_label(label.with_message("Cannot use the \"return\" keyword outside of a function scope")),
            SemanticErrorKind::BreakKeywordOutsideLoop { .. } => err
                .with_message("Keyword \"break\" used outside of a loop scope")
                .with_label(label.with_message("Cannot use the \"break\" keyword outside of a loop scope")),
            SemanticErrorKind::ContinueKeywordOutsideLoop { .. } => err
                .with_message("Keyword \"continue\" used outside of a loop scope")
                .with_label(label.with_message("Cannot use the \"continue\" keyword outside of a loop scope")),
            SemanticErrorKind::InvalidLValue { .. } => err
                .with_message("Invalid assignment target")
                .with_label(label.with_message("Invalid assignment target")),
            SemanticErrorKind::TypeMismatch { expected, received } => {
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
            SemanticErrorKind::ReturnNotLastStatement { .. } => err
                .with_message("Expected the return statement to be the last statement in the function")
                .with_label(label.with_message("Expected the return statement to be the last statement in the function")),
            SemanticErrorKind::ReturnTypeMismatch { expected, received } => {
                err.with_message("Return type mismatch")
                    .with_label(label.with_message(format!(
                        "Expected the return value to be assignable to {}, found {}",
                        type_to_string(&expected.kind, string_interner),
                        type_to_string(&received.kind, string_interner)
                    )))
            }
            SemanticErrorKind::CannotAccess(target) => {
                err.with_message("Cannot access field").with_label(label.with_message(format!(
                    "Cannot use the access operator on the type \"{}\"",
                    type_to_string(&target.kind, string_interner)
                )))
            }
            SemanticErrorKind::CannotCall(target) => {
                err.with_message("Cannot use the function call operator")
                    .with_label(label.with_message(format!(
                        "Cannot use the function-call operator on type \"{}\"",
                        type_to_string(&target.kind, string_interner)
                    )))
            }
            SemanticErrorKind::FnArgumentCountMismatch { expected, received, .. } => {
                let s = if expected > 1 { "s" } else { "" };
                err.with_message("Function argument count mismatch")
                    .with_label(label.with_message(format!(
                        "This function expects {} argument{}, but instead received {}",
                        expected.to_string(),
                        s,
                        received.to_string()
                    )))
            }
            SemanticErrorKind::CannotUseVariableDeclarationAsType { .. } => err
                .with_message("Cannot use variable declaration as a type")
                .with_label(label.with_message("Cannot use variable declaration as a type")),
            SemanticErrorKind::AccessToUndefinedField(field) => {
                let name = string_interner.resolve(field.name);
                err.with_message("Access to an undefined field")
                    .with_label(label.with_message(format!("Field {} is not defined", name)))
            }
            SemanticErrorKind::TypeAliasMustBeDeclaredAtTopLevel { .. } => err
                .with_message("Type aliases must be declared in the file scope")
                .with_label(label.with_message("Type aliases must be declared in the file scope")),
            SemanticErrorKind::StructMustBeDeclaredAtTopLevel { .. } => err
                .with_message("Structs must be declared in the file scope")
                .with_label(label.with_message("Structs must be declared in the file scope")),
            SemanticErrorKind::DuplicateStructFieldInitializer(id) => {
                let name = string_interner.resolve(id.name);
                err.with_message("Duplicate initializer for a struct field")
                    .with_label(label.with_message(format!("Struct field \"{}\" cannot be initialized multiple times", name)))
            }
            SemanticErrorKind::UnknownStructFieldInitializer(id) => {
                let name = string_interner.resolve(id.name);
                err.with_message("Unknown field in the struct initializer")
                    .with_label(label.with_message(format!("Unknown struct field \"{}\"", name)))
            }
            SemanticErrorKind::MissingStructFieldInitializers(missing_fields) => {
                let field_names: Vec<&'a str> = missing_fields.into_iter().map(|f| string_interner.resolve(f)).collect();
                let joined = field_names
                    .iter()
                    .map(|n| format!("\"{}\"", n))
                    .collect::<Vec<String>>()
                    .join(", ");
                err.with_message("Missing field initializers")
                    .with_label(label.with_message(format!("Missing initializers for the following struct fields {}", joined)))
            }
            SemanticErrorKind::VarDeclWithoutConstraintOrInitializer { .. } => err
                .with_message("Variable declarations must have an initializer")
                .with_label(label.with_message("This variable declaration must have an initializer")),
            SemanticErrorKind::DuplicateIdentifier(id) => {
                let identifier_name = string_interner.resolve(id.name);
                err.with_message("Duplicate identifier")
                    .with_label(label.with_message(format!("Duplicate identifier declaration \"{}\"", identifier_name)))
            }
            SemanticErrorKind::CannotIndex(_) => todo!(),
            SemanticErrorKind::IncompatibleBranchTypes { first, second } => todo!(),
            SemanticErrorKind::ExpectedEnumType => todo!(),
            SemanticErrorKind::TypeMismatchExpectedOneOf { expected, received } => todo!(),
            SemanticErrorKind::CannotStaticAccess(_) => todo!(),
            SemanticErrorKind::ExpectedAType => todo!(),
            SemanticErrorKind::AccessToUndefinedStaticField(identifier_node) => todo!(),
            SemanticErrorKind::IfExpressionMissingElse => todo!(),
            SemanticErrorKind::CannotCastType {
                source_type,
                target_type,
            } => todo!(),
            SemanticErrorKind::CannotUseTypeDeclarationAsValue => todo!(),
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
        println!(
            "Compilation successful for {} (no errors found).",
            file_path
        );
    }
}

pub struct Compiler {
    errors: Vec<CompilationError>,
}

pub enum CompilationError {
    CouldNotReadFile {
        path: PathBuf,
        error: std::io::Error,
    },
    ModuleNotFound {
        importing_module: PathBuf,
        target_path: PathBuf,
        error: std::io::Error,
    },
    Tokenization {
        path: PathBuf,
        errors: Vec<TokenizationError>,
    },
    Parsing {
        path: PathBuf,
        errors: Vec<ParsingError>,
    },
    Semantic {
        path: PathBuf,
        errors: Vec<SemanticError>,
    },
}

struct ParsingResult {
    tokenization_errors: Vec<TokenizationError>,
    parsing_errors: Vec<ParsingError>,
    statements: Vec<Stmt>,
}

impl Compiler {
    pub fn compile(&mut self, main_path: PathBuf) {
        let mut interner = StringInterner::new();
        let mut program_builder = ProgramBuilder::new(&mut interner);
        let modules = self.parse_all_modules(main_path, &mut program_builder);
    }

    pub fn parse_all_modules(
        &mut self,
        main: PathBuf,
        program_builder: &mut ProgramBuilder,
    ) -> HashMap<PathBuf, Vec<Stmt>> {
        let mut modules: HashMap<PathBuf, Vec<Stmt>> = HashMap::new();
        let mut work_queue: VecDeque<PathBuf> = VecDeque::new();
        let mut visited: HashSet<PathBuf> = HashSet::new();

        let canonical_main =
            fs::canonicalize(main).expect("Could not find the main module");
        work_queue.push_back(canonical_main.clone());
        visited.insert(canonical_main);

        while let Some(source_path) = work_queue.pop_front() {
            let source_code = match fs::read_to_string(&source_path) {
                Ok(source_code) => source_code,
                Err(e) => {
                    self.errors.push(CompilationError::CouldNotReadFile {
                        path: source_path,
                        error: e,
                    });
                    continue;
                }
            };
            let (tokens, tokenization_errors) =
                Tokenizer::tokenize(&source_code, program_builder.string_interner);
            if tokenization_errors.len() > 0 {
                self.errors.push(CompilationError::Tokenization {
                    path: source_path.clone(),
                    errors: tokenization_errors,
                })
            }

            let (statements, parsing_errors) =
                Parser::parse(tokens, program_builder.string_interner);
            if parsing_errors.len() > 0 {
                self.errors.push(CompilationError::Parsing {
                    path: source_path.clone(),
                    errors: parsing_errors,
                })
            }

            let dependency_modules =
                self.find_dependency_modules(program_builder, &source_path, &statements);

            for module_path in dependency_modules {
                if visited.insert(module_path.clone()) {
                    work_queue.push_back(module_path);
                }
            }

            modules.entry(source_path).or_insert(statements);
        }

        modules
    }

    pub fn find_dependency_modules(
        &mut self,
        program_builder: &mut ProgramBuilder,
        current_module_path: &Path,
        statements: &[Stmt],
    ) -> HashSet<PathBuf> {
        let mut dependencies: HashSet<PathBuf> = HashSet::new();

        // TODO: fix the following expression, currently it panics
        let current_module_builder = program_builder
            .modules
            .get_mut(current_module_path)
            .expect(&format!(
                "INTERNAL COMPILER ERROR: Expected the module {} to exist",
                current_module_path.display(),
            ));

        for stmt in statements {
            match &stmt.kind {
                StmtKind::From { path, .. } => {
                    let relative_path_str = &path.value;

                    let mut target_path = current_module_path.to_path_buf();
                    target_path.pop();
                    target_path.push(relative_path_str);

                    match fs::canonicalize(target_path.clone()) {
                        Ok(canonical_path) => {
                            dependencies.insert(canonical_path);
                        }
                        Err(e) => {
                            self.errors.push(CompilationError::ModuleNotFound {
                                importing_module: current_module_path.to_path_buf(),
                                target_path,
                                error: e,
                            });
                        }
                    }
                }
                StmtKind::TypeAliasDecl(decl) => {
                    let id = program_builder.new_declaration_id();
                    // TODO: add new method -> current_module_builder.file_scope_insert(decl.identifier.name, id);

                    let checked_decl = todo!();

                    program_builder
                        .declarations
                        .insert(id, CheckedDeclaration::TypeAlias(checked_decl));
                }
                StmtKind::Expression(Expr {
                    kind: ExprKind::Fn { name, .. },
                    ..
                }) => {
                    let id = program_builder.new_declaration_id();
                    // TODO: add new method -> current_module_builder.file_scope_insert(name.name, id);

                    let checked_fn_placeholder = todo!();

                    program_builder
                        .declarations
                        .insert(id, CheckedDeclaration::Function(checked_fn_placeholder));
                }
                _ => {}
            }
        }

        dependencies
    }
}
