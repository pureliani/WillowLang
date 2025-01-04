use super::{TokenizationError, Tokenizer};

impl Tokenizer {
    pub fn tokenize_documentation(&mut self) -> Result<String, TokenizationError> {
        self.consume();
        self.consume();
        self.consume();

        let start = self.offset;
        while let Some(c) = self.current() {
            if c == "-" && self.peek(1) == Some("-") && self.peek(2) == Some("-") {
                let doc_content = self.slice(start, self.offset).to_owned();
                self.consume();
                self.consume();
                self.consume();
                return Ok(doc_content);
            }
            self.consume();
        }

        Err(TokenizationError::UnterminatedDoc)
    }
}
