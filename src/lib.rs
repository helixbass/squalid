mod bool;
mod collections;
mod cow;
mod default;
mod everything;
mod hash_map;
mod iterator;
mod macros;
mod option;
mod vec;

pub use collections::Contains;
pub use cow::{CowExt, CowStrExt};
pub use default::_d;
pub use everything::EverythingExt;
pub use hash_map::HashMapExt;
pub use iterator::IteratorExt;
pub use option::{IsEmpty, NonEmpty, OptionExt, OptionExtDeref, OptionExtIterator, OptionExtVec};
pub use vec::{SliceExtClone, SliceExtCloneOrd, VecExt, VecExtOrd};

pub use crate::bool::BoolExt;
