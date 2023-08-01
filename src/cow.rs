use std::borrow::Cow;

pub trait CowStrExt<'a> {
    fn sliced(&self, get_slice: impl FnOnce(&str) -> &str) -> Cow<'a, str>;
}

impl<'a> CowStrExt<'a> for Cow<'a, str> {
    fn sliced(&self, get_slice: impl FnOnce(&str) -> &str) -> Cow<'a, str> {
        match self {
            Self::Borrowed(value) => get_slice(value).into(),
            Self::Owned(value) => get_slice(value).to_owned().into(),
        }
    }
}
