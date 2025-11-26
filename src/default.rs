pub fn _d<TValue: Default>() -> TValue {
    TValue::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d() {
        let foo: String = _d();
        assert_eq!(foo, "");
    }
}
