use atoi_simd::Parse;
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashMap;
use std::{
    fmt::{self, Display, Formatter},
    hash::Hash,
    io::{self, Write},
    ops::{Add, AddAssign, Deref, Sub, SubAssign},
    thread,
    time::Duration,
};

pub use itertools::Itertools;
pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
pub use tinyvec::{array_vec, ArrayVec, TinyVec};

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
from_int!(i32);
from_int!(u32);
from_int!(i64);
from_int!(u64);
from_int!(isize);
from_int!(usize);

pub fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Default, Debug)]
#[repr(u8)]
pub enum Dir {
    #[default]
    North = b'^',
    East = b'>',
    South = b'v',
    West = b'<',
}

impl Dir {
    pub fn clockwise(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn counter_clockwise(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

pub type Index = (isize, isize);

impl Add<Dir> for Index {
    type Output = Index;

    fn add(self, dir: Dir) -> Self::Output {
        let (row, column) = self;
        match dir {
            Dir::North => (row - 1, column),
            Dir::East => (row, column + 1),
            Dir::South => (row + 1, column),
            Dir::West => (row, column - 1),
        }
    }
}

impl AddAssign<Dir> for Index {
    fn add_assign(&mut self, dir: Dir) {
        *self = *self + dir;
    }
}

impl Sub<Dir> for Index {
    type Output = Index;

    fn sub(self, dir: Dir) -> Self::Output {
        let (row, column) = self;
        match dir {
            Dir::North => (row + 1, column),
            Dir::East => (row, column - 1),
            Dir::South => (row - 1, column),
            Dir::West => (row, column + 1),
        }
    }
}

impl SubAssign<Dir> for Index {
    fn sub_assign(&mut self, dir: Dir) {
        *self = *self - dir;
    }
}

pub const DIRS: [Index; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn dirs((row, column): Index) -> [Index; DIRS.len()] {
    DIRS.map(|(r, c)| (row + r, column + c))
}

pub const DIAGS: [Index; 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

pub fn diags((row, column): Index) -> [Index; DIAGS.len()] {
    DIAGS.map(|(r, c)| (row + r, column + c))
}

#[derive(Debug)]
pub struct Grid<'a>(Vec<&'a [u8]>);

impl<'a> Grid<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(input.lines().map(str::as_bytes).collect())
    }

    pub fn indices(&self) -> impl Iterator<Item = (Index, u8)> + use<'_> {
        unsafe {
            (0..self.0.len()).flat_map(move |row| {
                (0..self.0.get_unchecked(row).len()).map(move |column| {
                    (
                        (row as _, column as _),
                        *self.0.get_unchecked(row).get_unchecked(column),
                    )
                })
            })
        }
    }

    pub fn get(&self, (row, column): Index) -> Option<u8> {
        self.0
            .get(row as usize)
            .and_then(|row| row.get(column as usize))
            .copied()
    }

    pub fn print_with(&self, mut f: impl FnMut(Index) -> Option<u8>) {
        let mut stdout = io::stdout().lock();
        for ((row, column), byte) in self.indices() {
            if row != 0 && column == 0 {
                stdout.write_all(b"\n").unwrap();
            }
            let byte = f((row, column)).unwrap_or(byte);
            stdout.write_all(&[byte]).unwrap();
        }
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();
    }
}

pub struct GridIter<'a> {
    curr_row: &'a [u8],
    rem_rows: &'a [&'a [u8]],
}

impl Iterator for GridIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let [first, rest @ ..] = self.curr_row {
            self.curr_row = rest;
            Some(*first)
        } else if let [[first_first, first_rest @ ..], rest @ ..] = self.rem_rows {
            self.curr_row = first_rest;
            self.rem_rows = rest;
            Some(*first_first)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Grid<'a> {
    type Item = u8;
    type IntoIter = GridIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let (first, rest) = self.0.split_first().unwrap();
        GridIter {
            curr_row: first,
            rem_rows: rest,
        }
    }
}

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
    (t, unsafe { s.get_unchecked(bytes..) })
}
