pub trait OptionExt {
    type Unwrapped;

    fn matches(self, predicate: impl FnOnce(Self::Unwrapped) -> bool) -> bool;
    fn try_matches<TError>(
        self,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<bool, TError>,
    ) -> Result<bool, TError>;
    fn is_none_or_matches(self, predicate: impl FnOnce(Self::Unwrapped) -> bool) -> bool;
}

impl<TValue> OptionExt for Option<TValue> {
    type Unwrapped = TValue;

    fn matches(self, predicate: impl FnOnce(Self::Unwrapped) -> bool) -> bool {
        self.map(predicate) == Some(true)
    }

    fn try_matches<TError>(
        self,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<bool, TError>,
    ) -> Result<bool, TError> {
        match self {
            None => Ok(false),
            Some(value) => Ok(predicate(value)?),
        }
    }

    fn is_none_or_matches(self, predicate: impl FnOnce(Self::Unwrapped) -> bool) -> bool {
        self.map_or(true, predicate)
    }
}
