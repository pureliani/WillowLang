use std::collections::HashMap;

use ariadne::{Cache, Source};

#[derive(Debug, Clone)]
pub struct FileSourceCache {
    sources: HashMap<String, Source>,
}

impl FileSourceCache {
    pub fn new() -> Self {
        FileSourceCache {
            sources: HashMap::new(),
        }
    }
    pub fn add(&mut self, id: String, source_str: String) {
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
