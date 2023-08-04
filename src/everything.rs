pub trait EverythingExt: Sized {
    fn thrush<TReturn>(self, callback: impl FnOnce(Self) -> TReturn) -> TReturn;
    fn when(self, callback: impl FnOnce(&Self) -> bool) -> Option<Self>;
}

impl<T: Sized> EverythingExt for T {
    fn thrush<TReturn>(self, callback: impl FnOnce(Self) -> TReturn) -> TReturn {
        callback(self)
    }

    fn when(self, callback: impl FnOnce(&Self) -> bool) -> Option<Self> {
        callback(&self).then_some(self)
    }
}
