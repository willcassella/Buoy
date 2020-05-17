mod type_id;
pub use type_id::TypeId;

mod socket;
pub use socket::{Socket, SocketCapacity, SocketName};

mod renderer;
pub use self::renderer::{
    DeviceIndex, IntoRenderer, LayoutIndex, Renderer, RendererLayoutResult, RendererWrapper,
};

// This trait is unsafe because a careless implementation
// of 'get_type_id()' (ie, returning a different TypeId than what this was registered with)
// will cause an improper downcast.
pub unsafe trait DynDevice {
    fn get_type_id(&self) -> TypeId;
}

pub trait Device: DynDevice {
    fn type_id() -> TypeId
    where
        Self: Sized;
}

auto_impl_upcast!(dyn Device);

unsafe impl<T: Device> DynDevice for T {
    fn get_type_id(&self) -> TypeId {
        T::type_id()
    }
}
