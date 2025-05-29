use super::{is_alphanumeric, Tokenizer};

impl<'a> Tokenizer<'a> {
    pub fn tokenize_identifier(&mut self) -> &'a str {
        let start = self.grapheme_offset;
        while let Some(c) = self.current() {
            if is_alphanumeric(c) || c == "_" {
                self.consume();
            } else {
                break;
            }
        }

        self.slice(start, self.grapheme_offset)
    }
}
