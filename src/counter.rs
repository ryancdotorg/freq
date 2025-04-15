//! Derived from v0.5.7 of the `counter` crate, but tracks insertion order
//! https://docs.rs/counter/0.5.7/counter/
//!
//! Counter counts recurrent elements of iterables. It is based on [the Python
//! implementation](https://docs.python.org/3/library/collections.html#collections.Counter).
//!
//! The struct [`Counter`](struct.Counter.html) is the entry-point type for this module.
//!
//! # Math Underpinnings
//!
//! Mathematically, a `Counter` implements a hash-based version of a [multiset],
//! or bag. This is simply an extension of the notion of a set to the idea that
//! we care not only about whether an entity exists within the set, but the number
//! of occurrences within the set. Normal set operations such as intersection,
//! union, etc. are of course still supported.
//!
//! [multiset]: https://en.wikipedia.org/wiki/Set_(abstract_data_type)#Multiset
//!
//! # Examples
//!
//! ## Just count an iterable
//!
//! ```rust
//! use counter::Counter;
//! let char_counts = "barefoot".chars().collect::<Counter<_>>();
//! let counts_counts = char_counts.values().collect::<Counter<_>>();
//! ```
//!
//! ## Update a count
//!
//! ```rust
//! # use counter::Counter;
//! let mut counts = "aaa".chars().collect::<Counter<_>>();
//! counts[&'a'] += 1;
//! counts[&'b'] += 1;
//! ```
//!
//! ```rust
//! # use counter::Counter;
//! let mut counts = "able babble table babble rabble table able fable scrabble"
//!     .split_whitespace().collect::<Counter<_>>();
//! // add or subtract an iterable of the same type
//! counts += "cain and abel fable table cable".split_whitespace();
//! // or add or subtract from another Counter of the same type
//! let other_counts = "scrabble cabbie fable babble"
//!     .split_whitespace().collect::<Counter<_>>();
//! let difference = counts - other_counts;
//! ```
//!
//! Extend a `Counter` with another `Counter`:
//! ```rust
//! # use counter::Counter;
//! # use std::collections::HashMap;
//! let mut counter = "abbccc".chars().collect::<Counter<_>>();
//! let another = "bccddd".chars().collect::<Counter<_>>();
//! counter.extend(&another);
//! let expect = [('a', 1), ('b', 3), ('c', 5), ('d', 3)].iter()
//!     .cloned().collect::<HashMap<_, _>>();
//! assert_eq!(counter.into_map(), expect);
//! ```
//! ## Get items with keys
//!
//! ```rust
//! # use counter::Counter;
//! let counts = "aaa".chars().collect::<Counter<_>>();
//! assert_eq!(counts[&'a'], 3);
//! assert_eq!(counts[&'b'], 0);
//! ```
//!
//! ## Get the most common items
//!
//! [`most_common_ordered()`] uses the natural ordering of keys which are [`Ord`].
//!
//! [`most_common_ordered()`]: Counter::most_common_ordered
//! [`Ord`]: https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html
//!
//! ```rust
//! # use counter::Counter;
//! let by_common = "eaddbbccc".chars().collect::<Counter<_>>().most_common_ordered();
//! let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
//! assert!(by_common == expected);
//! ```
//!
//! [`k_most_common_ordered()`] takes an argument `k` of type `usize` and returns the top `k` most
//! common items.  This is functionally equivalent to calling `most_common_ordered()` and then
//! truncating the result to length `k`.  However, if `k` is smaller than the length of the counter
//! then `k_most_common_ordered()` can be more efficient, often much more so.
//!
//! ```rust
//! # use counter::Counter;
//! let by_common = "eaddbbccc".chars().collect::<Counter<_>>().k_most_common_ordered(2);
//! let expected = vec![('c', 3), ('b', 2)];
//! assert!(by_common == expected);
//! ```
//!
//! [`k_most_common_ordered()`]: Counter::k_most_common_ordered
//! [`most_common_ordered()`]: Counter::most_common_ordered
//!
//! ## Get the most common items using your own ordering
//!
//! For example, here we break ties reverse alphabetically.
//!
//! ```rust
//! # use counter::Counter;
//! let counter = "eaddbbccc".chars().collect::<Counter<_>>();
//! let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
//! let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
//! assert!(by_common == expected);
//! ```
//!
//! ## Test counters against another
//!
//! Counters are multi-sets and so can be sub- or supersets of each other.
//!
//! A counter is a _subset_ of another if for all its elements, the other
//! counter has an equal or higher count. Test for this with [`is_subset()`]:
//!
//! ```rust
//! # use counter::Counter;
//! let counter = "aaabb".chars().collect::<Counter<_>>();
//! let superset = "aaabbbc".chars().collect::<Counter<_>>();
//! let not_a_superset = "aaae".chars().collect::<Counter<_>>();
//! assert!(counter.is_subset(&superset));
//! assert!(!counter.is_subset(&not_a_superset));
//! ```
//!
//! Testing for a _superset_ is the inverse, [`is_superset()`] is true if the counter can contain another counter in its entirety:
//!
//! ```rust
//! # use counter::Counter;
//! let counter = "aaabbbc".chars().collect::<Counter<_>>();
//! let subset = "aabbb".chars().collect::<Counter<_>>();
//! let not_a_subset = "aaae".chars().collect::<Counter<_>>();
//! assert!(counter.is_superset(&subset));
//! assert!(!counter.is_superset(&not_a_subset));
//! ```
//!
//! These relationships continue to work when [using a _signed_ integer type for the counter][signed]: all values in the subset must be equal or lower to the values in the superset. Negative
//! values are interpreted as 'missing' those values, and the subset would need to miss those
//! same elements, or be short more, to still be a subset:
//!
//! ```rust
//! # use counter::Counter;
//! let mut subset = "aaabb".chars().collect::<Counter<_, i8>>();
//! subset.insert('e', -2);  // short 2 'e's
//! subset.insert('f', -1);  // and 1 'f'
//! let mut superset = "aaaabbb".chars().collect::<Counter<_, i8>>();
//! superset.insert('e', -1);  // short 1 'e'
//! assert!(subset.is_subset(&superset));
//! assert!(superset.is_superset(&subset));
//! ```
//!
//! [`is_subset()`]: Counter::is_subset
//! [`is_superset()`]: Counter::is_superset
//! [signed]: #use-your-own-type-for-the-count
//!
//! ## Counter intersection and union
//!
//! You can intersect two counters, giving you the minimal counts of their
//! combined elements using the [`&` bitwise and operator][BitAnd], and produce
//! their union with the maximum counts using [`|` bitwise or][BitOr]:
//!
//! ```rust
//! # use counter::Counter;
//! let a = "aaabb".chars().collect::<Counter<_>>();
//! let b = "aabbbbe".chars().collect::<Counter<_>>();
//!
//! let intersection = a & b;
//! let expected_intersection = "aabb".chars().collect::<Counter<_>>();
//! assert_eq!(intersection, expected_intersection);
//!
//! let c = "aaabb".chars().collect::<Counter<_>>();
//! let d = "aabbbbe".chars().collect::<Counter<_>>();
//!
//! let union = c | d;
//! let expected_union = "aaabbbbe".chars().collect::<Counter<_>>();
//! assert_eq!(union, expected_union)
//! ```
//!
//! The in-place [`&=`] and [`|=`] operations are also supported.
//!
//! [BitAnd]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
//! [BitOr]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
//! [`&=`]: https://doc.rust-lang.org/std/ops/trait.BitAndAssign.html
//! [`|=`]: https://doc.rust-lang.org/std/ops/trait.BitOrAssign.html
//!
//! ## Treat it like a `HashMap`
//!
//! `Counter<T, N>` implements [`Deref`]`<Target=HashMap<T, N>>` and
//! [`DerefMut`]`<Target=HashMap<T, N>>`, which means that you can perform any operations
//! on it which are valid for a [`HashMap`].
//!
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//! [`Deref`]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
//! [`DerefMut`]: https://doc.rust-lang.org/stable/std/ops/trait.DerefMut.html
//!
//! ```rust
//! # use counter::Counter;
//! let mut counter = "aa-bb-cc".chars().collect::<Counter<_>>();
//! counter.remove(&'-');
//! assert!(counter == "aabbcc".chars().collect::<Counter<_>>());
//! ```
//!
//! Note that `Counter<T, N>` itself implements [`Index`]. `Counter::index` returns a reference to
//! a [`Zero::zero`] value for missing keys.
//!
//! [`Index`]: https://doc.rust-lang.org/stable/std/ops/trait.Index.html
//! [`Zero::zero`]: https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html#tymethod.zero
//!
//! ```rust
//! # use counter::Counter;
//! let counter = "aaa".chars().collect::<Counter<_>>();
//! assert_eq!(counter[&'b'], 0);
//! // panics
//! // assert_eq!((*counter)[&'b'], 0);
//! ```
//!
//! # Advanced Usage
//!
//! ## Count any iterable which is `Hash + Eq`
//!
//! You can't use the `most_common*` functions unless `T` is also [`Clone`], but simple counting
//! works fine on a minimal data type.
//!
//! [`Clone`]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
//!
//! ```rust
//! # use counter::Counter;
//! #[derive(Debug, Hash, PartialEq, Eq)]
//! struct Inty {
//!     i: usize,
//! }
//!
//! impl Inty {
//!     pub fn new(i: usize) -> Inty {
//!         Inty { i: i }
//!     }
//! }
//!
//! // <https://en.wikipedia.org/wiki/867-5309/Jenny>
//! let intys = vec![
//!     Inty::new(8),
//!     Inty::new(0),
//!     Inty::new(0),
//!     Inty::new(8),
//!     Inty::new(6),
//!     Inty::new(7),
//!     Inty::new(5),
//!     Inty::new(3),
//!     Inty::new(0),
//!     Inty::new(9),
//! ];
//!
//! let inty_counts = intys.iter().collect::<Counter<_>>();
//! println!("{:?}", inty_counts);
//! // {Inty { i: 8 }: 2, Inty { i: 0 }: 3, Inty { i: 9 }: 1, Inty { i: 3 }: 1,
//! //  Inty { i: 7 }: 1, Inty { i: 6 }: 1, Inty { i: 5 }: 1}
//! assert!(inty_counts.get(&Inty { i: 8 }) == Some(&2));
//! assert!(inty_counts.get(&Inty { i: 0 }) == Some(&3));
//! assert!(inty_counts.get(&Inty { i: 6 }) == Some(&1));
//! ```
//!
//! ## Use your own type for the count
//!
//! Sometimes [`usize`] just isn't enough. If you find yourself overflowing your
//! machine's native size, you can use your own type. Here, we use an [`i8`], but
//! you can use most numeric types, including bignums, as necessary.
//!
//! [`usize`]: https://doc.rust-lang.org/stable/std/primitive.usize.html
//! [`i8`]: https://doc.rust-lang.org/stable/std/primitive.i8.html
//!
//! ```rust
//! # use counter::Counter;
//! # use std::collections::HashMap;
//! let counter: Counter<_, i8> = "abbccc".chars().collect();
//! let expected: HashMap<char, i8> = [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect();
//! assert!(counter.into_map() == expected);
//! ```

