use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
pub mod file_cache;
pub mod interner;

use ariadne::{Color, Label, Report};

use crate::{
    ast::{
        decl::Declaration,
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
    },
    compile::{
        file_cache::FileCache,
        interner::{SharedStringInterner, SharedTagInterner},
    },
    hir::{
        errors::{SemanticError, SemanticErrorKind},
        utils::type_to_string::type_to_string,
        ProgramBuilder,
    },
    parse::{Parser, ParsingError, ParsingErrorKind},
    tokenize::{
        token_kind_to_string, TokenizationError, TokenizationErrorKind, Tokenizer,
    },
};

pub struct Compiler {
    string_interner: Arc<SharedStringInterner>,
    tag_interner: Arc<SharedTagInterner>,
    files: Arc<Mutex<FileCache>>,
    errors: Vec<CompilationError>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ParallelParseResult {
    path: PathBuf,
    statements: Vec<Stmt>,
    tokenization_errors: Vec<TokenizationError>,
    parsing_errors: Vec<ParsingError>,
    declarations: Vec<Declaration>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            string_interner: Arc::new(SharedStringInterner::default()),
            tag_interner: Arc::new(SharedTagInterner::default()),
            files: Arc::new(Mutex::new(FileCache::default())),
            errors: Vec::new(),
        }
    }

    pub fn compile(&mut self, main_path: PathBuf) {
        let parsed_modules = self.parallel_parse_modules(main_path);

        for m in parsed_modules {
            match m {
                Err(e) => {
                    self.errors.push(e);
                }
                Ok(module) => {
                    let has_tokenization_errors = !module.tokenization_errors.is_empty();
                    let has_parsing_errors = !module.parsing_errors.is_empty();

                    if has_tokenization_errors {
                        self.errors.push(CompilationError::Tokenization {
                            path: module.path.clone(),
                            errors: module.tokenization_errors.clone(),
                        });
                    }

                    if has_parsing_errors {
                        self.errors.push(CompilationError::Parsing {
                            path: module.path.clone(),
                            errors: module.parsing_errors.clone(),
                        });
                    }
                }
            };
        }

        if !self.errors.is_empty() {
            self.report_errors();
            return;
        }

        let mut program_builder =
            ProgramBuilder::new(self.string_interner.clone(), self.tag_interner.clone());

        // TODO: Pass `modules_to_compile` to the program_builder to generate HIR
        // let semantic_errors = program_builder.build(modules_to_compile);
        // self.errors.push(semantic_errors);

        // if !self.errors.is_empty() {
        //     self.report_errors();
        //     return;
        // }
    }

    pub fn parallel_parse_modules(
        &self,
        main_path: PathBuf,
    ) -> Vec<Result<ParallelParseResult, CompilationError>> {
        let canonical_main = main_path
            .canonicalize()
            .expect("Could not find the main module");

        let visited = Arc::new(Mutex::new(HashSet::new()));
        let all_results = Arc::new(Mutex::new(Vec::new()));

        rayon::scope(|s| {
            fn parse_recursive(
                path: PathBuf,
                s: &rayon::Scope,
                interner: Arc<SharedStringInterner>,
                files: Arc<Mutex<FileCache>>,
                visited: Arc<Mutex<HashSet<PathBuf>>>,
                all_results: Arc<
                    Mutex<Vec<Result<ParallelParseResult, CompilationError>>>,
                >,
            ) {
                let path = path.canonicalize().expect("Could not find a module");

                {
                    let mut visited_guard = visited.lock().unwrap();
                    if !visited_guard.insert(path.clone()) {
                        return;
                    }
                }

                let source_code = match fs::read_to_string(&path) {
                    Ok(sc) => sc,
                    Err(e) => {
                        all_results.lock().unwrap().push(Err(
                            CompilationError::CouldNotReadFile { path, error: e },
                        ));
                        return;
                    }
                };

                let (tokens, tokenization_errors) =
                    Tokenizer::tokenize(&source_code, interner.clone());
                let (statements, parsing_errors) =
                    Parser::parse(tokens, interner.clone());

                let (dependencies, dependency_errors, declarations) =
                    find_dependencies(&path, &statements);

                for dep_path in dependencies {
                    let interner = Arc::clone(&interner);
                    let files = Arc::clone(&files);
                    let visited = Arc::clone(&visited);
                    let all_results = Arc::clone(&all_results);

                    s.spawn(move |s| {
                        parse_recursive(
                            dep_path,
                            s,
                            interner,
                            files,
                            visited,
                            all_results,
                        );
                    });
                }

                files.lock().unwrap().insert(path.clone(), source_code);

                let mut results_guard = all_results.lock().unwrap();
                results_guard.extend(dependency_errors.into_iter().map(Err));
                results_guard.push(Ok(ParallelParseResult {
                    path,
                    statements,
                    declarations,
                    tokenization_errors,
                    parsing_errors,
                }));
            }

            parse_recursive(
                canonical_main,
                s,
                self.string_interner.clone(),
                self.files.clone(),
                visited,
                all_results.clone(),
            );
        });

        Arc::try_unwrap(all_results)
            .expect("Arc unwrap failed")
            .into_inner()
            .expect("Mutex into_inner failed")
    }

    pub fn report_errors(&self) {
        let mut cache = self.files.lock().unwrap();

        for error in &self.errors {
            match error {
                CompilationError::Tokenization { path, errors } => {
                    errors.iter().for_each(|e| {
                        let span = e.span.start.byte_offset..e.span.end.byte_offset;
                        let report = Report::build(
                            ariadne::ReportKind::Error,
                            (path.clone(), span.clone()),
                        )
                        .with_code(format!("T{}", e.kind.code()));
                        let label = Label::new((path.clone(), span));

                        let final_report = match &e.kind {
                            TokenizationErrorKind::UnterminatedString => {
                                report.with_message("Unterminated string").with_label(
                                    label.with_message("This string is not terminated"),
                                )
                            }
                            TokenizationErrorKind::UnknownToken => {
                                report.with_message("Unknown token").with_label(
                                    label.with_message("This token is not recognized"),
                                )
                            }
                            TokenizationErrorKind::UnknownEscapeSequence => {
                                report.with_message("Unknown escape sequence").with_label(
                                    label.with_message(
                                        "The escape sequence here is invalid",
                                    ),
                                )
                            }
                            TokenizationErrorKind::InvalidFloatingNumber => report
                                .with_message("Invalid floating-point number")
                                .with_label(label.with_message(
                                    "This is not a valid floating-point number",
                                )),
                            TokenizationErrorKind::InvalidIntegerNumber => {
                                report.with_message("Invalid integer number").with_label(
                                    label.with_message(
                                        "This is not a valid integer number",
                                    ),
                                )
                            }
                            TokenizationErrorKind::UnterminatedDoc => report
                                .with_message("Unterminated documentation")
                                .with_label(label.with_message(
                                    "This documentation block is not terminated",
                                )),
                        };

                        let _ = final_report.finish().print(&mut *cache);
                    });
                }
                CompilationError::Parsing { path, errors } => {
                    errors.iter().for_each(|e| {
                        let span = e.span.start.byte_offset..e.span.end.byte_offset;
                        let report = Report::build(
                            ariadne::ReportKind::Error,
                            (path.clone(), span.clone()),
                        )
                        .with_code(format!("P{}", e.kind.code()));
                        let label = Label::new((path.clone(), span)).with_color(Color::Red);

                        let final_report = match &e.kind {
                            ParsingErrorKind::DocMustBeFollowedByDeclaration => report
                                .with_message("Documentation must be followed by a declaration of ")
                                .with_label(label.with_message("This documentation must be followed by a declaration of a type alias or a variable")),
                            ParsingErrorKind::ExpectedAnExpressionButFound(token) => report
                                .with_message("Expected an expression")
                                .with_label(label.with_message(format!("Expected an expression but instead found token \"{}\"", token_kind_to_string(&token.kind, self.string_interner.clone())))),
                            ParsingErrorKind::ExpectedATypeButFound(token) => report
                                .with_message("Expected a type")
                                .with_label(label.with_message(format!("Expected a type but instead found token \"{}\"", token_kind_to_string(&token.kind, self.string_interner.clone())))),
                            ParsingErrorKind::InvalidSuffixOperator(token) => report
                                .with_message("Invalid suffix operator")
                                .with_label(label.with_message(format!("Invalid token as expression suffix operator \"{}\"", token_kind_to_string(&token.kind, self.string_interner.clone())))),
                            ParsingErrorKind::UnexpectedEndOfInput => report.with_message("Unexpected end of input").with_label(label.with_message("Unexpected end of input")),
                            ParsingErrorKind::ExpectedAnIdentifier => report.with_message("Expected an identifier").with_label(label.with_message("Expected an identifier")),
                            ParsingErrorKind::ExpectedAPunctuationMark(punctuation_kind) => report.with_message("Expected a punctuation mark").with_label(label.with_message(format!("Expected the \"{}\" punctuation mark", punctuation_kind.to_string()))),
                            ParsingErrorKind::ExpectedAKeyword(keyword_kind) => report.with_message("Expected a keyword").with_label(label.with_message(format!("Expected the \"{}\" keyword", keyword_kind.to_string()))),
                            ParsingErrorKind::ExpectedAStringValue => report.with_message("Expected a string literal").with_label(label.with_message("Expected a string literal")),
                            ParsingErrorKind::ExpectedANumericValue => report.with_message("Expected a numeric literal").with_label(label.with_message("Expected a numeric literal")),
                            ParsingErrorKind::UnknownStaticMethod(identifier_node) => {
                                let name = self.string_interner.resolve(identifier_node.name);
                                report.with_message("Unknown static method").with_label(label.with_message(format!("Static method with name \"{}\" doesn't exist", name)))
                            }
                            ParsingErrorKind::UnexpectedStatementAfterFinalExpression => report.with_message("Unexpected statement after final expression").with_label(label.with_message("Final expression of a codeblock must not be followed by another statement")),
                            ParsingErrorKind::ExpectedStatementOrExpression { found } => report
                                .with_message("Expected a statement or an expression")
                                .with_label(label.with_message(format!("Expected a statement or an expression but instead found token \"{}\"", token_kind_to_string(&found.kind, self.string_interner.clone())))),
                            ParsingErrorKind::UnexpectedTokenAfterFinalExpression { found } => report
                                .with_message("Unexpected token after final expression")
                                .with_label(label.with_message(format!("Unexpected token \"{}\" after final expression", token_kind_to_string(&found.kind, self.string_interner.clone())))),
                            ParsingErrorKind::ExpectedATagTypeButFound(type_annotation) => todo!(),

                        };

                        let _ = final_report.finish().print(&mut *cache);
                    });
                }
                CompilationError::Semantic { path, errors } => {
                    errors.iter().for_each(|e| {
let span = e.span.start.byte_offset..e.span.end.byte_offset;
                        let report =
                            Report::build(ariadne::ReportKind::Error, (path.clone(), span.clone()))
                                .with_code(format!("S{}", e.kind.code()));
                        let label = Label::new((path.clone(), span));

                        let final_report = match &e.kind {
                            SemanticErrorKind::ExpectedANumericOperand => report.with_message("Expected a numeric operand").with_label(label.with_message("Expected this value to have a numeric type")),
                            SemanticErrorKind::MixedSignedAndUnsigned => report.with_message("Mixed signed and unsigned operands").with_label(label.with_message("Mixing signed and unsigned operands in an arithmetic operation is not allowed")),
                            SemanticErrorKind::MixedFloatAndInteger => report.with_message("Mixed float and integer operands").with_label(label.with_message("Mixing integer and floating-point numbers in an arithmetic operation is not allowed")),
                            SemanticErrorKind::CannotCompareType { of, to } => {
                                report.with_message("Cannot compare types")
                                    .with_label(label.with_message(format!("Cannot compare type \"{}\" to type \"{}\"", type_to_string(&of.kind, self.string_interner.clone()), type_to_string(&to.kind, self.string_interner.clone()))))
                            }
                            SemanticErrorKind::UndeclaredIdentifier(id) => {
                                let name = self.string_interner.resolve(id.name);

                                report.with_message("Undeclared identifier").with_label(label.with_message(format!("Undeclared identifier \"{}\"", name)))
                            }
                            SemanticErrorKind::UndeclaredType(id) => {
                                let name = self.string_interner.resolve(id.name);

                                report.with_message("Undeclared type").with_label(label.with_message(format!("Undeclared type \"{}\"", name)))
                            }
                            SemanticErrorKind::ReturnKeywordOutsideFunction => report.with_message("Keyword \"return\" used outside of a function scope").with_label(label.with_message("Cannot use the \"return\" keyword outside of a function scope")),
                            SemanticErrorKind::BreakKeywordOutsideLoop => report.with_message("Keyword \"break\" used outside of a loop scope").with_label(label.with_message("Cannot use the \"break\" keyword outside of a loop scope")),
                            SemanticErrorKind::ContinueKeywordOutsideLoop => report.with_message("Keyword \"continue\" used outside of a loop scope").with_label(label.with_message("Cannot use the \"continue\" keyword outside of a loop scope")),
                            SemanticErrorKind::InvalidLValue => report.with_message("Invalid assignment target").with_label(label.with_message("Invalid assignment target")),
                            SemanticErrorKind::TypeMismatch { expected, received } => {
                                let constraint_str = type_to_string(&expected.kind, self.string_interner.clone());
                                let declaration_of_expected = expected.span.start.byte_offset..expected.span.end.byte_offset;

                                report.with_message("Type mismatch").with_labels(vec![
                                    label.with_message(format!("Type mismatch, expected `{}`, instead found `{}`", constraint_str, type_to_string(&received.kind, self.string_interner.clone()))),
                                    Label::new((path.clone(), declaration_of_expected)).with_message(format!("expected type \"{}\" originated here", constraint_str)),
                                ])
                            }
                            SemanticErrorKind::ReturnNotLastStatement => report
                                .with_message("Expected the return statement to be the last statement in the function")
                                .with_label(label.with_message("Expected the return statement to be the last statement in the function")),
                            SemanticErrorKind::ReturnTypeMismatch { expected, received } => {
                                report.with_message("Return type mismatch")
                                    .with_label(label.with_message(format!("Expected the return value to be assignable to {}, found {}", type_to_string(&expected.kind, self.string_interner.clone()), type_to_string(&received.kind, self.string_interner.clone()))))
                            }
                            SemanticErrorKind::CannotAccess(target) => report
                                .with_message("Cannot access field")
                                .with_label(label.with_message(format!("Cannot use the access operator on the type \"{}\"", type_to_string(&target.kind, self.string_interner.clone())))),
                            SemanticErrorKind::CannotCall(target) => report
                                .with_message("Cannot use the function call operator")
                                .with_label(label.with_message(format!("Cannot use the function-call operator on type \"{}\"", type_to_string(&target.kind, self.string_interner.clone())))),
                            SemanticErrorKind::FnArgumentCountMismatch { expected, received, .. } => {
                                let s = if *expected > 1 { "s" } else { "" };
                                report.with_message("Function argument count mismatch")
                                    .with_label(label.with_message(format!("This function expects {} argument{}, but instead received {}", expected.to_string(), s, received.to_string())))
                            }
                            SemanticErrorKind::CannotUseVariableDeclarationAsType => report.with_message("Cannot use variable declaration as a type").with_label(label.with_message("Cannot use variable declaration as a type")),
                            SemanticErrorKind::AccessToUndefinedField(field) => {
                                let name = self.string_interner.resolve(field.name);
                                report.with_message("Access to an undefined field").with_label(label.with_message(format!("Field {} is not defined", name)))
                            }
                            SemanticErrorKind::TypeAliasMustBeDeclaredAtTopLevel => report.with_message("Type aliases must be declared in the file scope").with_label(label.with_message("Type aliases must be declared in the file scope")),
                            SemanticErrorKind::DuplicateStructFieldInitializer(id) => {
                                let name = self.string_interner.resolve(id.name);
                                report.with_message("Duplicate initializer for a struct field").with_label(label.with_message(format!("Struct field \"{}\" cannot be initialized multiple times", name)))
                            }
                            SemanticErrorKind::UnknownStructFieldInitializer(id) => {
                                let name = self.string_interner.resolve(id.name);
                                report.with_message("Unknown field in the struct initializer").with_label(label.with_message(format!("Unknown struct field \"{}\"", name)))
                            }
                            SemanticErrorKind::MissingStructFieldInitializers(missing_fields) => {
                                let field_names: Vec<String> = missing_fields.into_iter().map(|f| self.string_interner.resolve(*f)).collect();
                                let joined = field_names.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<String>>().join(", ");
                                report.with_message("Missing field initializers").with_label(label.with_message(format!("Missing initializers for the following struct fields {}", joined)))
                            }
                            SemanticErrorKind::VarDeclWithoutConstraintOrInitializer { .. } => report.with_message("Variable declarations must have an initializer").with_label(label.with_message("This variable declaration must have an initializer")),
                            SemanticErrorKind::DuplicateIdentifier(id) => {
                                let identifier_name = self.string_interner.resolve(id.name);
                                report.with_message("Duplicate identifier").with_label(label.with_message(format!("Duplicate identifier declaration \"{}\"", identifier_name)))
                            }
                            SemanticErrorKind::CannotIndex(_) => todo!(),
                            SemanticErrorKind::IncompatibleBranchTypes { first, second } => todo!(),
                            SemanticErrorKind::TypeMismatchExpectedOneOf { expected, received } => todo!(),
                            SemanticErrorKind::CannotStaticAccess(_) => todo!(),
                            SemanticErrorKind::AccessToUndefinedStaticField(identifier_node) => todo!(),
                            SemanticErrorKind::IfExpressionMissingElse => todo!(),
                            SemanticErrorKind::CannotCastType { source_type, target_type } => todo!(),
                            SemanticErrorKind::CannotUseTypeDeclarationAsValue => todo!(),
                            SemanticErrorKind::UnreachableCode => todo!(),
                            SemanticErrorKind::FromStatementMustBeDeclaredAtTopLevel => todo!(),
                            SemanticErrorKind::ModuleNotFound(path_buf) => todo!(),
                            SemanticErrorKind::CannotDeclareGlobalVariable => todo!(),
                            SemanticErrorKind::CannotUseFunctionDeclarationAsType => todo!(),
                            SemanticErrorKind::UseOfUninitializedVariable(identifier_node) => todo!(),
                            SemanticErrorKind::DuplicateUnionVariant(identifier_node) => todo!(),
                        };

                        let _ = final_report.finish().print(&mut *cache);
                    });
                }
                CompilationError::CouldNotReadFile { path, error } => {
                    println!(
                        "Could not read file at path {}, error {}",
                        path.display(),
                        error
                    )
                }
                CompilationError::ModuleNotFound {
                    importing_module,
                    target_path,
                    error,
                } => {
                    println!(
                        "Module not found {}, imported from {}, error: {}",
                        target_path.display(),
                        importing_module.display(),
                        error
                    )
                }
            };
        }
    }
}

fn find_dependencies(
    current_module_path: &Path,
    statements: &[Stmt],
) -> (HashSet<PathBuf>, Vec<CompilationError>, Vec<Declaration>) {
    let mut dependencies = HashSet::new();
    let mut errors = vec![];
    let mut declarations: Vec<Declaration> = vec![];

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
                        errors.push(CompilationError::ModuleNotFound {
                            importing_module: current_module_path.to_path_buf(),
                            target_path,
                            error: e,
                        });
                    }
                }
            }
            StmtKind::Expression(Expr {
                kind: ExprKind::Fn(decl),
                ..
            }) => {
                declarations.push(Declaration::Fn(decl.clone()));
            }
            StmtKind::TypeAliasDecl(decl) => {
                declarations.push(Declaration::TypeAlias(decl.clone()));
            }
            _ => {}
        }
    }

    (dependencies, errors, declarations)
}
