use std::cmp::Ordering;

pub trait VecExt {
    type Item;

    fn and_push(self, item: Self::Item) -> Self;
    fn and_extend(self, iter: impl IntoIterator<Item = Self::Item>) -> Self;
    fn and_sort_by(self, compare: impl FnMut(&Self::Item, &Self::Item) -> Ordering) -> Self;
    fn append_to(self, other: &mut Vec<Self::Item>);
}

impl<TItem> VecExt for Vec<TItem> {
    type Item = TItem;

    fn and_push(mut self, item: TItem) -> Self {
        self.push(item);
        self
    }

    fn and_extend(mut self, iter: impl IntoIterator<Item = TItem>) -> Self {
        self.extend(iter);
        self
    }

    fn and_sort_by(mut self, compare: impl FnMut(&Self::Item, &Self::Item) -> Ordering) -> Self {
        self.sort_by(compare);
        self
    }

    fn append_to(mut self, other: &mut Vec<Self::Item>) {
        other.append(&mut self);
    }
}

pub trait VecExtOrd {
    type Item: Ord;

    fn and_sort(self) -> Self;
}

impl<TItem: Ord> VecExtOrd for Vec<TItem> {
    type Item = TItem;

    fn and_sort(mut self) -> Self {
        self.sort();
        self
    }
}

pub trait SliceExtClone {
    type Item: Clone;

    fn sorted_by_key<TKey>(self, f: impl FnMut(&Self::Item) -> TKey) -> Vec<Self::Item>
    where
        TKey: Ord;
}

impl<TItem: Clone + Ord> SliceExtClone for &[TItem] {
    type Item = TItem;

    fn sorted_by_key<TKey>(self, f: impl FnMut(&Self::Item) -> TKey) -> Vec<Self::Item>
    where
        TKey: Ord,
    {
        let mut cloned = self.to_owned();
        cloned.sort_by_key(f);
        cloned
    }
}

pub trait SliceExtCloneOrd {
    type Item: Clone + Ord;

    fn sorted(self) -> Vec<Self::Item>;
}

impl<TItem: Clone + Ord> SliceExtCloneOrd for &[TItem] {
    type Item = TItem;

    fn sorted(self) -> Vec<TItem> {
        let mut cloned = self.to_owned();
        cloned.sort();
        cloned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_push() {
        assert_eq!(vec![1].and_push(2), vec![1, 2]);
    }

    #[test]
    fn test_and_extend() {
        assert_eq!(vec![1].and_extend([2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_and_sort() {
        assert_eq!(vec![1, 3, 2].and_sort(), vec![1, 2, 3]);
    }

    #[test]
    fn test_sorted() {
        let foo = vec![1, 3, 2];
        let sorted = foo.sorted();
        assert_eq!(sorted, vec![1, 2, 3]);
        assert_eq!(foo, vec![1, 3, 2]);
    }

    #[test]
    fn test_append_to() {
        let mut ret = vec![3, 4];
        vec![1, 2].append_to(&mut ret);
        assert_eq!(ret, vec![3, 4, 1, 2]);
    }
}
