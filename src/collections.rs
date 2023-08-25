use std::{collections::HashSet, hash};

pub trait Contains<TItem> {
    fn contains_(&self, item: &TItem) -> bool;
}

impl<T> Contains<T> for [T]
where
    T: PartialEq,
{
    fn contains_(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Contains<T> for HashSet<T>
where
    T: PartialEq + Eq + hash::Hash,
{
    fn contains_(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T, const T_LEN: usize> Contains<T> for [T; T_LEN]
where
    T: PartialEq,
{
    fn contains_(&self, item: &T) -> bool {
        self.contains(item)
    }
}
