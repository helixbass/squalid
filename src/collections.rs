use std::{collections::HashSet, hash};

pub trait Contains {
    type Item;

    fn contains_(&self, item: &Self::Item) -> bool;
}

impl<T> Contains for [T]
where
    T: PartialEq,
{
    type Item = T;

    fn contains_(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Contains for HashSet<T>
where
    T: PartialEq + Eq + hash::Hash,
{
    type Item = T;

    fn contains_(&self, item: &T) -> bool {
        self.contains(item)
    }
}
