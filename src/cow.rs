use std::{
    borrow::{Borrow, Cow},
    slice::SliceIndex,
};

pub trait CowExt<'a> {
    type Borrowed: 'a + ToOwned + ?Sized;

    fn map_borrowed(
        self,
        mapper: impl FnOnce(&Self::Borrowed) -> &Self::Borrowed,
    ) -> Cow<'a, Self::Borrowed>;
    fn map_borrowed_ref(
        &self,
        mapper: impl FnOnce(&Self::Borrowed) -> &Self::Borrowed,
    ) -> Cow<'a, Self::Borrowed>;
    fn map_cow(
        self,
        mapper: impl FnOnce(&Self::Borrowed) -> Cow<'_, Self::Borrowed>,
    ) -> Cow<'a, Self::Borrowed>;
    fn map_cow_ref(
        &self,
        mapper: impl FnOnce(&Self::Borrowed) -> Cow<'_, Self::Borrowed>,
    ) -> Cow<'a, Self::Borrowed>;
}

impl<'a, TBorrowed> CowExt<'a> for Cow<'a, TBorrowed>
where
    TBorrowed: 'a + ToOwned + ?Sized + PartialEq,
{
    type Borrowed = TBorrowed;

    fn map_borrowed(self, mapper: impl FnOnce(&TBorrowed) -> &TBorrowed) -> Cow<'a, TBorrowed> {
        match self {
            Cow::Borrowed(value) => Cow::Borrowed(mapper(value)),
            Cow::Owned(value) => {
                let value_ref = value.borrow();
                let mapped = mapper(value_ref);
                Cow::Owned(if value_ref == mapped {
                    value
                } else {
                    mapped.to_owned()
                })
            }
        }
    }

    fn map_borrowed_ref(
        &self,
        mapper: impl FnOnce(&TBorrowed) -> &TBorrowed,
    ) -> Cow<'a, TBorrowed> {
        match self {
            Cow::Borrowed(value) => Cow::Borrowed(mapper(value)),
            Cow::Owned(value) => Cow::Owned(mapper(value.borrow()).to_owned()),
        }
    }

    fn map_cow(self, mapper: impl FnOnce(&TBorrowed) -> Cow<'_, TBorrowed>) -> Cow<'a, TBorrowed> {
        match self {
            Cow::Borrowed(value) => mapper(value),
            Cow::Owned(value) => {
                let value_ref = value.borrow();
                Cow::Owned(match mapper(value_ref) {
                    Cow::Owned(mapped) => mapped,
                    Cow::Borrowed(mapped) => {
                        if value_ref == mapped {
                            value
                        } else {
                            mapped.to_owned()
                        }
                    }
                })
            }
        }
    }

    fn map_cow_ref(
        &self,
        mapper: impl FnOnce(&TBorrowed) -> Cow<'_, TBorrowed>,
    ) -> Cow<'a, TBorrowed> {
        match self {
            Cow::Borrowed(value) => mapper(value),
            Cow::Owned(value) => Cow::Owned(mapper(value.borrow()).into_owned()),
        }
    }
}

pub trait CowStrExt<'a> {
    fn sliced<TRange: SliceIndex<str, Output = str>>(
        &self,
        get_range: impl FnOnce(usize) -> TRange,
    ) -> Cow<'a, str>;
    fn sliced_owned<TRange: SliceIndex<str, Output = str>>(
        self,
        get_range: impl FnOnce(usize) -> TRange,
    ) -> Cow<'a, str>;
    fn trimmed(self) -> Cow<'a, str>;
    fn trimmed_ref(&self) -> Cow<'a, str>;
}

impl<'a> CowStrExt<'a> for Cow<'a, str> {
    fn sliced<TRange: SliceIndex<str, Output = str>>(
        &self,
        get_range: impl FnOnce(usize) -> TRange,
    ) -> Cow<'a, str> {
        let range = get_range(self.len());
        self.map_borrowed_ref(|value| &value[range])
    }

    fn sliced_owned<TRange: SliceIndex<str, Output = str>>(
        self,
        get_range: impl FnOnce(usize) -> TRange,
    ) -> Cow<'a, str> {
        let range = get_range(self.len());
        self.map_borrowed(|value| &value[range])
    }

    fn trimmed(self) -> Cow<'a, str> {
        self.map_borrowed(str::trim)
    }

    fn trimmed_ref(&self) -> Cow<'a, str> {
        self.map_borrowed_ref(str::trim)
    }
}

pub trait IntoCow<'a, T>
where
    T: ToOwned + ?Sized,
{
    fn into_cow(self) -> Cow<'a, T>;
}

impl<'a, T: ToOwned<Owned = T>> IntoCow<'a, T> for T {
    fn into_cow(self) -> Cow<'a, T> {
        Cow::Owned(self)
    }
}

impl<'a, T: ToOwned + ?Sized> IntoCow<'a, T> for &'a T {
    fn into_cow(self) -> Cow<'a, T> {
        Cow::Borrowed(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_into_cow_owned() {
        assert_eq!(
            HashMap::<String, ()>::default().into_cow(),
            Cow::Owned(HashMap::<String, ()>::default())
        );
    }

    #[test]
    fn test_into_cow_reference() {
        let hash_map = HashMap::<String, ()>::default();
        assert_eq!(
            IntoCow::<HashMap<_, _>>::into_cow(&hash_map),
            Cow::Borrowed(&HashMap::<String, ()>::default())
        );
    }

    #[test]
    fn test_into_cow_str() {
        let str = "foo";
        assert_eq!(IntoCow::<str>::into_cow(str), Cow::Borrowed("foo"));
    }

    #[test]
    fn test_into_cow_slice() {
        let vec = vec!["foo", "bar"];
        let slice = &*vec;
        assert_eq!(
            IntoCow::<[_]>::into_cow(slice),
            Cow::Borrowed(&*vec!["foo", "bar"])
        );
    }
}
