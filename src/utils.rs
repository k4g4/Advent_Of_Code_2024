use atoi_simd::Parse;
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashMap;
use std::{
    fmt::{self, Display, Formatter},
    hash::Hash,
    ops::Deref,
};

pub enum Answer {
    Finished(i64),
    Unfinished,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Self::Finished(answer) = self {
            write!(f, "{answer}")
        } else {
            Ok(())
        }
    }
}

macro_rules! from_int {
    ($int:ty) => {
        impl From<$int> for Answer {
            fn from(answer: $int) -> Self {
                Self::Finished(answer as _)
            }
        }
    };
}
from_int!(i64);
from_int!(u64);
from_int!(usize);

pub struct Counter<K>(FxHashMap<K, usize>);

impl<K> Default for Counter<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: Eq + Hash> Counter<K> {
    pub fn add(&mut self, key: K) {
        *self.0.entry(key).or_default() += 1;
    }
}

impl<K: Eq + Hash> Extend<K> for Counter<K> {
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for key in iter {
            self.add(key);
        }
    }
}

impl<K: Eq + Hash> FromIterator<K> for Counter<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut counter = Self::default();
        counter.extend(iter);
        counter
    }
}

impl<K: Eq + Hash + Send + Sync> FromParallelIterator<K> for Counter<K> {
    fn from_par_iter<I: IntoParallelIterator<Item = K>>(par_iter: I) -> Self {
        par_iter
            .into_par_iter()
            .fold(Self::default, |mut counter, key| {
                counter.add(key);
                counter
            })
            .reduce(Self::default, |mut acc, counter| {
                for (key, count) in counter.0 {
                    *acc.0.entry(key).or_default() += count;
                }
                acc
            })
    }
}

impl<K> Deref for Counter<K> {
    type Target = FxHashMap<K, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn parse<T: Parse>(s: &str) -> (T, &str) {
    let (t, bytes) = atoi_simd::parse_any(s.as_bytes()).unwrap();
    (t, unsafe { &s.get_unchecked(bytes..) })
}
