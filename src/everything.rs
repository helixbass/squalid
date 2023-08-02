pub trait EverythingExt: Sized {
    fn thrush<TReturn>(self, callback: impl FnOnce(Self) -> TReturn) -> TReturn;
}

impl<T: Sized> EverythingExt for T {
    fn thrush<TReturn>(self, callback: impl FnOnce(Self) -> TReturn) -> TReturn {
        callback(self)
    }
}
