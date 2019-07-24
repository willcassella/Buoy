use std::rc::Rc;

use crate::core::element::*;
use crate::util::linked_buffer::LBBox;

#[derive(Clone, Default)]
pub struct FilterStack {
    pub(crate) filters: Vec<Rc<dyn Filter>>,
    pub(crate) late_filters: Vec<Rc<dyn Filter>>,
}

impl FilterStack {
    pub fn add_filter(&mut self, filter: Rc<dyn Filter>) {
        self.filters.push(filter);
    }

    pub fn add_filter_late(&mut self, filter: Rc<dyn Filter>) {
        self.late_filters.push(filter);
    }
}

pub trait Filter {
    fn element<'ctx, 'frm>(
        &self,
        mut ctx: Context<'ctx, 'frm>,
        id: Id,
        element: LBBox<'frm, dyn Element>,
        _filters: &mut FilterStack
    ) -> LayoutNode<'frm> {
        // Default implementation just uses the element as a sub-element (no-op)
        let mut sub = ctx.open_element(ctx.max_area(), id, element);
        sub.connect_all_sockets();
        sub.close()
    }
}