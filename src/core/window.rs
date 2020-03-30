use crate::core::element::*;
use crate::core::filter::*;
use crate::core::id::*;
use crate::message::*;
use crate::space::*;
use crate::util::arena::Arena;

#[derive(Default)]
pub struct Window {
    incoming_messages: MessageMap,
    outgoing_messages: MessageMap,
    buffer: Arena,
}

impl Window {
    pub fn run<E: Element>(
        &mut self,
        max_area: Area,
        root: E,
        filter_stack: FilterStack,
    ) -> LayoutNode<'_> {
        assert!(filter_stack.is_empty(), "Filters aren't working yet");

        std::mem::swap(&mut self.incoming_messages, &mut self.outgoing_messages);
        self.outgoing_messages.clear();

        let mut subctx_stack = Vec::new();
        let ctx = Context {
            max_area,
            children: SocketTree::default(),
            filter_stack: FilterStack::default(),

            incoming_messages: &self.incoming_messages,
            outgoing_messages: &mut self.outgoing_messages,
            buffer: &self.buffer,
            subctx_stack: &mut subctx_stack,
        };

        // Run the element
        root.run(ctx, Id::default())
    }

    pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.outgoing_messages.insert(outbox.id(), Box::new(value));
    }
}
