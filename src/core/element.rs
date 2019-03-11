use std::any::Any;

mod context;
pub use self::context::Context;

mod id;
pub use self::id::Id;

mod dyn_element;
pub use self::dyn_element::DynElement;

mod socket;
pub use self::socket::{Socket, SocketName};

mod layout;
pub use self::layout::{Layout, LayoutObj, NullLayout};

pub trait Element: Sized + Clone + Any {
    type Suspended: Element;

    fn run(
        self,
        ctx: &mut Context,
        socket: &mut dyn Socket,
    ) -> Option<Self::Suspended>;

    fn upcast(
        self,
    ) -> Box<dyn DynElement> {
        Box::new(self)
    }

    fn downcast<D: Element>(
        self,
    ) -> Result<D, Self> {
        Err(self) // TODO: This should handle when Self == D
    }
}

impl Element for () {
    type Suspended = ();

    fn run(
        self,
        _ctx: &mut Context,
        _socket: &mut dyn Socket,
    ) -> Option<Self::Suspended> {
        // Do nothing
        None
    }
}