use num_traits::{One, Zero};

use std::borrow::Borrow;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::iter;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Deref, DerefMut, Index, IndexMut,
    Sub, SubAssign,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct OrderedCount<N = usize> {
    count: N,
    order: u64,
}

impl<N> AddAssign<N> for OrderedCount<N> {
    fn add_assign(&mut self, other: N) {
        *self = Self {
            count: self.count + other,
            order: self.order,
        };
    }
}

type CounterMap<T, N, C> = HashMap<T, (N, C)>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Counter<T: Hash + Eq, N = usize, C = usize> {
    map: CounterMap<T, (N, C)>,
    index: N,
    // necessary for `Index::index` since we cannot declare generic `static` variables.
    zero: C,
}

impl<T, N, C> Counter<T, N, C>
where
    T: Hash + Eq,
{
    /// Consumes this counter and returns a [`HashMap`] mapping the items to the counts.
    ///
    /// [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
    pub fn into_map(self) -> HashMap<T, N, C> {
        self.map
    }

    /// Returns the sum of the counts.
    ///
    /// Use [`len`] to get the number of elements in the counter and use `total` to get the sum of
    /// their counts.
    ///
    /// [`len`]: struct.Counter.html#method.len
    ///
    /// # Examples
    ///
    /// ```
    /// # use counter::Counter;
    /// let counter = Counter::init("abracadabra".chars());
    /// assert_eq!(counter.total::<usize>(), 11);
    /// assert_eq!(counter.len(), 5);
    /// ```
    pub fn total<'a, S>(&'a self) -> S
    where
        S: iter::Sum<&'a N>,
    {
        self.map.values().sum()
    }
}

