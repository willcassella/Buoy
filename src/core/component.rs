use crate::core::context::{Context, SubContext};
use crate::core::id::Id;
use crate::space::Area;
use crate::util::arena::{ABox, Arena};
use crate::util::ref_move::{Ext, RefMove};
use crate::util::upcast::Upcast;
use arrayvec::ArrayString;
use std::fmt;

mod socket;
pub use self::socket::{Socket, SocketCapacity, SocketName};

mod layout;
pub use self::layout::{Layout, LayoutNode};

// TODO: Index should be "unforgeable" (as in, not cloneable)
pub type ComponentIndex = usize;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TypeId {
    pub package: ArrayString<[u8; 4]>,
    pub name: ArrayString<[u8; 12]>,
}

impl TypeId {
    pub fn new(package: &str, name: &str) -> Self {
        // TODO: Have better error handling here
        TypeId {
            package: ArrayString::from(package).unwrap(),
            name: ArrayString::from(name).unwrap(),
        }
    }
}

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.package, self.name)
    }
}

impl fmt::Debug for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

// This trait is unsafe because a careless implementation
// of 'get_type_id()' (ie, returning a different TypeId than what this was registered with)
// will cause an improper downcast.
pub unsafe trait DynComponent {
    fn get_type_id(&self) -> TypeId;
}

auto_impl_upcast!(dyn DynComponent);

pub trait Component: DynComponent {
    fn type_id() -> TypeId;
}

unsafe impl<T: Component> DynComponent for T {
    fn get_type_id(&self) -> TypeId {
        T::type_id()
    }
}

pub trait RendererFactory {
    // TODO: fn supports(&self, type_id: TypeId) -> bool;

    fn create_renderer<'frm, 'thrd>(
        &self,
        type_id: TypeId,
        buf: &'thrd Arena,
    ) -> ABox<'thrd, dyn Renderer<'frm>>;
}

pub trait Renderer<'frm>: 'frm {
    fn type_id(&self) -> TypeId;

    fn alloc(&self, comp: RefMove<dyn DynComponent + 'frm>) -> ComponentIndex;

    // Begins the layout process for a component previously allocated in this renderer with 'alloc'.
    // The renderer may open sockets or subcontexts, and calling ctx.defer() will block until all
    // dependencies are met, then this function will be run again.
    // TODO: Implement ctx.defer()
    // TODO: Maybe allow this to specify which things its deferring on/how?
    fn layout<'thrd, 'ctx>(
        &self,
        ctx: Context<'frm, 'thrd, 'ctx>,
        comp_index: ComponentIndex,
    ) -> LayoutNode<'frm>;

    //TODO: fn render<'ctx>(LayoutNode);
}

impl<'frm, T: Renderer<'frm>> Upcast<dyn Renderer<'frm>> for T {
    fn upcast(&self) -> &dyn Renderer<'frm> {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn Renderer<'frm> {
        self
    }
}

// TODO: This should be externalized (and made nicer)
pub trait Builder<'frm>: Sized {
    type Component: Component + 'frm;

    fn get_id(&self) -> Id;

    fn get_socket(&self) -> SocketName;

    fn get_component(self) -> Self::Component;

    fn push(self, sub_ctx: &mut SubContext<'frm, '_, '_>) {
        sub_ctx.push_into(
            self.get_socket(),
            self.get_id(),
            self.get_component().anchor(),
        );
    }

    fn open<'thrd, 'ctx, 'a>(
        self,
        ctx: &'a mut Context<'frm, 'thrd, 'ctx>,
        max_area: Area,
    ) -> SubContext<'frm, 'thrd, 'a> {
        ctx.sub_component(max_area, self.get_id(), self.get_component().anchor())
    }
}
