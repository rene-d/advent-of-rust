use std::borrow::Borrow;
use std::iter;
use std::ops::{Index, IndexMut};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Counter<T: Hash + Eq> {
    map: HashMap<T, usize>,
    zero: usize,
}

impl<T> Counter<T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            zero: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            zero: 0,
        }
    }

    pub fn insert(&mut self, elem: T) {
        *self.map.entry(elem).or_insert(0) += 1;
    }

    pub fn update<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iterable {
            let entry = self.map.entry(item).or_insert(0);
            *entry += 1;
        }
    }

    pub fn total(&self) -> usize {
        self.map.values().sum()
    }
}

impl<T> Default for Counter<T>
where
    T: Hash + Eq,
{
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            zero: 0,
        }
    }
}

impl<T> iter::FromIterator<T> for Counter<T>
where
    T: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self {
        let mut counter = Self::new();
        counter.update(iterable);
        counter
    }
}

impl<T> iter::FromIterator<(T, usize)> for Counter<T>
where
    T: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = (T, usize)>>(iter: I) -> Self {
        let mut cnt = Self::new();
        for (item, item_count) in iter {
            let entry = cnt.map.entry(item).or_insert(0);
            *entry += item_count;
        }
        cnt
    }
}

impl<T, Q> Index<&'_ Q> for Counter<T>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
{
    type Output = usize;

    fn index(&self, key: &'_ Q) -> &usize {
        self.map.get(key).unwrap_or(&self.zero)
    }
}

impl<T, Q> IndexMut<&'_ Q> for Counter<T>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ToOwned<Owned = T>,
{
    fn index_mut(&mut self, key: &'_ Q) -> &mut usize {
        self.map.entry(key.to_owned()).or_insert(0)
    }
}

impl<T> Extend<T> for Counter<T>
where
    T: Hash + Eq,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.update(iter);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count() {
        let mut counter = "aaa".chars().collect::<Counter<_>>();
        counter[&'a'] += 1;
        counter.insert('b');

        assert_eq!(counter[&'a'], 4);
        assert_eq!(counter[&'b'], 1);
        assert_eq!(counter.total(), 5);
    }

    #[test]
    fn count2() {
        let mut counter = Counter::<u32>::new();

        counter[&10] += 2;
        counter.insert(20);

        assert_eq!(counter[&10], 2);
        assert_eq!(counter[&20], 1);
        assert_eq!(counter.total(), 3);
    }
}
