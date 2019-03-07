use std::rc::Rc;
use crate::Context;
use crate::element::{UIWidget, UISocket};
use crate::element::widget;

#[derive(Clone, Default)]
pub struct FilterStack {
    pub(crate) filters: Vec<UIFilter>,
    pub(crate) late_filters: Vec<UIFilter>,
}

impl FilterStack {
    pub fn add_filter(
        &mut self,
        filter: UIFilter,
    ) {
        self.filters.push(filter);
    }

    pub fn add_filter_late(
        &mut self,
        filter: UIFilter,
    ) {
        self.late_filters.push(filter);
    }
}

pub type UIFilter = Rc<dyn UIFilterImpl>;

pub trait UIFilterImpl {
    fn target_id(&self) -> Option<widget::Id> {
        None
    }

    fn recurse(&self) -> bool {
        false
    }

    fn widget(
        &self,
        ctx: &mut Context,
        widget: UIWidget,
        _filters: &mut FilterStack,
    ) {
        // ctx.begin_widget(widget);
        //     ctx.anchor_default();
        // ctx.end();
    }

    fn socket(
        &self,
        ctx: &mut Context,
        //socket: UISocket,
        _filters: &mut FilterStack,
    ) {
        // ctx.begin_socket(socket);
        //     ctx.anchor_default();
        // ctx.end();
    }
}
