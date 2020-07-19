use crate::core::context::*;
use crate::core::device::*;
use crate::core::id::Id;
use crate::message::*;
use crate::space::*;
use crate::util::arena::Arena;
use crate::util::ref_move::{ref_move, Anchor};
use std::collections::{hash_map::Entry, HashMap};
use std::rc::Rc;

pub struct GuiContext<C> {
    outgoing_messages: MessageMap,
    pub(in crate::core) renderers: HashMap<TypeId, Rc<dyn IntoRenderer<C>>>,
}

impl<C> Default for GuiContext<C> {
    fn default() -> Self {
        GuiContext {
            outgoing_messages: Default::default(),
            renderers: Default::default(),
        }
    }
}

impl<C: 'static> GuiContext<C> {
    pub fn register_device(&mut self, type_id: TypeId, renderer_factory: Rc<dyn IntoRenderer<C>>) {
        // TODO: assert!(renderer_factory.supports(type_id);
        match self.renderers.entry(type_id) {
            Entry::Occupied(_) => panic!("Cannot register the same type id twice"),
            Entry::Vacant(entry) => {
                entry.insert(renderer_factory);
            }
        }
    }

    pub fn render_window<'frm, D: Anchor<dyn Device + 'frm>>(
        &mut self,
        window_region: Region,
        root: D,
        canvas: &mut C,
    ) {
        // Create a frame context and thread context
        let buffer = Arena::new();
        let frame_context = FrameContext::new(std::mem::take(&mut self.outgoing_messages));
        let mut thread_context = ThreadContext::new(&buffer);

        // Create a renderer for the root and allocate it
        let renderer = thread_context.renderer_for(self, root.get_type_id());
        let device_index = ref_move(root, |root| renderer.alloc(root));

        // Run layout on the device
        let layout_ctx = LayoutContext {
            gui_ctx: self,
            frame_ctx: &frame_context,
            thread_ctx: &thread_context,

            id: Id::default(),
            max_area: window_region.area,
            children: Vec::default(),
        };

        match renderer.layout(device_index, layout_ctx) {
            RendererLayoutResult::None => (),
            RendererLayoutResult::Complete(layout) => {
                // Get the renderer for the layout
                let renderer = thread_context.renderer_for(self, layout.type_id);

                // Render the device
                let render_ctx = RenderContext {
                    region: window_region,
                    gui_ctx: self,
                    thread_ctx: &thread_context,
                };
                renderer.render(layout.index, render_ctx, canvas);
            }
        }

        // Output messages
        let mut thread_outgoing_messages = thread_context.take_outgoing_messages();
        std::mem::drop(thread_context);
        std::mem::drop(frame_context);

        self.outgoing_messages.extend(&mut thread_outgoing_messages);
    }

    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.outgoing_messages.write(outbox, value);
    }
}
