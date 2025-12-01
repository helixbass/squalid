use std::{fmt, vec};

pub trait IteratorExt {
    type Item;

    fn log(self, message_key: &str) -> vec::IntoIter<Self::Item>;
}

impl<TItem: fmt::Debug, TIterator: Iterator<Item = TItem>> IteratorExt for TIterator {
    type Item = TItem;

    fn log(self, message_key: &str) -> vec::IntoIter<TItem> {
        let collected: Vec<_> = self.collect();
        println!("{message_key}: {collected:?}");
        collected.into_iter()
    }
}

// #[cfg(test)]
// mod tests {
//     use std::io::Read;

//     use gag::BufferRedirect;

//     use super::*;

//     #[test]
//     fn test_log() {
//         let mut captured = BufferRedirect::stdout().unwrap();

//         assert_eq!(
//             vec!["foo", "bar"].into_iter().log("whee").nth(1).unwrap(),
//             "bar"
//         );

//         let mut output = String::default();
//         captured.read_to_string(&mut output).unwrap();
//         assert_eq!(output, r#"whee: ["foo", "bar"]"#);
//     }
// }