impl<T, N, C> Counter<T, N, C>
where
    T: Hash + Eq,
    N: Zero,
    C: Zero,
{
    /// Create a new, empty `Counter`
    pub fn new() -> Counter<T, N, C> {
        Counter {
            map: HashMap::new(),
            index: N::zero(),
            zero: C::zero(),
        }
    }
}

impl<T, N, C> Counter<T, N, C>
where
    T: Hash + Eq,
    N: AddAssign + Zero + One,
    C: AddAssign + Zero + One,
{
    /// Create a new `Counter` initialized with the given iterable.
    pub fn init<I>(iterable: I) -> Counter<T, N>
    where
        I: IntoIterator<Item = T>,
    {
        let mut counter = Counter::new();
        counter.update(iterable);
        counter
    }

    /// Add the counts of the elements from the given iterable to this counter.
    pub fn update<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iterable {
            let entry = self.map.entry(item).or_insert_with(N::zero);
            *entry += N::one();
        }
    }
}

impl<T, N> Counter<T, N>
where
    T: Hash + Eq,
    N: PartialOrd + SubAssign + Zero + One,
{
    /// Remove the counts of the elements from the given iterable to this counter.
    ///
    /// Non-positive counts are automatically removed.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut counter = "abbccc".chars().collect::<Counter<_>>();
    /// counter.subtract("abba".chars());
    /// let expect = [('c', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    pub fn subtract<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iterable {
            let mut remove = false;
            if let Some(entry) = self.map.get_mut(&item) {
                if *entry > N::zero() {
                    *entry -= N::one();
                }
                remove = *entry == N::zero();
            }
            if remove {
                self.map.remove(&item);
            }
        }
    }
}

impl<T, N> Counter<T, N>
where
    T: Hash + Eq + Clone,
    N: Clone + Ord,
{
    /// Create a vector of `(elem, frequency)` pairs, sorted most to least common.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// let mc = "pappaopolo".chars().collect::<Counter<_>>().most_common();
    /// let expected = vec![('p', 4), ('o', 3), ('a', 2), ('l', 1)];
    /// assert_eq!(mc, expected);
    /// ```
    ///
    /// Note that the ordering of duplicates is unstable.
    pub fn most_common(&self) -> Vec<(T, N)> {
        use std::cmp::Ordering;
        self.most_common_tiebreaker(|_a, _b| Ordering::Equal)
    }

    /// Create a vector of `(elem, frequency)` pairs, sorted most to least common.
    ///
    /// In the event that two keys have an equal frequency, use the supplied ordering function
    /// to further arrange the results.
    ///
    /// For example, we can sort reverse-alphabetically:
    ///
    /// ```rust
    /// # use counter::Counter;
    /// let counter = "eaddbbccc".chars().collect::<Counter<_>>();
    /// let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
    /// let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
    /// assert_eq!(by_common, expected);
    /// ```
    pub fn most_common_tiebreaker<F>(&self, mut tiebreaker: F) -> Vec<(T, N)>
    where
        F: FnMut(&T, &T) -> ::std::cmp::Ordering,
    {
        let mut items = self
            .map
            .iter()
            .map(|(key, count)| (key.clone(), count.clone()))
            .collect::<Vec<_>>();
        items.sort_unstable_by(|(a_item, a_count), (b_item, b_count)| {
            b_count
                .cmp(a_count)
                .then_with(|| tiebreaker(a_item, b_item))
        });
        items
    }
}

impl<T, N> Counter<T, N>
where
    T: Hash + Eq + Clone + Ord,
    N: Clone + Ord,
{
    /// Create a vector of `(elem, frequency)` pairs, sorted most to least common.
    ///
    /// In the event that two keys have an equal frequency, use the natural ordering of the keys
    /// to further sort the results.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use counter::Counter;
    /// let mc = "abracadabra".chars().collect::<Counter<_>>().most_common_ordered();
    /// let expect = vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)];
    /// assert_eq!(mc, expect);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(*n* \* log *n*), where *n* is the number of items in the counter.  If all you want is
    /// the top *k* items and *k* < *n* then it can be more efficient to use
    /// [`k_most_common_ordered`].
    ///
    /// [`k_most_common_ordered`]: Counter::k_most_common_ordered
    pub fn most_common_ordered(&self) -> Vec<(T, N)> {
        self.most_common_tiebreaker(Ord::cmp)
    }

    /// Returns the `k` most common items in decreasing order of their counts.
    ///
    /// The returned vector is the same as would be obtained by calling `most_common_ordered` and
    /// then truncating the result to length `k`.  In particular, items with the same count are
    /// sorted in *increasing* order of their keys.  Further, if `k` is greater than the length of
    /// the counter then the returned vector will have length equal to that of the counter, not
    /// `k`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use counter::Counter;
    /// let counter: Counter<_> = "abracadabra".chars().collect();
    /// let top3 = counter.k_most_common_ordered(3);
    /// assert_eq!(top3, vec![('a', 5), ('b', 2), ('r', 2)]);
    /// ```
    ///
    /// # Time complexity
    ///
    /// This method can be much more efficient than [`most_common_ordered`] when *k* is much
    /// smaller than the length of the counter *n*.  When *k* = 1 the algorithm is equivalent
    /// to finding the minimum (or maximum) of *n* items, which requires *n* \- 1 comparisons.  For
    /// a fixed value of *k* > 1, the number of comparisons scales with *n* as *n* \+ *O*(log *n*)
    /// and the number of swaps scales as *O*(log *n*).  As *k* approaches *n*, this algorithm
    /// approaches a heapsort of the *n* items, which has complexity *O*(*n* \* log *n*).
    ///
    /// For values of *k* close to *n* the sorting algorithm used by [`most_common_ordered`] will
    /// generally be faster than the heapsort used by this method by a small constant factor.
    /// Exactly where the crossover point occurs will depend on several factors.  For small *k*
    /// choose this method.  If *k* is a substantial fraction of *n*, it may be that
    /// [`most_common_ordered`] is faster.  If performance matters in your application then it may
    /// be worth experimenting to see which of the two methods is faster.
    ///
    /// [`most_common_ordered`]: Counter::most_common_ordered
    pub fn k_most_common_ordered(&self, k: usize) -> Vec<(T, N)> {
        use std::cmp::Reverse;

        if k == 0 {
            return vec![];
        }

        // The quicksort implementation used by `most_common_ordered()` is generally faster than
        // the heapsort used below when sorting the entire counter.
        if k >= self.map.len() {
            return self.most_common_ordered();
        }

        // Clone the counts as we iterate over the map to eliminate an extra indirection when
        // comparing counts.  This will be an improvement in the typical case where `N: Copy`.
        // Defer cloning the keys until we have selected the top `k` items so that we clone only
        // `k` keys instead of all of them.
        let mut items = self.map.iter().map(|(t, n)| (Reverse(n.clone()), t));

        // Step 1. Make a heap out of the first `k` items; this makes O(k) comparisons.
        let mut heap: BinaryHeap<_> = items.by_ref().take(k).collect();

        // Step 2. Successively compare each of the remaining `n - k` items to the top of the heap,
        // replacing the root (and subsequently sifting down) whenever the item is less than the
        // root.  This takes at most n - k + k * (1 + log2(k)) * (H(n) - H(k)) comparisons, where
        // H(i) is the ith [harmonic number](https://en.wikipedia.org/wiki/Harmonic_number).  For
        // fixed `k`, this scales as *n* + *O*(log(*n*)).
        items.for_each(|item| {
            // If `items` is nonempty at this point then we know the heap contains `k > 0`
            // elements.
            let mut root = heap.peek_mut().expect("the heap is empty");
            if *root > item {
                *root = item;
            }
        });

        // Step 3. Sort the items in the heap with the second phases of heapsort.  The number of
        // comparisons is 2 * k * log2(k) + O(k).
        heap.into_sorted_vec()
            .into_iter()
            .map(|(Reverse(n), t)| (t.clone(), n))
            .collect()
    }
}

