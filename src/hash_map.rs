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

pub trait IteratorExt: Iterator {
    type Key;
    type Value;

    fn map_values<TNewValue, TMapper>(
        &self,
        mapper: TMapper,
    ) -> MapValues<Self, TMapper, Self::Key, Self::Value>
    where
        TMapper: FnMut(Self::Value) -> TNewValue;
}

impl<TKey, TValue, TIterator> IteratorExt for TIterator
where
    TIterator: Iterator<Item = (&TKey, &TValue)>,
{
    type Key = TKey;
    type Value = TKey;

    fn map_values<TNewValue>(
        &self,
        mapper: impl Fn(&Self::Value) -> TNewValue,
    ) -> MapValues<Self::Key, Self::Value>;
}

pub struct MapValues<TInner, TKey, TValue>
where
    TInner: Iterator<Item = (&TKey, &TValue)>,
{
    inner: TInner,
}
