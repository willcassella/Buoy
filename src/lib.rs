#[macro_use]
pub mod util;
pub mod render;
pub mod space;

mod core;
pub use self::core::{component, context::*, id, message};

pub mod basic_renderer;

pub mod primitives;

pub mod prelude {
    pub use crate::component::*;
    pub use crate::id::Id;
    pub use crate::message::*;
    pub use crate::render;
    pub use crate::space::*;

    pub use crate::{Context, FrameContext, ThreadContext, Window};
}
