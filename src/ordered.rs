use std::cmp::Ordering::{self, Equal};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

pub struct Ordered<T, U>(T, U);

impl<T, U> Ordered<T, U> {
    pub fn new(n: T, s: U) -> Ordered<T, U> {
        Ordered(n, s)
    }
}

impl<T, U: Hash> Hash for Ordered<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl<T, U: PartialEq> PartialEq for Ordered<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<T: Eq, U: Eq> Eq for Ordered<T, U> {}

impl<T: PartialOrd, U: PartialOrd> PartialOrd for Ordered<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s_cmp = self.1.partial_cmp(&other.1);
        if s_cmp == Some(Equal) {
            s_cmp
        } else {
            let n_cmp = self.0.partial_cmp(&other.0);
            if n_cmp == Some(Equal) {
                s_cmp
            } else {
                n_cmp
            }
        }
    }
}

impl<T: Ord, U: Ord> Ord for Ordered<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        let s_cmp = self.1.cmp(&other.1);
        if s_cmp == Equal {
            s_cmp
        } else {
            let n_cmp = self.0.cmp(&other.0);
            if n_cmp == Equal {
                s_cmp
            } else {
                n_cmp
            }
        }
    }
}

pub type OrderedString = Ordered<usize, String>;

impl From<OrderedString> for String {
    fn from(value: OrderedString) -> Self {
        value.1
    }
}

impl<'a> From<&'a OrderedString> for &'a str {
    fn from(value: &'a OrderedString) -> Self {
        &value.1
    }
}

impl Deref for OrderedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl AsRef<str> for OrderedString {
    fn as_ref(&self) -> &str {
        &self.1
    }
}
