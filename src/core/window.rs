use std::mem::replace;
use std::rc::Rc;

use crate::core::common::*;
use crate::core::element::*;
use crate::core::filter::*;
use crate::state::*;
use crate::space::*;
use crate::util::linear_buffer::LinearBuffer;

#[derive(Default)]
pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,

    prev_frame_state: StateCache,
    cur_frame_state: StateCache,

    next_frame_filters: FilterStack,
    buffer: LinearBuffer,
}

impl Window {
    pub fn run<E: Element>(&mut self, max_area: Area, root: E) -> LayoutObj {
        // Increment frame id
        self.frame_id = self.frame_id.next();
        self.next_context_id = Default::default();

        // Swap input caches
        std::mem::swap(&mut self.prev_frame_state, &mut self.cur_frame_state);
        self.cur_frame_state.clear();

        // Get filters for the next frame
        let frame_filters = replace(&mut self.next_frame_filters, FilterStack::default());

        // Create a context for running
        let mut global_data = GlobalData {
            next_state_id: StateId::new(self.frame_id, ContextId(0)),
            next_frame_filters: FilterStack::default(),
        };

        let ctx = Context {
            max_area,
            children: Children::new(),

            prev_frame_state: &self.prev_frame_state,
            global_data: &mut global_data,
            buffer: &self.buffer,
        };

        // Run the element
        root.run(ctx, Id::default())
    }

    pub fn filter(&mut self, filter: Rc<dyn Filter>) {
        self.next_frame_filters.add_filter(filter);
    }

    pub fn filter_late(&mut self, filter: Rc<dyn Filter>) {
        self.next_frame_filters.add_filter_late(filter);
    }

   pub fn write_state<T: StateT>(&mut self, state: State<T>, value: T) {
        if state.id.frame_id != self.frame_id {
            panic!("Writing to state for wrong frame");
        }

        self.cur_frame_state.insert(state.id, Box::new(value));
    }
}