impl<T, N> Default for Counter<T, N>
where
    T: Hash + Eq,
    N: Default,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            zero: Default::default(),
        }
    }
}

impl<T, N> AddAssign for Counter<T, N>
where
    T: Hash + Eq,
    N: Zero + AddAssign,
{
    /// Add another counter to this counter.
    ///
    /// `c += d;` -> `c[x] += d[x]` for all `x`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// c += d;
    ///
    /// let expect = [('a', 4), ('b', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(c.into_map(), expect);
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.map {
            let entry = self.map.entry(key).or_insert_with(N::zero);
            *entry += value;
        }
    }
}

impl<T, N> Add for Counter<T, N>
where
    T: Clone + Hash + Eq,
    N: AddAssign + Zero,
{
    type Output = Counter<T, N>;

    /// Add two counters together.
    ///
    /// `out = c + d;` -> `out[x] == c[x] + d[x]` for all `x`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// let e = c + d;
    ///
    /// let expect = [('a', 4), ('b', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn add(mut self, rhs: Counter<T, N>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T, N> SubAssign for Counter<T, N>
where
    T: Hash + Eq,
    N: PartialOrd + PartialEq + SubAssign + Zero,
{
    /// Subtract (keeping only positive values).
    ///
    /// `c -= d;` -> `c[x] -= d[x]` for all `x`,
    /// keeping only items with a value greater than [`N::zero()`].
    ///
    /// [`N::zero()`]:
    /// https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html#tymethod.zero
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// c -= d;
    ///
    /// let expect = [('a', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(c.into_map(), expect);
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        for (key, value) in rhs.map {
            let mut remove = false;
            if let Some(entry) = self.map.get_mut(&key) {
                if *entry >= value {
                    *entry -= value;
                } else {
                    remove = true;
                }
                if *entry == N::zero() {
                    remove = true;
                }
            }
            if remove {
                self.map.remove(&key);
            }
        }
    }
}

impl<T, N> Sub for Counter<T, N>
where
    T: Hash + Eq,
    N: PartialOrd + PartialEq + SubAssign + Zero,
{
    type Output = Counter<T, N>;

    /// Subtract (keeping only positive values).
    ///
    /// `out = c - d;` -> `out[x] == c[x] - d[x]` for all `x`,
    /// keeping only items with a value greater than [`N::zero()`].
    ///
    /// [`N::zero()`]:
    /// https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html#tymethod.zero
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// let e = c - d;
    ///
    /// let expect = [('a', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn sub(mut self, rhs: Counter<T, N>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T, N> Counter<T, N>
where
    T: Hash + Eq,
    N: PartialOrd + Zero,
{
    /// Test whether this counter is a superset of another counter.
    /// This is true if for all elements in this counter and the other,
    /// the count in this counter is greater than or equal to the count in the other.
    ///
    /// `c.is_superset(&d);` -> `c.iter().all(|(x, n)| n >= d[x]) && d.iter().all(|(x, n)| c[x] >= n)`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaabbc".chars().collect::<Counter<_>>();
    /// let mut d = "abb".chars().collect::<Counter<_>>();
    ///
    /// assert!(c.is_superset(&d));
    /// d[&'e'] = 1;
    /// assert!(!c.is_superset(&d));
    /// ```
    pub fn is_superset(&self, other: &Self) -> bool {
        // need to test keys from both counters, because if N is signed, counts in `self`
        // could be < 0 for elements missing in `other`. For the unsigned case, only elements
        // from `other` would need to be tested.
        self.keys()
            .chain(other.keys())
            .all(|key| self[key] >= other[key])
    }

    /// Test whether this counter is a subset of another counter.
    /// This is true if for all elements in this counter and the other,
    /// the count in this counter is less than or equal to the count in the other.
    ///
    /// `c.is_subset(&d);` -> `c.iter().all(|(x, n)| n <= d[x]) && d.iter().all(|(x, n)| c[x] <= n)`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "abb".chars().collect::<Counter<_>>();
    /// let mut d = "aaabbc".chars().collect::<Counter<_>>();
    ///
    /// assert!(c.is_subset(&d));
    /// c[&'e'] = 1;
    /// assert!(!c.is_subset(&d));
    /// ```
    pub fn is_subset(&self, other: &Self) -> bool {
        // need to test keys from both counters, because if N is signed, counts in `other`
        // could be < 0 for elements missing in `self`. For the unsigned case, only elements
        // from `self` would need to be tested.
        self.keys()
            .chain(other.keys())
            .all(|key| self[key] <= other[key])
    }
}

impl<T, N> BitAnd for Counter<T, N>
where
    T: Hash + Eq,
    N: Ord + Zero,
{
    type Output = Counter<T, N>;

    /// Returns the intersection of `self` and `rhs` as a new `Counter`.
    ///
    /// `out = c & d;` -> `out[x] == min(c[x], d[x])`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// let e = c & d;
    ///
    /// let expect = [('a', 1), ('b', 1)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn bitand(self, mut rhs: Counter<T, N>) -> Self::Output {
        use std::cmp::min;

        let mut counter = Counter::new();
        for (key, lhs_count) in self.map {
            if let Some(rhs_count) = rhs.remove(&key) {
                let count = min(lhs_count, rhs_count);
                counter.map.insert(key, count);
            }
        }
        counter
    }
}

