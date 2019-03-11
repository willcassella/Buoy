use std::rc::Rc;

use crate::space::*;
use crate::input::*;
use crate::core::element::*;
use crate::core::tree::*;
use crate::core::filter::*;
use crate::core::common::*;

pub struct TreeContext<'a> {
    pub(crate) socket: &'a mut dyn Socket,
    pub(crate) max_area: Area,

    pub(crate) prev_input: &'a InputCache,
    pub(crate) global_data: &'a mut GlobalData,
}

impl<'a> TreeContext<'a> {
    pub fn remaining_capacity(&self) -> usize {
        self.socket.remaining_capacity()
    }

    pub fn filter_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.global_data.next_frame_filters.add_filter(filter);
    }

    pub fn filter_next_frame_late(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.global_data.next_frame_filters.add_filter(filter);
    }

    pub fn element<E: Element, T: TreeProvider>(
        &mut self,
        id: Id,
        element: E,
        mut sub_provider: T,
    ) {
        // Create a new element context
        let mut ctx = Context::new(
            &mut sub_provider,
            id,
            self.max_area,
            self.prev_input,
            self.global_data
        );

        element.run(&mut ctx, self.socket);
    }
}
