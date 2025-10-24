use ariadne::{Cache, Source};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct FileCache {
    sources: HashMap<PathBuf, Source>,
}

impl Cache<PathBuf> for FileCache {
    type Storage = String;

    fn fetch(
        &mut self,
        id: &PathBuf,
    ) -> Result<
        &ariadne::Source<<Self as ariadne::Cache<std::path::PathBuf>>::Storage>,
        impl std::fmt::Debug,
    > {
        self.sources
            .get(id)
            .ok_or_else(|| Box::new("File not found") as Box<dyn std::fmt::Debug>)
    }

    fn display<'a>(&self, id: &'a PathBuf) -> Option<impl std::fmt::Display + 'a> {
        Some(id.display())
    }
}

impl FileCache {
    pub fn insert(
        &mut self,
        path: PathBuf,
        source: String,
    ) -> Option<self::Source<String>> {
        self.sources.insert(path, Source::from(source))
    }
}
