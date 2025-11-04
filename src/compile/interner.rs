use std::{borrow::Borrow, collections::HashMap, hash::Hash, sync::RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternerId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagId(pub usize);

#[derive(Debug, Clone, Default)]
pub struct Interner<T: Eq + Hash + Clone> {
    pub forward: HashMap<T, usize>,
    backward: Vec<T>,
}

impl<T: Eq + Hash + Clone> Interner<T> {
    pub fn new() -> Interner<T> {
        Interner {
            forward: HashMap::new(),
            backward: vec![],
        }
    }

    pub fn intern<Q>(&mut self, key: &Q) -> InternerId
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq + ToOwned<Owned = T>,
    {
        if let Some(id) = self.forward.get(key) {
            return InternerId(*id);
        }

        let owned_key: T = key.to_owned();
        let id = self.backward.len();
        self.backward.push(owned_key.clone());
        self.forward.insert(owned_key, id);

        InternerId(id)
    }

    pub fn resolve(&self, key: InternerId) -> &T {
        self.backward.get(key.0).unwrap_or_else(|| {
            panic!(
                "INTERNAL COMPILER ERROR: string interner expected key {} to exist",
                key.0
            )
        })
    }
}

#[derive(Default)]
pub struct SharedInterner<T: Eq + Hash + Clone> {
    interner: RwLock<Interner<T>>,
}

impl<T: Eq + Hash + Clone> SharedInterner<T> {
    pub fn intern<Q>(&self, key: &Q) -> InternerId
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq + ToOwned<Owned = T>,
    {
        let reader = self.interner.read().unwrap();
        if let Some(id) = reader.forward.get(key) {
            return InternerId(*id);
        }
        drop(reader);

        let mut writer = self.interner.write().unwrap();
        if let Some(id) = writer.forward.get(key) {
            return InternerId(*id);
        }

        writer.intern(key)
    }

    pub fn resolve(&self, key: InternerId) -> T {
        let reader = self.interner.read().unwrap();
        reader.resolve(key).to_owned()
    }
}

pub type SharedStringInterner = SharedInterner<String>;
pub type SharedTagInterner = SharedInterner<TagId>;
