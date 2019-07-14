#![feature(unsize, coerce_unsized)]
#![feature(ptr_internals)]

pub mod state;
pub mod render;
pub mod space;
pub mod util;

mod core;
pub use self::core::{element, filter, Window};

pub mod primitives;

pub mod prelude {
    pub use crate::element::*;
    pub use crate::filter::*;
    pub use crate::state::*;
    pub use crate::space::*;

    pub use crate::Window;
}
