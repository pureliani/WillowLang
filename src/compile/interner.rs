use std::{
    borrow::Borrow, collections::HashMap, hash::Hash, marker::PhantomData, sync::RwLock,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(pub usize);

impl Id for StringId {
    fn from_usize(id: usize) -> Self {
        StringId(id)
    }
    fn as_usize(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagId(pub usize);

impl Id for TagId {
    fn from_usize(id: usize) -> Self {
        TagId(id)
    }
    fn as_usize(&self) -> usize {
        self.0
    }
}

pub trait Id: Copy + Eq + Hash {
    fn from_usize(id: usize) -> Self;
    fn as_usize(&self) -> usize;
}

#[derive(Debug, Clone, Default)]
pub struct Interner<T: Eq + Hash + Clone, I: Id> {
    pub forward: HashMap<T, usize>,
    backward: Vec<T>,
    _marker: PhantomData<I>,
}

impl<T, I> Interner<T, I>
where
    T: Eq + Hash + Clone,
    I: Id,
{
    pub fn new() -> Self {
        Interner {
            forward: HashMap::new(),
            backward: vec![],
            _marker: PhantomData,
        }
    }

    pub fn intern<Q>(&mut self, key: &Q) -> I
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq + ToOwned<Owned = T>,
    {
        if let Some(id) = self.forward.get(key) {
            return I::from_usize(*id);
        }

        let owned_key: T = key.to_owned();
        let id = self.backward.len();
        self.backward.push(owned_key.clone());
        self.forward.insert(owned_key, id);

        I::from_usize(id)
    }

    pub fn resolve(&self, key: I) -> &T {
        self.backward.get(key.as_usize()).unwrap_or_else(|| {
            panic!(
                "INTERNAL COMPILER ERROR: interner expected key {} to exist",
                key.as_usize()
            )
        })
    }
}

#[derive(Default)]
pub struct SharedInterner<T, I>
where
    T: Eq + Hash + Clone,
    I: Id,
{
    interner: RwLock<Interner<T, I>>,
}

impl<T, I> SharedInterner<T, I>
where
    T: Eq + Hash + Clone,
    I: Id,
{
    pub fn intern<Q>(&self, key: &Q) -> I
    where
        T: Borrow<Q>,
        Q: ?Sized + Hash + Eq + ToOwned<Owned = T>,
    {
        let reader = self.interner.read().unwrap();
        if let Some(id) = reader.forward.get(key) {
            return I::from_usize(*id);
        }
        drop(reader);

        let mut writer = self.interner.write().unwrap();
        if let Some(id) = writer.forward.get(key) {
            return I::from_usize(*id);
        }

        writer.intern(key)
    }

    pub fn resolve(&self, key: I) -> T {
        let reader = self.interner.read().unwrap();
        reader.resolve(key).to_owned()
    }
}

pub type SharedStringInterner = SharedInterner<String, StringId>;
pub type SharedTagInterner = SharedInterner<StringId, TagId>;
