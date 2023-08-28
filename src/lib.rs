mod bool;
mod collections;
mod cow;
mod everything;
mod macros;
mod option;
mod vec;

pub use crate::bool::BoolExt;
pub use collections::Contains;
pub use cow::CowStrExt;
pub use everything::EverythingExt;
pub use option::{IsEmpty, NonEmpty, OptionExt, OptionExtDeref, OptionExtIterator};
pub use vec::{SliceExtCloneOrd, VecExt, VecExtOrd};
