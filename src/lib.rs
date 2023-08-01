mod bool;
mod cow;
mod option;
mod vec;

pub use crate::bool::BoolExt;
pub use cow::CowStrExt;
pub use option::{IsEmpty, NonEmpty, OptionExt};
pub use vec::{SliceExtCloneOrd, VecExt, VecExtOrd};