impl<T, N> BitAndAssign for Counter<T, N>
where
    T: Hash + Eq,
    N: Ord + Zero,
{
    /// Updates `self` with the intersection of `self` and `rhs`
    ///
    /// `c &= d;` -> `c[x] == min(c[x], d[x])`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// c &= d;
    ///
    /// let expect = [('a', 1), ('b', 1)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(c.into_map(), expect);
    /// ```
    fn bitand_assign(&mut self, mut rhs: Counter<T, N>) {
        for (key, rhs_count) in rhs.drain() {
            if rhs_count < self[&key] {
                self.map.insert(key, rhs_count);
            }
        }
    }
}

impl<T, N> BitOr for Counter<T, N>
where
    T: Hash + Eq,
    N: Ord + Zero,
{
    type Output = Counter<T, N>;

    /// Returns the union of `self` and `rhs` as a new `Counter`.
    ///
    /// `out = c | d;` -> `out[x] == max(c[x], d[x])`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// let e = c | d;
    ///
    /// let expect = [('a', 3), ('b', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn bitor(mut self, rhs: Counter<T, N>) -> Self::Output {
        for (key, rhs_value) in rhs.map {
            let entry = self.map.entry(key).or_insert_with(N::zero);
            // We want to update the value of the now occupied entry in `self` with the maximum of
            // its current value and `rhs_value`.  If that max is `rhs_value`, we can just update
            // the value of the entry.  If the max is the current value, we do nothing.  Note that
            // `Ord::max()` returns the second argument (here `rhs_value`) if its two arguments are
            // equal, justifying the use of the weak inequality below instead of a strict
            // inequality.
            //
            // Doing it this way with an inequality instead of actually using `std::cmp::max()`
            // lets us avoid trying (and failing) to move the non-copy value out of the entry in
            // order to pass it as an argument to `std::cmp::max()`, while still holding a mutable
            // reference to the value slot in the entry.
            //
            // And while using the inequality seemingly only requires the bound `N: PartialOrd`, we
            // nevertheless prefer to require `Ord` as though we were using `std::cmp::max()`
            // because the semantics of `BitOr` for `Counter` really do not make sense if there are
            // possibly non-comparable values of type `N`.
            if rhs_value >= *entry {
                *entry = rhs_value;
            }
        }
        self
    }
}

impl<T, N> BitOrAssign for Counter<T, N>
where
    T: Hash + Eq,
    N: Ord + Zero,
{
    /// Updates `self` with the union of `self` and `rhs`
    ///
    /// `c |= d;` -> `c[x] == max(c[x], d[x])`
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "aaab".chars().collect::<Counter<_>>();
    /// let d = "abb".chars().collect::<Counter<_>>();
    ///
    /// c |= d;
    ///
    /// let expect = [('a', 3), ('b', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(c.into_map(), expect);
    /// ```
    fn bitor_assign(&mut self, mut rhs: Counter<T, N>) {
        for (key, rhs_count) in rhs.drain() {
            if rhs_count > self[&key] {
                self.map.insert(key, rhs_count);
            }
        }
    }
}

impl<T, N> Deref for Counter<T, N>
where
    T: Hash + Eq,
{
    type Target = CounterMap<T, N>;
    fn deref(&self) -> &CounterMap<T, N> {
        &self.map
    }
}

impl<T, N> DerefMut for Counter<T, N>
where
    T: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut CounterMap<T, N> {
        &mut self.map
    }
}

