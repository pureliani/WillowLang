use std::{collections::HashMap, sync::RwLock};

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

pub struct SharedStringInterner {
    interner: RwLock<StringInterner>,
}

impl SharedStringInterner {
    pub fn new() -> Self {
        Self {
            interner: RwLock::new(StringInterner::new()),
        }
    }

    pub fn intern(&self, key: &str) -> InternerId {
        let reader = self.interner.read().unwrap();
        if let Some(id) = reader.forward.get(key) {
            return InternerId(*id);
        }
        drop(reader);

        let mut writer = self.interner.write().unwrap();

        if let Some(id) = writer.forward.get(key) {
            return InternerId(*id);
        }

        let owned_key = key.to_string();
        let id = writer.backward.len();
        writer.backward.push(owned_key.clone());
        writer.forward.insert(owned_key, id);

        InternerId(id)
    }

    pub fn resolve(&self, key: InternerId) -> String {
        let reader = self.interner.read().unwrap();
        reader.resolve(key).to_string()
    }
}
