use std::any::Any;

use crate::core::tree::*;
use crate::core::common::*;

mod context;
pub use self::context::Context;

mod id;
pub use self::id::Id;

mod socket;
pub use self::socket::{Socket, SocketName};

mod layout;
pub use self::layout::{Layout, LayoutObj};

// An 'Element' is something run in the the context of a socket
// This is the starting point for any UI tree
pub trait Element: Any {
    fn run<'window, 'ctx>(
        &self,
        ctx: Context<'window, 'ctx>,
    );
}

impl Element for () {
    fn run<'window, 'ctx>(
        &self,
        _ctx: Context<'window, 'ctx>,
    ) {
        // Do nothing
    }
}
