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

#[macro_export]
macro_rules! fancy_regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<fancy_regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| fancy_regex::Regex::new($re).unwrap())
    }};
}

#[macro_export]
macro_rules! return_if_none {
    ($expr:expr $(,)?) => {
        match $expr {
            None => return,
            Some(value) => value,
        }
    };
}

#[macro_export]
macro_rules! continue_if_none {
    ($expr:expr $(,)?) => {
        match $expr {
            None => continue,
            Some(value) => value,
        }
    };
}

#[macro_export]
macro_rules! break_if_none {
    ($expr:expr $(,)?) => {
        match $expr {
            None => break,
            Some(value) => value,
        }
    };
}

#[macro_export]
macro_rules! run_once {
    ($($stmt:stmt)*) => {{
        static ONCE_LOCK: std::sync::OnceLock<()> = std::sync::OnceLock::new();
        #[allow(clippy::unused_unit, redundant_semicolons)]
        ONCE_LOCK.get_or_init(|| {
            {
                $($stmt)*
                ()
            }
        });
    }}
}

#[macro_export]
macro_rules! json_object {
    ($($json:tt)+) => {
        match serde_json::json!($($json)*) {
            serde_json::Value::Object(value) => value,
            _ => panic!("Expected object"),
        }
    };
}
