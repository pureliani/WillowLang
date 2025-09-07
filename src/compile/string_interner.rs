use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternerId(pub usize);

#[derive(Debug, Clone)]
pub struct StringInterner<'a> {
    pub forward: HashMap<&'a str, usize>,
    backward: Vec<&'a str>,
    next_id: usize,
}

impl<'a> StringInterner<'a> {
    pub fn new() -> StringInterner<'a> {
        StringInterner {
            forward: HashMap::new(),
            backward: vec![],
            next_id: 0,
        }
    }

    pub fn from(value: impl IntoIterator<Item = &'a str>) -> StringInterner<'a> {
        let mut interner = Self::new();

        value.into_iter().for_each(|v| {
            interner.intern(v);
        });

        interner
    }

    pub fn intern(&mut self, key: &'a str) -> InternerId {
        match self.forward.entry(key) {
            Entry::Occupied(v) => InternerId(*v.get()),
            Entry::Vacant(e) => {
                let id = self.next_id;

                e.insert(id);
                self.backward.push(key);

                self.next_id += 1;
                InternerId(id)
            }
        }
    }

    pub fn resolve(&self, key: InternerId) -> &'a str {
        self.backward.get(key.0).map(|v| *v).expect(&format!(
            "INTERNAL COMPILER ERROR: string interner expected key {} to exist",
            key.0
        ))
    }
}
