use willow::{
    ast::{Position, Span},
    tokenizer::{KeywordKind, Token, TokenKind, Tokenizer},
};

#[test]
fn tokenizes_simple_identifiers() {
    let tokens = Tokenizer::tokenize("hello".to_owned());

    assert_eq!(
        tokens,
        vec![Token {
            kind: TokenKind::Identifier("hello".to_owned()),
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 6 }
            }
        }]
    )
}

#[test]
fn tokenizes_keywords() {
    let tokens = Tokenizer::tokenize("struct".to_owned());

    assert_eq!(
        tokens,
        vec![Token {
            kind: TokenKind::Keyword(KeywordKind::Struct),
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 7 }
            }
        }]
    )
}

#[test]
fn tokenizes_sequence_as_identifier() {
    let tokens = Tokenizer::tokenize("\nstructhello".to_owned());

    assert_eq!(
        tokens,
        vec![Token {
            kind: TokenKind::Identifier("structhello".to_owned()),
            span: Span {
                start: Position { line: 2, col: 1 },
                end: Position { line: 2, col: 12 }
            }
        }]
    )
}
