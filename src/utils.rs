use atoi_simd::Parse;
use rustc_hash::FxHashMap;
use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    io::{self, Write},
    mem,
    ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign},
    thread,
    time::Duration,
};

pub use itertools::Itertools;
pub use rayon::{
    iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator},
    str::ParallelString,
};
pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
pub use tinyvec::{array_vec, ArrayVec, TinyVec};

#[derive(PartialEq)]
pub enum Answer {
    Number(i64),
    String(String),
    Unfinished,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Unfinished => write!(f, "[unfinished]"),
        }
    }
}

impl From<String> for Answer {
    fn from(answer: String) -> Self {
        Self::String(answer)
    }
}

macro_rules! from_int {
    ($int:ty) => {
        impl From<$int> for Answer {
            fn from(answer: $int) -> Self {
                Self::Number(answer as _)
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

pub const DIRS: [Index; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn dirs((row, column): Index) -> [Index; DIRS.len()] {
    DIRS.map(|(r, c)| (row + r, column + c))
}

pub const DIAGS: [Index; 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

pub fn diags((row, column): Index) -> [Index; DIAGS.len()] {
    DIAGS.map(|(r, c)| (row + r, column + c))
}

pub const ALL_DIRS: [Index; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn all_dirs((row, column): Index) -> [Index; ALL_DIRS.len()] {
    ALL_DIRS.map(|(r, c)| (row + r, column + c))
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Byte(u8);

impl Byte {
    pub fn is_null(&self) -> bool {
        *self == Self(b'\0')
    }

    pub fn make_null(&mut self) {
        *self = Self(b'\0');
    }
}

impl From<u8> for Byte {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl From<Byte> for char {
    fn from(byte: Byte) -> Self {
        char::from(byte.0)
    }
}

impl Deref for Byte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Byte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&char::from(self.0), f)
    }
}

impl PartialEq<u8> for Byte {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<char> for Byte {
    fn eq(&self, other: &char) -> bool {
        char::from(self.0).eq(other)
    }
}

pub trait AsBytes {
    fn as_bytes(&self) -> &[Byte];
}

impl AsBytes for str {
    fn as_bytes(&self) -> &[Byte] {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Debug)]
pub struct Grid<'a>(Box<[&'a [Byte]]>);

impl<'a> Grid<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(input.lines().map(AsBytes::as_bytes).collect())
    }

    pub fn bounds(&self) -> Index {
        (self.0.len() as _, self.0[0].len() as _)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Index, Byte)> + use<'_> {
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

    pub fn get(&self, (row, column): Index) -> Option<Byte> {
        self.0
            .get(row as usize)
            .and_then(|row| row.get(column as usize))
            .copied()
    }

    pub fn print_with(&self, mut f: impl FnMut(Index) -> Option<char>) {
        let mut stdout = io::stdout().lock();
        for ((row, column), byte) in self.iter() {
            if row != 0 && column == 0 {
                stdout.write_all(b"\n").unwrap();
            }
            let char = f((row, column)).unwrap_or(byte.into());
            write!(&mut stdout, "{char}").unwrap();
        }
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();
    }
}

pub struct GridOwned {
    buf: Box<[Byte]>,
    columns: usize,
}

impl GridOwned {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let first = lines.next().unwrap();
        let columns = first.len();
        Self {
            buf: first
                .bytes()
                .chain(lines.flat_map(str::bytes))
                .map_into()
                .collect(),
            columns,
        }
    }

    pub fn bounds(&self) -> Index {
        let rows = self.buf.len() / self.columns;
        (rows as _, self.columns as _)
    }

    pub fn indices(&self) -> impl Iterator<Item = Index> + use<> {
        let (rows, cols) = self.bounds();
        (0..rows).cartesian_product(0..cols)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Index, Byte)> + use<'_> {
        self.indices().zip(self.buf.iter().copied())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Index, &mut Byte)> + use<'_> {
        let (rows, cols) = self.bounds();
        (0..rows)
            .cartesian_product(0..cols)
            .zip(self.buf.iter_mut())
    }

    pub fn get(&self, (row, column): Index) -> Option<Byte> {
        let columns = self.columns as isize;
        (row >= 0 && (0..columns).contains(&column))
            .then(|| self.buf.get((row * columns + column) as usize))
            .flatten()
            .copied()
    }

    pub fn get_mut(&mut self, (row, column): Index) -> Option<&mut Byte> {
        let columns = self.columns as isize;
        (row >= 0 && (0..columns).contains(&column))
            .then(|| self.buf.get_mut((row * columns + column) as usize))
            .flatten()
    }

    pub fn set(&mut self, index: Index, char: char) {
        *self.get_mut(index).unwrap() = (char as u8).into();
    }

    pub fn print_with(&self, mut f: impl FnMut(Index) -> Option<char>) {
        let mut stdout = io::stdout().lock();
        for ((row, column), byte) in self.iter() {
            if row != 0 && column == 0 {
                stdout.write_all(b"\n").unwrap();
            }
            let char = f((row, column)).unwrap_or(byte.into());
            write!(&mut stdout, "{char}").unwrap();
        }
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();
    }
}

#[derive(Debug)]
pub struct Counter<K>(FxHashMap<K, u64>);

impl<K> Default for Counter<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: Eq + Hash> Counter<K> {
    pub fn add(&mut self, key: K) {
        self.add_n(key, 1);
    }

    pub fn add_n(&mut self, key: K, n: u64) {
        *self.0.entry(key).or_default() += n;
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
    type Target = FxHashMap<K, u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn parse<T: Parse>(s: &str) -> (T, &str) {
    let (t, bytes) = atoi_simd::parse_any(s.as_bytes()).unwrap();
    (t, unsafe { s.get_unchecked(bytes..) })
}

pub fn digits(mut num: u64) -> u32 {
    if num == 0 {
        1
    } else {
        let mut num_len = 0;
        while num > 0 {
            num /= 10;
            num_len += 1;
        }
        num_len
    }
}

macro_rules! gcd_impl {
    ($f:ident($t:ty)) => {
        pub fn $f(a: $t, b: $t) -> $t {
            let (mut a, mut b) = (a.abs_diff(0), b.abs_diff(0));
            let mut d = 0;
            loop {
                match (a % 2, b % 2) {
                    (0, 0) => {
                        a /= 2;
                        b /= 2;
                        d += 1;
                    }
                    (0, 1) => a /= 2,
                    (1, 0) => b /= 2,
                    (1, 1) if a > b => a -= b,
                    (1, 1) if a < b => b -= a,
                    _ => break (a as u64 * 2u64.pow(d)) as _,
                }
            }
        }
    };
}
gcd_impl!(gcd_i32(i32));
gcd_impl!(gcd_u32(u32));
gcd_impl!(gcd_i64(i64));
gcd_impl!(gcd_u64(u64));
gcd_impl!(gcd_isize(isize));
gcd_impl!(gcd_usize(usize));
