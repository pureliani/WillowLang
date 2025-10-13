use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternerId(pub usize);

#[derive(Debug, Clone)]
pub struct StringInterner {
    pub forward: HashMap<String, usize>,
    backward: Vec<String>,
}

impl StringInterner {
    pub fn new() -> StringInterner {
        StringInterner {
            forward: HashMap::new(),
            backward: vec![],
        }
    }

    pub fn intern(&mut self, key: &str) -> InternerId {
        if let Some(id) = self.forward.get(key) {
            return InternerId(*id);
        }

        let owned_key = key.to_string();
        let id = self.backward.len();
        self.backward.push(owned_key.clone());
        self.forward.insert(owned_key, id);

        InternerId(id)
    }

    pub fn resolve(&self, key: InternerId) -> &str {
        self.backward.get(key.0).map(|v| v).expect(&format!(
            "INTERNAL COMPILER ERROR: string interner expected key {} to exist",
            key.0
        ))
    }
}
