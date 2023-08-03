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

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
