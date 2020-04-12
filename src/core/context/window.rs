use crate::core::component::*;
use crate::core::context::{Context, FrameContext, ThreadContext};
use crate::core::id::Id;
use crate::message::*;
use crate::render::CommandList;
use crate::space::*;
use crate::util::arena::Arena;
use crate::util::ref_move::{ref_move, Anchor};
use std::collections::{hash_map::Entry, HashMap};
use std::rc::Rc;

#[derive(Default)]
pub struct Window {
    outgoing_messages: MessageMap,
    pub renderers: HashMap<TypeId, Rc<dyn RendererFactory>>,
}

impl Window {
    pub fn register_component(
        &mut self,
        type_id: TypeId,
        renderer_factory: Rc<dyn RendererFactory>,
    ) {
        // TODO: assert!(renderer_factory.supports(type_id);
        match self.renderers.entry(type_id) {
            Entry::Occupied(_) => panic!("Cannot register the same type id twice"),
            Entry::Vacant(entry) => {
                entry.insert(renderer_factory);
            }
        }
    }

    pub fn run<'a, C: Anchor<dyn DynComponent + 'a>>(
        &mut self,
        commands: &mut CommandList,
        window_region: Region,
        root: C,
    ) {
        // Create a frame context
        let frame_context = FrameContext::new(std::mem::take(&mut self.outgoing_messages));

        // reate a thread context
        let buffer = Arena::new();
        let mut thread_context = ThreadContext::new(&buffer);

        let mut subctx_stack = Vec::new();
        let ctx = Context {
            window_context: self,
            frame_context: &frame_context,
            thread_context: &thread_context,

            id: Id::default(),
            max_area: window_region.area,
            children: Vec::default(),
            subctx_stack: &mut subctx_stack,
        };

        // Allocate the component and run it
        let renderer = thread_context.renderer_for(self, root.get_type_id());
        let index = ref_move(root, |root| renderer.alloc(root));

        // Run the element
        let layout = renderer.layout(ctx, index);
        layout.render(window_region, commands);

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
