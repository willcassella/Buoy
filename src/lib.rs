#[macro_use]
pub mod util;
pub mod render;
pub mod space;

mod core;
pub use self::core::{element, filter, id, message, Window};

pub mod primitives;

pub mod prelude {
    pub use crate::element::*;
    pub use crate::filter::{self, Filter, FilterStack, TypedFilter};
    pub use crate::id::Id;
    pub use crate::message::*;
    pub use crate::render;
    pub use crate::space::*;

    pub use crate::Window;
}
