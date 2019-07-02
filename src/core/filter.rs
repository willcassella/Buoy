use std::rc::Rc;

use crate::core::common::*;
use crate::core::element::*;
use crate::core::tree::*;

#[derive(Clone, Default)]
pub struct FilterStack {
    pub(crate) filters: Vec<Rc<dyn Filter>>,
    pub(crate) late_filters: Vec<Rc<dyn Filter>>,
}

impl FilterStack {
    pub fn add_filter(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.filters.push(filter);
    }

    pub fn add_filter_late(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.late_filters.push(filter);
    }
}

pub trait Filter {
    fn element<'window, 'ctx>(
        &self,
        ctx: TreeContext<'window, 'ctx>,
        id: Id,
        element: &dyn Element,
        children: &mut dyn TreeProvider,
        _filters: &mut FilterStack,
    ) {
        //ctx.element(id, element, children);
        unimplemented!()
    }
}
