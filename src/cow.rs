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
