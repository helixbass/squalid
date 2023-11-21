use std::{borrow::Cow, slice::SliceIndex};

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
