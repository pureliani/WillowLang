use pretty_assertions::assert_eq;
use willow::{
    ast::{Position, Span},
    tokenizer::{NumberKind, Token, TokenKind, Tokenizer},
};

#[test]
fn tokenizes_numbers_with_suffixes() {
    let test_cases = vec![
        (
            "1.",
            NumberKind::F64(1.0f64),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 3,
                    byte_offset: 2,
                },
            },
        ),
        (
            "1.5",
            NumberKind::F64(1.5f64),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 4,
                    byte_offset: 3,
                },
            },
        ),
        (
            "1",
            NumberKind::I64(1i64),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 2,
                    byte_offset: 1,
                },
            },
        ),
        (
            "1.5f64",
            NumberKind::F64(1.5f64),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 7,
                    byte_offset: 6,
                },
            },
        ),
        (
            "1.5f32",
            NumberKind::F32(1.5f32),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 7,
                    byte_offset: 6,
                },
            },
        ),
        (
            "1f64",
            NumberKind::F64(1f64),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 5,
                    byte_offset: 4,
                },
            },
        ),
        (
            "1f32",
            NumberKind::F32(1f32),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 5,
                    byte_offset: 4,
                },
            },
        ),
        (
            "1u8",
            NumberKind::U8(1u8),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 4,
                    byte_offset: 3,
                },
            },
        ),
        (
            "1u16",
            NumberKind::U16(1u16),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 5,
                    byte_offset: 4,
                },
            },
        ),
        (
            "1u32",
            NumberKind::U32(1u32),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 5,
                    byte_offset: 4,
                },
            },
        ),
        (
            "1u64",
            NumberKind::U64(1u64),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 5,
                    byte_offset: 4,
                },
            },
        ),
        (
            "1i8",
            NumberKind::I8(1i8),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 4,
                    byte_offset: 3,
                },
            },
        ),
        (
            "1i16",
            NumberKind::I16(1i16),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 5,
                    byte_offset: 4,
                },
            },
        ),
        (
            "1i32",
            NumberKind::I32(1i32),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 5,
                    byte_offset: 4,
                },
            },
        ),
        (
            "1i64",
            NumberKind::I64(1i64),
            Span {
                start: Position {
                    line: 1,
                    col: 1,
                    byte_offset: 0,
                },
                end: Position {
                    line: 1,
                    col: 5,
                    byte_offset: 4,
                },
            },
        ),
    ];

    for (input, expected_kind, span) in test_cases {
        let (tokens, _) = Tokenizer::tokenize(input);

        assert_eq!(
            tokens,
            vec![Token {
                span,
                kind: TokenKind::Number(expected_kind),
            }]
        );
    }
}
