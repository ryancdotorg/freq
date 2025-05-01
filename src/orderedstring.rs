use std::cmp::Ordering::{self, Equal};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

pub struct OrderedString<T>(T, String);

impl<T> OrderedString<T> {
    pub fn new(n: T, s: String) -> OrderedString<T> {
        OrderedString(n, s)
    }
}

impl<T> From<OrderedString<T>> for String {
    fn from(value: OrderedString<T>) -> Self {
        value.1
    }
}

impl<'a, T> From<&'a OrderedString<T>> for &'a str {
    fn from(value: &'a OrderedString<T>) -> Self {
        &value.1
    }
}

impl<T> Deref for OrderedString<T> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T> Hash for OrderedString<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl<T> PartialEq for OrderedString<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<T: Eq> Eq for OrderedString<T> {}

impl<T: PartialOrd> PartialOrd for OrderedString<T> {
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

impl<T: Ord> Ord for OrderedString<T> {
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