impl<'a, T, N> IntoIterator for &'a Counter<T, N>
where
    T: Hash + Eq,
{
    type Item = (&'a T, &'a N);
    type IntoIter = std::collections::hash_map::Iter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<T, N> IntoIterator for Counter<T, N>
where
    T: Hash + Eq,
{
    type Item = (T, N);
    type IntoIter = std::collections::hash_map::IntoIter<T, N>;

    /// Consumes the `Counter` to produce an iterator that owns the values it returns.
    ///
    /// # Examples
    /// ```rust
    /// # use counter::Counter;
    ///
    /// let counter: Counter<_> = "aaab".chars().collect();
    ///
    /// let vec: Vec<_> = counter.into_iter().collect();
    ///
    /// for (item, count) in &vec {
    ///     if item == &'a' {
    ///         assert_eq!(count, &3);
    ///     }
    ///     if item == &'b' {
    ///         assert_eq!(count, &1);
    ///     }
    /// }
    /// ```

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<'a, T, N> IntoIterator for &'a mut Counter<T, N>
where
    T: Hash + Eq,
{
    type Item = (&'a T, &'a mut N);
    type IntoIter = std::collections::hash_map::IterMut<'a, T, N>;

    /// Creates an iterator that provides mutable references to the counts, but keeps the keys immutable.
    ///
    /// # Examples
    /// ```rust
    /// # use counter::Counter;
    ///
    /// let mut counter: Counter<_> = "aaab".chars().collect();
    ///
    /// for (item, count) in &mut counter {
    ///     if *item == 'a' {
    ///         // 'a' is so great it counts as 2
    ///         *count *= 2;
    ///     }
    /// }
    ///
    /// assert_eq!(counter[&'a'], 6);
    /// assert_eq!(counter[&'b'], 1);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.map.iter_mut()
    }
}

impl<T, Q, N> Index<&'_ Q> for Counter<T, N>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
    N: Zero,
{
    type Output = N;

    /// Index in immutable contexts.
    ///
    /// Returns a reference to a [`zero`] value for missing keys.
    ///
    /// [`zero`]:
    /// https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html#tymethod.zero
    ///
    /// ```
    /// # use counter::Counter;
    /// let counter = Counter::<_>::init("aabbcc".chars());
    /// assert_eq!(counter[&'a'], 2);
    /// assert_eq!(counter[&'b'], 2);
    /// assert_eq!(counter[&'c'], 2);
    /// assert_eq!(counter[&'d'], 0);
    /// ```
    ///
    /// Note that the [`zero`] is a struct field but not one of the values of the inner
    /// [`HashMap`].  This method does not modify any existing value.
    ///
    /// [`zero`]:
    /// https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html#tymethod.zero
    /// [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
    ///
    /// ```
    /// # use counter::Counter;
    /// let counter = Counter::<_>::init("".chars());
    /// assert_eq!(counter[&'a'], 0);
    /// assert_eq!(counter.get(&'a'), None); // as `Deref<Target = HashMap<_, _>>`
    /// ```
    fn index(&self, key: &'_ Q) -> &N {
        self.map.get(key).unwrap_or(&self.zero)
    }
}

impl<T, Q, N> IndexMut<&'_ Q> for Counter<T, N>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ToOwned<Owned = T>,
    N: Zero,
{
    /// Index in mutable contexts.
    ///
    /// If the given key is not present, creates a new entry and initializes it with a [`zero`]
    /// value.
    ///
    /// [`zero`]:
    /// https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html#tymethod.zero
    ///
    /// ```
    /// # use counter::Counter;
    /// let mut counter = Counter::<_>::init("aabbcc".chars());
    /// counter[&'c'] += 1;
    /// counter[&'d'] += 1;
    /// assert_eq!(counter[&'c'], 3);
    /// assert_eq!(counter[&'d'], 1);
    /// ```
    ///
    /// Unlike `Index::index`, the returned mutable reference to the [`zero`] is actually one of the
    /// values of the inner [`HashMap`].
    ///
    /// [`zero`]:
    /// https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html#tymethod.zero
    /// [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
    ///
    /// ```
    /// # use counter::Counter;
    /// let mut counter = Counter::<_>::init("".chars());
    /// assert_eq!(counter.get(&'a'), None); // as `Deref<Target = HashMap<_, _>>`
    /// let _ = &mut counter[&'a'];
    /// assert_eq!(counter.get(&'a'), Some(&0));
    /// ```
    fn index_mut(&mut self, key: &'_ Q) -> &mut N {
        self.map.entry(key.to_owned()).or_insert_with(N::zero)
    }
}

impl<I, T, N> AddAssign<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: AddAssign + Zero + One,
{
    /// Directly add the counts of the elements of `I` to `self`.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut counter = Counter::init("abbccc".chars());
    ///
    /// counter += "aeeeee".chars();
    /// let expected: HashMap<char, usize> = [('a', 2), ('b', 2), ('c', 3), ('e', 5)]
    ///     .iter().cloned().collect();
    /// assert_eq!(counter.into_map(), expected);
    /// ```
    fn add_assign(&mut self, rhs: I) {
        self.update(rhs);
    }
}

impl<I, T, N> Add<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: AddAssign + Zero + One,
{
    type Output = Self;
    /// Consume `self` producing a `Counter` like `self` updated with the counts of
    /// the elements of `I`.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let counter = Counter::init("abbccc".chars());
    ///
    /// let new_counter = counter + "aeeeee".chars();
    /// let expected: HashMap<char, usize> = [('a', 2), ('b', 2), ('c', 3), ('e', 5)]
    ///     .iter().cloned().collect();
    /// assert_eq!(new_counter.into_map(), expected);
    /// ```
    fn add(mut self, rhs: I) -> Self::Output {
        self.update(rhs);
        self
    }
}

impl<I, T, N> SubAssign<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: PartialOrd + SubAssign + Zero + One,
{
    /// Directly subtract the counts of the elements of `I` from `self`,
    /// keeping only items with a value greater than [`N::zero()`].
    ///
    /// [`N::zero()`]:
    /// https://docs.rs/num-traits/latest/num_traits/identities/trait.Zero.html#tymethod.zero
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut c = "aaab".chars().collect::<Counter<_>>();
    /// c -= "abb".chars();
    ///
    /// let expect = [('a', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(c.into_map(), expect);
    /// ```
    fn sub_assign(&mut self, rhs: I) {
        self.subtract(rhs);
    }
}

impl<I, T, N> Sub<I> for Counter<T, N>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
    N: PartialOrd + SubAssign + Zero + One,
{
    type Output = Self;
    /// Consume `self` producing a `Counter` like `self` with the counts of the
    /// elements of `I` subtracted, keeping only positive values.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let c = "aaab".chars().collect::<Counter<_>>();
    /// let e = c - "abb".chars();
    ///
    /// let expect = [('a', 2)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(e.into_map(), expect);
    /// ```
    fn sub(mut self, rhs: I) -> Self::Output {
        self.subtract(rhs);
        self
    }
}

impl<T, N> iter::FromIterator<T> for Counter<T, N>
where
    T: Hash + Eq,
    N: AddAssign + Zero + One,
{
    /// Produce a `Counter` from an iterator of items. This is called automatically
    /// by [`Iterator::collect()`].
    ///
    /// [`Iterator::collect()`]:
    /// https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let counter = "abbccc".chars().collect::<Counter<_>>();
    /// let expect = [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    ///
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Counter::<T, N>::init(iter)
    }
}

impl<T, N> iter::FromIterator<(T, N)> for Counter<T, N>
where
    T: Hash + Eq,
    N: AddAssign + Zero,
{
    /// Creates a counter from `(item, count)` tuples.
    ///
    /// The counts of duplicate items are summed.
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let counter = [('a', 1), ('b', 2), ('c', 3), ('a', 4)].iter()
    ///     .cloned().collect::<Counter<_>>();
    /// let expect = [('a', 5), ('b', 2), ('c', 3)].iter()
    ///     .cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    fn from_iter<I: IntoIterator<Item = (T, N)>>(iter: I) -> Self {
        let mut cnt = Counter::new();
        for (item, item_count) in iter {
            let entry = cnt.map.entry(item).or_insert_with(N::zero);
            *entry += item_count;
        }
        cnt
    }
}

