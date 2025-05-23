use super::{TokenizationErrorKind, Tokenizer};

impl<'a> Tokenizer<'a> {
    pub fn tokenize_documentation(&mut self) -> Result<String, TokenizationErrorKind> {
        self.consume();
        self.consume();
        self.consume();

        let start = self.grapheme_offset;
        while let Some(c) = self.current() {
            if c == "-" && self.peek(1) == Some("-") && self.peek(2) == Some("-") {
                let doc_content = self.slice(start, self.grapheme_offset).to_owned();
                self.consume();
                self.consume();
                self.consume();
                return Ok(doc_content);
            }
            self.consume();
        }

        Err(TokenizationErrorKind::UnterminatedDoc)
    }
}
