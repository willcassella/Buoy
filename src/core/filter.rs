use std::rc::Rc;

use crate::core::*;

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
    fn target_id(&self) -> Option<element::Id> {
        None
    }

    fn recurse(&self) -> bool {
        false
    }

    fn element(
        &self,
        _ctx: &mut Context,
        _element: Box<dyn element::DynElement>,
        _filters: &mut FilterStack,
    ) {
        // ctx.begin_widget(widget);
        //     ctx.anchor_default();
        // ctx.end();
    }

    fn socket(
        &self,
        _ctx: &mut Context,
        //socket: UISocket,
        _filters: &mut FilterStack,
    ) {
        // ctx.begin_socket(socket);
        //     ctx.anchor_default();
        // ctx.end();
    }
}
