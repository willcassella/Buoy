use std::mem::replace;
use std::rc::Rc;

use crate::core::common::*;
use crate::core::element::*;
use crate::core::filter::*;
use crate::state::*;
use crate::space::*;
use crate::util::arena::Arena;

#[derive(Default)]
pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,

    prev_frame_state: StateCache,
    cur_frame_state: StateCache,

    next_frame_filters: FilterStack,
    buffer: Arena,
}

impl Window {
    pub fn run<'frm, E: Element>(
        &'frm mut self,
        max_area: Area,
        root: E,
        filter_stack: FilterStackBuilder
    ) -> LayoutNode<'frm> {
        // Increment frame id
        self.frame_id = self.frame_id.next();
        self.next_context_id = Default::default();

        // Swap input caches
        std::mem::swap(&mut self.prev_frame_state, &mut self.cur_frame_state);
        self.cur_frame_state.clear();

        // Get filters for the next frame
        let mut frame_filters = replace(&mut self.next_frame_filters, FilterStack::default());
        frame_filters = filter_stack.append_to(frame_filters);

        // Create a context for running
        let mut global_data = GlobalData {
            next_state_id: StateId::new(self.frame_id, ContextId(0)),
            next_frame_filters: FilterStack::default(),
        };

        let mut subctx_stack = Vec::new();
        let ctx = Context {
            max_area,
            children: SocketTree::default(),
            filter_stack: frame_filters,

            prev_frame_state: &self.prev_frame_state,
            global_data: &mut global_data,
            buffer: &self.buffer,
            subctx_stack: &mut subctx_stack,
        };

        // Run the element
        let result = root.run(ctx, Id::default());

        // TODO: Should do this more elegantly
        self.next_frame_filters = global_data.next_frame_filters;

        result
    }

    pub fn filter(&mut self, filter: Rc<dyn Filter>) {
        self.next_frame_filters.append(filter);
    }

   pub fn write_state<T: StateT>(&mut self, state: State<T>, value: T) {
        if state.id.frame_id != self.frame_id {
            panic!("Writing to state for wrong frame");
        }

        self.cur_frame_state.insert(state.id, Box::new(value));
    }
}
