#[macro_use]
pub mod util;
pub mod render;
pub mod space;

mod core;
pub use self::core::{id, message, element, filter, Window};

pub mod primitives;

pub mod prelude {
    pub use crate::id::Id;
    pub use crate::element::*;
    pub use crate::filter::{FilterStack, Filter, TypedFilter, self};
    pub use crate::message::*;
    pub use crate::space::*;
    pub use crate::render;

    pub use crate::Window;
}
