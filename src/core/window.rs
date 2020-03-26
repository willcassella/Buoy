use crate::core::id::*;
use crate::core::element::*;
use crate::core::filter::*;
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
    pub fn run<'frm, E: Element>(
        &'frm mut self,
        max_area: Area,
        root: E,
        filter_stack: FilterStack,
    ) -> LayoutNode<'frm> {
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
        let result = root.run(ctx, Id::default());
        result
    }

   pub fn write_message<T: Message>(&mut self, outbox: Outbox<T>, value: T) {
        self.outgoing_messages.insert(outbox.id(), Box::new(value));
    }
}
