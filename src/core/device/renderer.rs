use crate::core::context::{LayoutContext, LayoutNode, LayoutResult, RenderContext};
use crate::core::device::Device;
use crate::util::ref_move::RefMove;
use std::cell::RefCell;

pub enum RendererLayoutResult {
    // TODO: Deferred
    Complete(LayoutNode),
}

pub struct DeviceIndex(pub usize);
pub struct LayoutIndex(pub usize);

pub trait Renderer<'frm, C>: Sized {
    type Device: Device + 'frm;
    type Layout: 'frm;

    // Begins the layout process for a device previously allocated in this renderer with 'alloc'.
    // The renderer may open sockets or subcontexts, and calling ctx.defer() will block until all
    // dependencies are met, then this function will be run again.
    // TODO: Implement ctx.defer()
    // TODO: Maybe allow this to specify which things its deferring on/how?
    fn layout<'thrd>(
        &self,
        device: Self::Device,
        ctx: LayoutContext<'thrd, 'frm, C>,
    ) -> LayoutResult<Self::Layout>;

    fn render<'ctx>(&self, layout: Self::Layout, ctx: RenderContext<'ctx, 'frm, C>, canvas: &mut C);
}

pub trait RendererWrapper<'frm, C> {
    fn alloc(&self, device: RefMove<dyn Device + 'frm>) -> DeviceIndex;

    fn layout<'thrd>(
        &self,
        device: DeviceIndex,
        ctx: LayoutContext<'thrd, 'frm, C>,
    ) -> RendererLayoutResult;

    fn render<'ctx>(&self, layout: LayoutIndex, ctx: RenderContext<'ctx, 'frm, C>, canvas: &mut C);
}

pub trait IntoRenderer<C: 'static> {
    fn into_renderer<'frm>(&'frm self) -> Box<dyn RendererWrapper<'frm, C> + 'frm>;
}

impl<C: 'static, T> IntoRenderer<C> for T
where
    for<'frm> T: Renderer<'frm, C>,
{
    fn into_renderer<'frm>(&'frm self) -> Box<dyn RendererWrapper<'frm, C> + 'frm> {
        Box::new(RendererWrapperImpl {
            renderer: self,
            devices: Default::default(),
            layouts: Default::default(),
        })
    }
}

struct RendererWrapperImpl<'frm, C: 'static, T: Renderer<'frm, C>> {
    renderer: &'frm T,
    devices: RefCell<Vec<Option<T::Device>>>,
    layouts: RefCell<Vec<Option<T::Layout>>>,
}

impl<'frm, C, T: Renderer<'frm, C>> RendererWrapper<'frm, C> for RendererWrapperImpl<'frm, C, T> {
    fn alloc(&self, device: RefMove<dyn Device + 'frm>) -> DeviceIndex {
        assert_eq!(device.get_type_id(), T::Device::type_id());
        let device = unsafe { RefMove::downcast_unchecked::<T::Device>(device).take() };

        let mut devices = self.devices.borrow_mut();
        devices.push(Some(device));

        DeviceIndex(devices.len() - 1)
    }

    fn layout<'ctx, 'thrd>(
        &self,
        device: DeviceIndex,
        ctx: LayoutContext<'thrd, 'frm, C>,
    ) -> RendererLayoutResult {
        let dev = self
            .devices
            .borrow_mut()
            .get_mut(device.0)
            .unwrap()
            .take()
            .unwrap();

        let (min_area, layout) = match self.renderer.layout(dev, ctx) {
            LayoutResult::Complete { min_area, layout } => (min_area, layout),
            LayoutResult::CompleteNode(node) => return RendererLayoutResult::Complete(node),
        };

        let mut layouts = self.layouts.borrow_mut();
        layouts.push(Some(layout));
        let layout_index = LayoutIndex(layouts.len() - 1);

        RendererLayoutResult::Complete(LayoutNode {
            min_area,
            type_id: T::Device::type_id(),
            index: layout_index,
        })
    }

    fn render<'ctx>(&self, layout: LayoutIndex, ctx: RenderContext<'ctx, 'frm, C>, canvas: &mut C) {
        let layout = self
            .layouts
            .borrow_mut()
            .get_mut(layout.0)
            .unwrap()
            .take()
            .unwrap();
        self.renderer.render(layout, ctx, canvas);
    }
}
