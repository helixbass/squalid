use std::{collections::HashMap, hash};

pub trait HashMapExt {
    type Key;
    type Value;

    fn and_extend(self, iter: impl IntoIterator<Item = (Self::Key, Self::Value)>) -> Self;
}

impl<TKey, TValue> HashMapExt for HashMap<TKey, TValue>
where
    TKey: Eq + hash::Hash,
{
    type Key = TKey;
    type Value = TValue;

    fn and_extend(mut self, iter: impl IntoIterator<Item = (TKey, TValue)>) -> Self {
        self.extend(iter);
        self
    }
}
