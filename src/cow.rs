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
        mapper: impl FnOnce(&Self::Borrowed) -> Cow<'a, Self::Borrowed>,
    ) -> Cow<'a, Self::Borrowed>;
    fn map_cow_ref(
        &self,
        mapper: impl FnOnce(&Self::Borrowed) -> Cow<'a, Self::Borrowed>,
    ) -> Cow<'a, Self::Borrowed>;
}

impl<'a, TBorrowed> CowExt<'a> for Cow<'a, TBorrowed>
where
    TBorrowed: 'a + ToOwned + ?Sized + PartialEq,
{
    type Borrowed = TBorrowed;

    fn map_borrowed(
        self,
        mapper: impl FnOnce(&TBorrowed) -> &TBorrowed,
    ) -> Cow<'a, TBorrowed> {
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

    fn map_cow(
        self,
        mapper: impl FnOnce(&TBorrowed) -> Cow<'a, TBorrowed>,
    ) -> Cow<'a, TBorrowed> {
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
        mapper: impl FnOnce(&TBorrowed) -> Cow<'a, TBorrowed>,
    ) -> Cow<'a, TBorrowed> {
        match self {
            Cow::Borrowed(value) => mapper(value),
            Cow::Owned(value) => Cow::Owned(mapper(value.borrow()).into_owned()),
        }
    }
}

pub trait CowStrExt<'a> {
    fn sliced(&self, range: impl SliceIndex<str, Output = str>) -> Cow<'a, str>;
    fn trimmed(&self) -> Cow<'a, str>;
}

impl<'a> CowStrExt<'a> for Cow<'a, str> {
    fn sliced(&self, range: impl SliceIndex<str, Output = str>) -> Cow<'a, str> {
        match self {
            Self::Borrowed(value) => value[range].into(),
            Self::Owned(value) => (&**value)[range].to_owned().into(),
        }
    }

    fn trimmed(&self) -> Cow<'a, str> {
        match self {
            Self::Borrowed(value) => value.trim().into(),
            Self::Owned(value) => value.trim().to_owned().into(),
        }
    }
}