impl<T, N> Extend<T> for Counter<T, N>
where
    T: Hash + Eq,
    N: AddAssign + Zero + One,
{
    /// Extend a `Counter` with an iterator of items.
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut counter = "abbccc".chars().collect::<Counter<_>>();
    /// counter.extend("bccddd".chars());
    /// let expect = [('a', 1), ('b', 3), ('c', 5), ('d', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.update(iter);
    }
}

impl<T, N> Extend<(T, N)> for Counter<T, N>
where
    T: Hash + Eq,
    N: AddAssign + Zero,
{
    /// Extend a counter with `(item, count)` tuples.
    ///
    /// The counts of duplicate items are summed.
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut counter = "abbccc".chars().collect::<Counter<_>>();
    /// counter.extend([('a', 1), ('b', 2), ('c', 3), ('a', 4)].iter().cloned());
    /// let expect = [('a', 6), ('b', 4), ('c', 6)].iter()
    ///     .cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    fn extend<I: IntoIterator<Item = (T, N)>>(&mut self, iter: I) {
        for (item, item_count) in iter {
            let entry = self.map.entry(item).or_insert_with(N::zero);
            *entry += item_count;
        }
    }
}

impl<'a, T: 'a, N: 'a> Extend<(&'a T, &'a N)> for Counter<T, N>
where
    T: Hash + Eq + Clone,
    N: AddAssign + Zero + Clone,
{
    /// Extend a counter with `(item, count)` tuples.
    ///
    /// You can extend a `Counter` with another `Counter`:
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let mut counter = "abbccc".chars().collect::<Counter<_>>();
    /// let another = "bccddd".chars().collect::<Counter<_>>();
    /// counter.extend(&another);
    /// let expect = [('a', 1), ('b', 3), ('c', 5), ('d', 3)].iter()
    ///     .cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    fn extend<I: IntoIterator<Item = (&'a T, &'a N)>>(&mut self, iter: I) {
        for (item, item_count) in iter {
            let entry = self.map.entry(item.clone()).or_insert_with(N::zero);
            *entry += item_count.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use rand::Rng;
    use std::collections::HashMap;

    #[test]
    fn test_creation() {
        let _: Counter<usize> = Counter::new();

        let initializer = &[1];
        let counter = Counter::init(initializer);

        let mut expected = HashMap::new();
        static ONE: usize = 1;
        expected.insert(&ONE, 1);
        assert!(counter.map == expected);
    }

    #[test]
    fn test_update() {
        let mut counter = Counter::init("abbccc".chars());
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);

        counter.update("aeeeee".chars());
        let expected = hashmap! {
            'a' => 2,
            'b' => 2,
            'c' => 3,
            'e' => 5,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_add_update_iterable() {
        let mut counter = Counter::init("abbccc".chars());
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);

        counter += "aeeeee".chars();
        let expected = hashmap! {
            'a' => 2,
            'b' => 2,
            'c' => 3,
            'e' => 5,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_add_update_counter() {
        let mut counter = Counter::init("abbccc".chars());
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);

        let other = Counter::init("aeeeee".chars());
        counter += other;
        let expected = hashmap! {
            'a' => 2,
            'b' => 2,
            'c' => 3,
            'e' => 5,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_subtract() {
        let mut counter = Counter::init("abbccc".chars());
        counter.subtract("bbccddd".chars());
        let expected = hashmap! {
            'a' => 1,
            'c' => 1,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_sub_update_iterable() {
        let mut counter = Counter::init("abbccc".chars());
        counter -= "bbccddd".chars();
        let expected = hashmap! {
            'a' => 1,
            'c' => 1,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_sub_update_counter() {
        let mut counter = Counter::init("abbccc".chars());
        let other = Counter::init("bbccddd".chars());
        counter -= other;
        let expected = hashmap! {
            'a' => 1,
            'c' => 1,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_composite_add_sub() {
        let mut counts = Counter::<_>::init(
            "able babble table babble rabble table able fable scrabble".split_whitespace(),
        );
        // add or subtract an iterable of the same type
        counts += "cain and abel fable table cable".split_whitespace();
        // or add or subtract from another Counter of the same type
        let other_counts = Counter::init("scrabble cabbie fable babble".split_whitespace());
        let _diff = counts - other_counts;
    }

    #[test]
    fn test_most_common() {
        let counter = Counter::init("abbccc".chars());
        let by_common = counter.most_common();
        let expected = vec![('c', 3), ('b', 2), ('a', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_most_common_tiebreaker() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter.most_common_tiebreaker(|&a, &b| a.cmp(&b));
        let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_most_common_tiebreaker_reversed() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter.most_common_tiebreaker(|&a, &b| b.cmp(&a));
        let expected = vec![('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)];
        assert!(by_common == expected);
    }

    // The main purpose of this test is to see that we can call `Counter::most_common_tiebreaker()`
    // with a closure that is `FnMut` but not `Fn`.
    #[test]
    fn test_most_common_tiebreaker_fn_mut() {
        let counter: Counter<_> = Counter::init("abracadabra".chars());
        // Count how many times the tiebreaker closure is called.
        let mut num_ties = 0;
        let sorted = counter.most_common_tiebreaker(|a, b| {
            num_ties += 1;
            a.cmp(b)
        });
        let expected = vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)];
        assert_eq!(sorted, expected);
        // We should have called the tiebreaker twice: once to resolve the tie between `'b'` and
        // `'r'` and once to resolve the tie between `'c'` and `'d'`.
        assert_eq!(num_ties, 2);
    }

    #[test]
    fn test_most_common_ordered() {
        let counter = Counter::init("eaddbbccc".chars());
        let by_common = counter.most_common_ordered();
        let expected = vec![('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)];
        assert!(by_common == expected);
    }

    #[test]
    fn test_k_most_common_ordered() {
        let counter: Counter<_> = "abracadabra".chars().collect();
        let all = counter.most_common_ordered();
        for k in 0..=counter.len() {
            let topk = counter.k_most_common_ordered(k);
            assert_eq!(&topk, &all[..k]);
        }
    }

    /// This test is fundamentally the same as `test_k_most_common_ordered`, but it operates on
    /// a wider variety of data. In particular, it tests both longer, narrower, and wider
    /// distributions of data than the other test does.
    #[test]
    fn test_k_most_common_ordered_heavy() {
        let mut rng = rand::thread_rng();

        for container_size in [5, 10, 25, 100, 256] {
            for max_value_factor in [0.25, 0.5, 1.0, 1.25, 2.0, 10.0, 100.0] {
                let max_value = ((container_size as f64) * max_value_factor) as u32;
                let mut values = vec![0; container_size];
                for value in values.iter_mut() {
                    *value = rng.gen_range(0..=max_value);
                }

                let counter: Counter<_> = values.into_iter().collect();
                let all = counter.most_common_ordered();
                for k in 0..=counter.len() {
                    let topk = counter.k_most_common_ordered(k);
                    assert_eq!(&topk, &all[..k]);
                }
            }
        }
    }

    #[test]
    fn test_total() {
        let counter = Counter::init("".chars());
        let total: usize = counter.total();
        assert_eq!(total, 0);

        let counter = Counter::init("eaddbbccc".chars());
        let total: usize = counter.total();
        assert_eq!(total, 9);
    }

    #[test]
    fn test_add() {
        let d = Counter::<_>::init("abbccc".chars());
        let e = Counter::<_>::init("bccddd".chars());

        let out = d + e;
        let expected = Counter::init("abbbcccccddd".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_sub() {
        let d = Counter::<_>::init("abbccc".chars());
        let e = Counter::<_>::init("bccddd".chars());

        let out = d - e;
        let expected = Counter::init("abc".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_intersection() {
        let d = Counter::<_>::init("abbccc".chars());
        let e = Counter::<_>::init("bccddd".chars());

        let out = d & e;
        let expected = Counter::init("bcc".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_union() {
        let d = Counter::<_>::init("abbccc".chars());
        let e = Counter::<_>::init("bccddd".chars());

        let out = d | e;
        let expected = Counter::init("abbcccddd".chars());
        assert!(out == expected);
    }

    #[test]
    fn test_delete_key_from_backing_map() {
        let mut counter = Counter::<_>::init("aa-bb-cc".chars());
        counter.remove(&'-');
        assert!(counter == Counter::init("aabbcc".chars()));
    }

    #[test]
    fn test_from_iter_simple() {
        let counter = "abbccc".chars().collect::<Counter<_>>();
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_from_iter_tuple() {
        let items = [('a', 1), ('b', 2), ('c', 3)];
        let counter = items.iter().cloned().collect::<Counter<_>>();
        let expected: HashMap<char, usize> = items.iter().cloned().collect();
        assert_eq!(counter.map, expected);
    }

    #[test]
    fn test_from_iter_tuple_with_duplicates() {
        let items = [('a', 1), ('b', 2), ('c', 3)];
        let counter = items
            .iter()
            .cycle()
            .take(items.len() * 2)
            .cloned()
            .collect::<Counter<_>>();
        let expected: HashMap<char, usize> = items.iter().map(|(c, n)| (*c, n * 2)).collect();
        assert_eq!(counter.map, expected);
    }

    #[test]
    fn test_extend_simple() {
        let mut counter = "abbccc".chars().collect::<Counter<_>>();
        counter.extend("bccddd".chars());
        let expected = hashmap! {
            'a' => 1,
            'b' => 3,
            'c' => 5,
            'd' => 3,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_extend_tuple() {
        let mut counter = "bccddd".chars().collect::<Counter<_>>();
        let items = [('a', 1), ('b', 2), ('c', 3)];
        counter.extend(items.iter().cloned());
        let expected = hashmap! {
            'a' => 1,
            'b' => 3,
            'c' => 5,
            'd' => 3,
        };
        assert_eq!(counter.map, expected);
    }

    #[test]
    fn test_extend_tuple_with_duplicates() {
        let mut counter = "ccc".chars().collect::<Counter<_>>();
        let items = [('a', 1), ('b', 2), ('c', 3)];
        counter.extend(items.iter().cycle().take(items.len() * 2 - 1).cloned());
        let expected: HashMap<char, usize> = items.iter().map(|(c, n)| (*c, n * 2)).collect();
        assert_eq!(counter.map, expected);
    }

    #[test]
    fn test_count_minimal_type() {
        #[derive(Debug, Hash, PartialEq, Eq)]
        struct Inty {
            i: usize,
        }

        impl Inty {
            pub fn new(i: usize) -> Inty {
                Inty { i }
            }
        }

        // <https://en.wikipedia.org/wiki/867-5309/Jenny>
        let intys = vec![
            Inty::new(8),
            Inty::new(0),
            Inty::new(0),
            Inty::new(8),
            Inty::new(6),
            Inty::new(7),
            Inty::new(5),
            Inty::new(3),
            Inty::new(0),
            Inty::new(9),
        ];

        let inty_counts = Counter::init(intys);
        // println!("{:?}", inty_counts.map); // test runner blanks this
        // {Inty { i: 8 }: 2, Inty { i: 0 }: 3, Inty { i: 9 }: 1, Inty { i: 3 }: 1,
        //  Inty { i: 7 }: 1, Inty { i: 6 }: 1, Inty { i: 5 }: 1}
        assert!(inty_counts.map.get(&Inty { i: 8 }) == Some(&2));
        assert!(inty_counts.map.get(&Inty { i: 0 }) == Some(&3));
        assert!(inty_counts.map.get(&Inty { i: 6 }) == Some(&1));
    }

    #[test]
    fn test_collect() {
        let counter: Counter<_> = "abbccc".chars().collect();
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_non_usize_count() {
        let counter: Counter<_, i8> = "abbccc".chars().collect();
        let expected = hashmap! {
            'a' => 1,
            'b' => 2,
            'c' => 3,
        };
        assert!(counter.map == expected);
    }

    #[test]
    fn test_superset_non_usize_count() {
        let mut a: Counter<_, i8> = "abbcccc".chars().collect();
        let mut b: Counter<_, i8> = "abb".chars().collect();
        assert!(a.is_superset(&b));
        // Negative values are possible, a is no longer a superset
        a[&'e'] = -1;
        assert!(!a.is_superset(&b));
        // Adjust b to make a a superset again
        b[&'e'] = -2;
        assert!(a.is_superset(&b));
    }

    #[test]
    fn test_subset_non_usize_count() {
        let mut a: Counter<_, i8> = "abb".chars().collect();
        let mut b: Counter<_, i8> = "abbcccc".chars().collect();
        assert!(a.is_subset(&b));
        // Negative values are possible; a is no longer a subset
        b[&'e'] = -1;
        assert!(!a.is_subset(&b));
        // Adjust a to make it a subset again
        a[&'e'] = -2;
        assert!(a.is_subset(&b));
    }
}
