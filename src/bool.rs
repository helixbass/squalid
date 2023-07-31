pub trait BoolExt {
    fn then_and<TMapped>(self, mapper: impl FnOnce() -> Option<TMapped>) -> Option<TMapped>;
    fn try_then_and<TMapped, TError>(
        self,
        mapper: impl FnOnce() -> Result<Option<TMapped>, TError>,
    ) -> Result<Option<TMapped>, TError>;
    fn try_then<TMapped, TError>(
        self,
        mapper: impl FnOnce() -> Result<TMapped, TError>,
    ) -> Result<Option<TMapped>, TError>;
}

impl BoolExt for bool {
    fn then_and<TMapped>(self, mapper: impl FnOnce() -> Option<TMapped>) -> Option<TMapped> {
        self.then(mapper).flatten()
    }

    fn try_then_and<TMapped, TError>(
        self,
        mapper: impl FnOnce() -> Result<Option<TMapped>, TError>,
    ) -> Result<Option<TMapped>, TError> {
        Ok(if self { mapper()? } else { None })
    }

    fn try_then<TMapped, TError>(
        self,
        mapper: impl FnOnce() -> Result<TMapped, TError>,
    ) -> Result<Option<TMapped>, TError> {
        Ok(if self { Some(mapper()?) } else { None })
    }
}
