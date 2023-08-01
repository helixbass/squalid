pub trait OptionExt {
    type Unwrapped;

    fn matches(self, predicate: impl FnOnce(Self::Unwrapped) -> bool) -> bool;
    fn try_matches<TError>(
        self,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<bool, TError>,
    ) -> Result<bool, TError>;
    #[allow(clippy::wrong_self_convention)]
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

pub trait IsEmpty {
    fn _is_empty(&self) -> bool;
}

macro_rules! impl_is_empty {
    ($type:ty $(,)?) => {
        impl IsEmpty for $type {
            fn _is_empty(&self) -> bool {
                self.is_empty()
            }
        }
    };
    ($type:ty, $generic:ident $(,)?) => {
        impl<$generic> IsEmpty for $type {
            fn _is_empty(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

impl_is_empty!(&str);
impl_is_empty!(&String);
impl_is_empty!(String);
impl_is_empty!(&[TItem], TItem);
impl_is_empty!(&Vec<TItem>, TItem);
impl_is_empty!(Vec<TItem>, TItem);

pub trait NonEmpty {
    type Optional;

    fn non_empty(self) -> Self::Optional;
    fn is_non_empty(&self) -> bool;
}

impl<T> NonEmpty for T
where
    T: IsEmpty,
{
    type Optional = Option<Self>;

    fn non_empty(self) -> Self::Optional {
        (!self._is_empty()).then_some(self)
    }

    fn is_non_empty(&self) -> bool {
        !self._is_empty()
    }
}

impl<T> NonEmpty for Option<T>
where
    T: IsEmpty,
{
    type Optional = Self;

    fn non_empty(self) -> Self::Optional {
        self.filter(|value| !value._is_empty())
    }

    fn is_non_empty(&self) -> bool {
        self.as_ref().filter(|value| !value._is_empty()).is_some()
    }
}
