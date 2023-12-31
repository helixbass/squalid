mod bool;
mod collections;
mod cow;
mod everything;
mod hash_map;
mod macros;
mod option;
mod vec;

pub use collections::Contains;
pub use cow::{CowExt, CowStrExt};
pub use everything::EverythingExt;
pub use hash_map::HashMapExt;
pub use option::{IsEmpty, NonEmpty, OptionExt, OptionExtDeref, OptionExtIterator};
pub use vec::{SliceExtClone, SliceExtCloneOrd, VecExt, VecExtOrd};

pub use crate::bool::BoolExt;
