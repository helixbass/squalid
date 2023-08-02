#[macro_export]
macro_rules! return_default_if_none {
    ($expr:expr $(,)?) => {
        match $expr {
            None => return Default::default(),
            Some(value) => value,
        }
    };
}

#[macro_export]
macro_rules! return_default_if_false {
    ($expr:expr $(,)?) => {
        if !$expr {
            return Default::default();
        }
    };
}
