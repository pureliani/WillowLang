use super::{is_alphanumeric, Tokenizer};

impl Tokenizer {
    pub fn tokenize_identifier(&mut self) -> String {
        let start = self.offset;
        while let Some(c) = self.current() {
            if is_alphanumeric(c) || c == "_" {
                self.consume();
            } else {
                break;
            }
        }

        self.slice(start, self.offset).to_owned()
    }
}
