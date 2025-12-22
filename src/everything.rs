pub trait EverythingExt: Sized {
    fn thrush<TReturn>(self, callback: impl FnOnce(Self) -> TReturn) -> TReturn;
    fn when(self, callback: impl FnOnce(&Self) -> bool) -> Option<Self>;
    fn when_ref(&self, callback: impl FnOnce(&Self) -> bool) -> Option<&Self>;
    fn tap(self, callback: impl FnOnce(&Self)) -> Self;
}

impl<T: Sized> EverythingExt for T {
    fn thrush<TReturn>(self, callback: impl FnOnce(Self) -> TReturn) -> TReturn {
        callback(self)
    }

    fn when(self, callback: impl FnOnce(&Self) -> bool) -> Option<Self> {
        callback(&self).then_some(self)
    }

    fn when_ref(&self, callback: impl FnOnce(&Self) -> bool) -> Option<&Self> {
        callback(self).then_some(self)
    }

    fn tap(self, callback: impl FnOnce(&Self)) -> Self {
        callback(&self);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_ref() {
        let foo = "foo".to_owned();
        assert_eq!(foo.when_ref(|value| value == "bar"), None);
        assert_eq!(
            foo.when_ref(|value| value == "foo")
                .map(|value| format!("{value}bar")),
            Some("foobar".to_owned())
        );
    }

    #[test]
    fn test_tap() {
        let mut foo = "".to_owned();
        let bar = "bar".to_owned().tap(|bar| {
            foo.push_str(bar);
            foo.push_str(bar);
        });
        assert_eq!(bar, "bar".to_owned());
        assert_eq!(foo, "barbar".to_owned());
    }
}
