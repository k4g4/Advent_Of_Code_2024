use rustc_hash::FxHashMap;
use std::{hash::Hash, ops::Deref};

pub struct Freqs<K>(FxHashMap<K, usize>);

impl<K> Default for Freqs<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: Eq + Hash> Freqs<K> {
    pub fn add(&mut self, key: K) {
        *self.0.entry(key).or_default() += 1;
    }
}

impl<K: Eq + Hash> FromIterator<K> for Freqs<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut freqs = Self::default();
        for key in iter {
            freqs.add(key);
        }
        freqs
    }
}

impl<K> Deref for Freqs<K> {
    type Target = FxHashMap<K, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
