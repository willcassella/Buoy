use crate::core::common::*;
use crate::core::element::*;
use crate::core::tree::*;

pub trait TreeProvider {
    // TODO: This should return something more descriptive than true/false
    fn socket<'a, C: TreeContext<'a>>(
        &mut self,
        ctx: C,
        name: SocketName,
    ) -> bool where Self: Sized;
}

impl TreeProvider for () {
    fn socket<'b, C: TreeContext<'b>>(
        &mut self,
        _ctx: C,
        _name: SocketName,
    ) -> bool {
        // Do nothing
        false
    }
}

pub trait DynTreeProvider: TreeProvider {
    fn socket_dyn<'a>(
        &mut self,
        ctx: DynTreeContext<'a>,
        name: SocketName,
    ) -> bool;
}

impl<T: TreeProvider> DynTreeProvider for T {
    fn socket_dyn<'a>(
        &mut self,
        ctx: DynTreeContext<'a>,
        name: SocketName,
    ) -> bool {
        self.socket(ctx, name)
    }
}

pub trait TreeProviderRef<'a>: Sized + 'a {
    fn invoke_socket<'b, C: TreeContext<'b>>(
        &mut self,
        ctx: C,
        name: SocketName,
    ) -> bool;

    fn upcast_mut(
        self,
    ) -> &'a mut dyn DynTreeProvider;
}

impl<'a, T: TreeProvider> TreeProviderRef<'a> for &'a mut T {
    fn invoke_socket<'b, C: TreeContext<'b>>(
        &mut self,
        ctx: C,
        name: SocketName,
    ) -> bool {
        self.socket(ctx, name)
    }

    fn upcast_mut(
        self,
    ) -> &'a mut dyn DynTreeProvider {
        self
    }
}

impl<'a> TreeProviderRef<'a> for &'a mut dyn DynTreeProvider {
    fn invoke_socket<'b, C: TreeContext<'b>>(
        &mut self,
        ctx: C,
        name: SocketName,
    ) -> bool {
        self.socket_dyn(ctx.upcast(), name)
    }

    fn upcast_mut(
        self,
    ) -> &'a mut dyn DynTreeProvider {
        self
    }
}
