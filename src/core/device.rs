mod socket;
pub use socket::{Socket, SocketSource};

use crate::core::context::{LayoutContext, RenderContext};
use crate::space::Space;

pub trait Device<C> {
    type Space: Space;

    fn layout(self, ctx: LayoutContext<C, Self::Space>);
}

pub trait DeviceWrapper<C> {
    type Space: Space;

    fn layout(self: Box<Self>, ctx: LayoutContext<C, Self::Space>);
}

impl<C, T: Device<C>> DeviceWrapper<C> for T {
    type Space = T::Space;

    fn layout(self: Box<Self>, ctx: LayoutContext<C, Self::Space>) {
        (*self).layout(ctx);
    }
}

pub trait Layout<C> {
    type Space: Space;

    fn render(self, ctx: RenderContext<C, Self::Space>, canvas: &mut C);
}

pub trait LayoutWrapper<C> {
    type Space: Space;

    fn render(self: Box<Self>, ctx: RenderContext<C, Self::Space>, canvas: &mut C);
}

impl<C, T: Layout<C>> LayoutWrapper<C> for T {
    type Space = T::Space;

    fn render(self: Box<Self>, ctx: RenderContext<C, Self::Space>, canvas: &mut C) {
        (*self).render(ctx, canvas);
    }
}
