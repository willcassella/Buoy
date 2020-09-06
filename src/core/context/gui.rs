use crate::core::context::*;
use crate::core::device::*;
use crate::message::*;
use crate::space::*;
use crate::util::arena::Arena;
use crate::util::ref_move::{ref_move, Anchor};
use std::collections::{hash_map::Entry, HashMap};
use std::marker::PhantomData;

pub struct GuiContext<C> {
    outgoing_messages: MessageMap,
    _p: PhantomData<C>,
}

impl<C> Default for GuiContext<C> {
    fn default() -> Self {
        GuiContext {
            outgoing_messages: Default::default(),
            _p: PhantomData,
        }
    }
}

impl<C: 'static> GuiContext<C> {
    pub fn render_window<'frm, D: Device<C, Space = Space2D> + 'frm>(
        &mut self,
        window_region: Region2D,
        root: D,
        canvas: &mut C,
    ) {
        // // Create a frame context and thread context
        // let buffer = Arena::new();
        // let frame_context = FrameContext::new(std::mem::take(&mut self.outgoing_messages));
        // let mut thread_context = ThreadContext::new(&buffer);

        // // Create a renderer for the root and allocate it
        // let renderer = thread_context.renderer_for(self, root.get_type_id());
        // let device_index = ref_move(root, |root| renderer.alloc(root));

        // // Run layout on the device
        // let layout_ctx = LayoutContext {
        //     gui_ctx: self,
        //     frame_ctx: &frame_context,
        //     thread_ctx: &thread_context,

        //     max_size: window_region.size,
        //     children: Vec::default(),
        // };

        // match renderer.layout(device_index, layout_ctx) {
        //     RendererLayoutResult::None => (),
        //     RendererLayoutResult::Complete(layout) => {
        //         // Get the renderer for the layout
        //         let renderer = thread_context.renderer_for(self, layout.type_id);

        //         // Render the device
        //         let render_ctx = RenderContext {
        //             region: window_region,
        //             gui_ctx: self,
        //             thread_ctx: &thread_context,
        //         };
        //         renderer.render(layout.index, render_ctx, canvas);
        //     }
        // }

        // // Output messages
        // let mut thread_outgoing_messages = thread_context.take_outgoing_messages();
        // std::mem::drop(thread_context);
        // std::mem::drop(frame_context);

        // self.outgoing_messages.extend(&mut thread_outgoing_messages);
    }

    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.outgoing_messages.write(outbox, value);
    }
}
