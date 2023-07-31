pub trait VecExt {
    type Item;

    fn and_push(self, item: Self::Item) -> Self;
    fn and_extend(self, iter: impl IntoIterator<Item = Self::Item>) -> Self;
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
}
