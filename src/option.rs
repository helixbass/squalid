pub trait OptionExt {
    type Unwrapped;

    fn matches(self, predicate: impl FnOnce(Self::Unwrapped) -> bool) -> bool;
    fn try_matches<TError>(
        self,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<bool, TError>,
    ) -> Result<bool, TError>;
    #[allow(clippy::wrong_self_convention)]
    fn is_none_or_matches(self, predicate: impl FnOnce(Self::Unwrapped) -> bool) -> bool;

    fn try_get_or_insert_with<TError>(
        &mut self,
        predicate: impl FnOnce() -> Result<Self::Unwrapped, TError>,
    ) -> Result<&mut Self::Unwrapped, TError>;

    fn try_or_else<TError>(
        self,
        predicate: impl FnOnce() -> Result<Option<Self::Unwrapped>, TError>,
    ) -> Result<Option<Self::Unwrapped>, TError>;

    fn try_unwrap_or_else<TError>(
        self,
        predicate: impl FnOnce() -> Result<Self::Unwrapped, TError>,
    ) -> Result<Self::Unwrapped, TError>;

    fn try_and_then<TMapped, TError>(
        self,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<Option<TMapped>, TError>,
    ) -> Result<Option<TMapped>, TError>;

    fn try_map<TMapped, TError>(
        self,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<TMapped, TError>,
    ) -> Result<Option<TMapped>, TError>;

    fn try_filter<TError>(
        self,
        predicate: impl FnOnce(&Self::Unwrapped) -> Result<bool, TError>,
    ) -> Result<Option<Self::Unwrapped>, TError>;

    fn try_map_or_else<TMapped, TError>(
        self,
        default: impl FnOnce() -> Result<TMapped, TError>,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<TMapped, TError>,
    ) -> Result<TMapped, TError>;

    fn try_map_or<TMapped, TError>(
        self,
        default: TMapped,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<TMapped, TError>,
    ) -> Result<TMapped, TError>;

    fn map_or_default<TMapped: Default>(
        self,
        mapper: impl FnOnce(Self::Unwrapped) -> TMapped,
    ) -> TMapped;

    fn try_map_or_default<TMapped: Default, TError>(
        self,
        mapper: impl FnOnce(Self::Unwrapped) -> Result<TMapped, TError>,
    ) -> Result<TMapped, TError>;
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

    fn try_get_or_insert_with<TError>(
        &mut self,
        predicate: impl FnOnce() -> Result<TValue, TError>,
    ) -> Result<&mut TValue, TError> {
        match self {
            Some(value) => Ok(value),
            None => {
                *self = Some(predicate()?);
                Ok(self.as_mut().unwrap())
            }
        }
    }

    fn try_or_else<TError>(
        self,
        predicate: impl FnOnce() -> Result<Option<Self::Unwrapped>, TError>,
    ) -> Result<Option<Self::Unwrapped>, TError> {
        match self {
            Some(value) => Ok(Some(value)),
            None => predicate(),
        }
    }

    fn try_unwrap_or_else<TError>(
        self,
        predicate: impl FnOnce() -> Result<Self::Unwrapped, TError>,
    ) -> Result<Self::Unwrapped, TError> {
        match self {
            Some(value) => Ok(value),
            None => predicate(),
        }
    }

    fn try_and_then<TMapped, TError>(
        self,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<Option<TMapped>, TError>,
    ) -> Result<Option<TMapped>, TError> {
        match self {
            Some(value) => predicate(value),
            None => Ok(None),
        }
    }

    fn try_map<TMapped, TError>(
        self,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<TMapped, TError>,
    ) -> Result<Option<TMapped>, TError> {
        Ok(match self {
            Some(value) => Some(predicate(value)?),
            None => None,
        })
    }

    fn try_filter<TError>(
        self,
        predicate: impl FnOnce(&TValue) -> Result<bool, TError>,
    ) -> Result<Option<TValue>, TError> {
        Ok(match self {
            None => None,
            Some(value) => {
                if predicate(&value)? {
                    Some(value)
                } else {
                    None
                }
            }
        })
    }

    fn try_map_or_else<TMapped, TError>(
        self,
        default: impl FnOnce() -> Result<TMapped, TError>,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<TMapped, TError>,
    ) -> Result<TMapped, TError> {
        match self {
            None => default(),
            Some(value) => predicate(value),
        }
    }

    fn try_map_or<TMapped, TError>(
        self,
        default: TMapped,
        predicate: impl FnOnce(Self::Unwrapped) -> Result<TMapped, TError>,
    ) -> Result<TMapped, TError> {
        match self {
            None => Ok(default),
            Some(value) => predicate(value),
        }
    }

    fn map_or_default<TMapped: Default>(
        self,
        mapper: impl FnOnce(Self::Unwrapped) -> TMapped,
    ) -> TMapped {
        self.map(mapper).unwrap_or_default()
    }

    fn try_map_or_default<TMapped: Default, TError>(
        self,
        mapper: impl FnOnce(Self::Unwrapped) -> Result<TMapped, TError>,
    ) -> Result<TMapped, TError> {
        Ok(self.try_map(mapper)?.unwrap_or_default())
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
