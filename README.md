
## Progress

### Lexical Analysis (Tokenization)

- 🟢 Tokenization of identifiers.
- 🟢 Tokenization of strings (including escape sequences: `\n`, `\t`, `\"`, `\\`).
- 🟢 Tokenization of numbers (integers and floating-point, various sizes: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`, `usize`, `isize`).
- 🟢 Tokenization of documentation comments (`--- ... ---`).
- 🟢 Tokenization of punctuation and operators (`:`, `::`, `|`, `||`, `&`, `&&`, `=`, `==`, `=>`, `<`, `<=`, `>`, `>=`, `!`, `!=`, `;`, `.`, `(`, `)`, `[`, `]`, `{`, `}`, `+`, `-`, `*`, `/`, `%`, `,`, `$`, `?`).
- 🟢 Tokenization of keywords (`struct`, `let`, `return`, `if`, `else`, `while`, `break`, `continue`, `type`, `from`, `void`, `null`, `true`, `false`, `pub`, `char`, `bool`, `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`, `usize`, `isize`).
- 🟢 Error handling during tokenization (unknown tokens, invalid numbers, unterminated strings/docs).
- 🟢 Tracking token spans (line and column numbers) for error reporting.

### Parsing

-   **Expressions:**
    - 🟢 Arithmetic operations (`+`, `-`, `*`, `/`, `%`).
    - 🟢 Logical operations (`!`, `&&`, `||`).
    - 🟢 Comparison operations (`<`, `<=`, `>`, `>=`, `==`, `!=`).
    - 🟢 Negation (`-1`).
    - 🟢 Parenthesized expressions.
    - 🟢 Array literals.
    - 🟢 Code block expressions (`{ ... }`).
    - 🟢 `if` expressions (including `else if` and `else` branches).
    - 🟢 Function call expressions.
    - 🟢 Struct initialization expressions.
    - 🟢 Member access (`.`).
    - 🟢 Static access (`::`).
    - 🟢 Type casting (`::as()`).
    - 🟢 Union type-narrowing (`::is()`).
    - 🟢 Generic applications (`<i32, char>`).
    - 🟢 Literals: identifiers, numbers, strings, booleans (`true`, `false`), and `null`.
    - 🟢 Function expressions.

-   **Statements:**
    - 🟢 `from` statements (imports).
    - 🟢 `while` loops.
    - 🟢 `return` statements.
    - 🟢 `break` statements.
    - 🟢 `continue` statements.
    - 🟢 `struct` declarations (including generic parameters and properties).
    - 🟢 `type` alias declarations.
    - ⚪ `views` declarations.
    - ⚪ `derived` declarations.
    - 🟢 Variable declarations (`let`) with optional type annotations and initializers.
    - 🟢 Assignment statements.
    - 🟢 Expression statements.

-   **Type Annotations:**
    - 🟢 Primitive types (`void`, `null`, `bool`, `char`, `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`, `usize`, `isize`).
    - 🟢 Function types (including generic parameters, parameter types, and return types, e.g `(name: string) => string`).
    - 🟢 Parenthesized type annotations.
    - 🟢 Union types (`|`).
    - 🟢 Array types (`type[size]`).
    - 🟢 Generic type application.
    - 🟢 Identifiers (for structs and type-aliases).

-   **General Parsing:**
    - 🟢 Parsing of generic parameters and arguments.
    - 🟢 Error handling during statement parsing and synchronization. 
    - ⚪ Error handling during expression parsing and synchronization. 
    - 🟢 Pratt parser implementation for expression parsing.
    - 🟢 Documentation annotations for declarations.

### Abstract Syntax Tree (AST)

- 🟢 Base AST nodes for expressions, statements, declarations, and types.
- 🟢 Checked AST nodes representing the type-checked program.
- 🟢 `Span` information for all AST nodes.
- 🟢 `IdentifierNode` and `StringNode` for identifiers and strings.
- ⚪ NumberNode for numbers.
- ⚪ Monomorphized AST (resolving generics to concrete type instances).

### Semantic Analysis and Type Checking

-   **Scope Management:**
    - 🟢 File scope.
    - 🟢 Function scope.
    - 🟢 `while` loop scope.
    - 🟢 Code block scope.

-   **Type Checking:**
    - 🟢 Arithmetic operations.
    - 🟢 Logical operations.
    - 🟢 Comparison operations.
    - 🟢 Negation.
    - 🟢 Member access.
    - 🟢 Literals (numbers, strings, booleans, null).
    - 🟢 Identifiers (variable lookup).
    - 🟢 Function expressions.
    - 🟢 `if` expressions.
    - 🟢 Array literals.
    - 🟢 Block expressions.
    - ⚪ Static access.
    - ⚪ Type casts.
    - ⚪ Generic application.
    - ⚪ Function calls.
        - ⚪ Checking that argument count match parameter count (considering optionals).
        - ⚪ Checking that argument types match parameter types (considering generics).
        - ⚪ Handling of optional properties.
        - ⚪ Handling of generic function calls (generic type inference/checking).
    - ⚪ Struct initialization.
        - ⚪ Checking that all required fields are provided.
        - ⚪ Checking that field types match the property constraints (considering generics).
        - ⚪ Handling of optional fields.
        - ⚪ Handling of generic struct initializations (generic type inference/checking).
    - 🟢 Detection of undeclared variables and types.
    - 🟢 Checking for invalid assignment targets.
    - 🟢 Checking for `return` statements outside of functions.
    - ⚪ Checking for return type mismatches.
    - 🟢 Checking for returns not being the last statement in codeblocks.
    - 🟢 Checking for mixed signed/unsigned and float/integer operations.
    - 🟢 Checking for invalid comparison operations.
    - 🟢 Checking for non-numeric operands in numeric operations.
    - 🟢 Checking for non-boolean operands in logical operations.
    - 🟢 Checking for invalid access operations.
    - 🟢 `break` statements (checking for loop context).
    - 🟢 `continue` statements (checking for loop context).
    - ⚪ Assignment statements (type compatibility checks).
    - 🟢 `while` loops (condition type checking).
    - 🟢 `struct` declarations.
    - ⚪ `from` statements.
    - ⚪ `type` alias declarations.

-   **Type Resolution and Inference:**
    - 🟢 Resolving type identifiers to actual types (checked ast).
    - 🟢 Basic type inference (e.g., for variable declarations without explicit types, return types).
    - ⚪ Generic type checking (instantiation and constraint satisfaction).

-   **Utility Functions:**
    - 🟢 `check_returns` function to collect return expressions.
    - 🟢 `union_of` function to combine inferred types.
    - ⚪ `check_is_assignable` function (implementation).

### Code Generation

- ⚪ Generation of LLVM IR.
- ⚪ Integration with a LLVM compiler.
- ⚪ Executable generation.

### Garbage Collection

- ⚪ Implementation of a garbage collector
- ⚪ Integration with the runtime.

### Error Reporting

- ⚪ Error messages with context and suggestions.
- ⚪ Integration with a language server for IDE support.

### Standard Library

- ⚪ Development of a standard library.

### Module System

- ⚪ Implementation of a module system.

