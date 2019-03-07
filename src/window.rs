use std::rc::Rc;
use std::mem::replace;
use crate::core::*;
use crate::layout::Area;
use super::input::{Input, InputState, FrameId, ContextId, InputId, InputCache};

#[derive(Default)]
pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,

    prev_input: InputCache,
    cur_input: InputCache,

    next_frame_filters: filter::FilterStack,
}

impl Window {
    pub fn run<E: Element>(
        &mut self,
        max_area: Area,
        root: E,
    ) -> Option<Render> {
        // Increment frame id
        self.frame_id = self.frame_id.next();
        self.next_context_id = Default::default();

        // Swap input caches
        std::mem::swap(&mut self.prev_input, &mut self.cur_input);
        self.cur_input.clear();

        // Get filters for the next frame
        let frame_filters = replace(&mut self.next_frame_filters, filter::FilterStack::default());

        // Create root socket
        let mut out: Option<Render> = None;

        // Create a context for running
        let mut global_data = context::GlobalData {
           next_input_id: InputId::new(self.frame_id, ContextId(0)),
           next_frame_filters: filter::FilterStack::default(),
        };

        let mut tree_provider = context::NullTree;
        let mut ctx = Context::new(
            &mut tree_provider,
            element::Id::from(""),
            max_area,
            &self.prev_input,
            &mut global_data,
        );

        // Run the element
        root.run(&mut ctx, &mut out);

        out
    }

    pub fn filter(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.next_frame_filters.add_filter(filter);
    }

    pub fn filter_late(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.next_frame_filters.add_filter_late(filter);
    }

    pub fn send_input<T: InputState>(&mut self, input: Input<T>, value: T) {
        if input.id.frame_id != self.frame_id {
            panic!("Writing to state for wrong frame");
        }

        self.cur_input.insert(input.id, Box::new(value));
    }
}
