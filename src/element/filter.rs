use std::rc::Rc;
use crate::Context;
use crate::element::{UIWidget, UISocket};

#[derive(Clone, Default)]
pub struct FilterStack {
    pub(crate) pre_filters: Vec<UIFilter>,
    pub(crate) post_filters: Vec<UIFilter>,
}

impl FilterStack {
    pub fn filter_pre(
        &mut self,
        filter: UIFilter,
    ) {
        self.pre_filters.push(filter);
    }

    pub fn filter_post(
        &mut self,
        filter: UIFilter,
    ) {
        self.post_filters.push(filter);
    }
}

pub type UIFilter = Rc<dyn UIFilterImpl>;

pub trait UIFilterImpl {
    fn widget<'ui, 'ctx>(
        &self,
        ctx: &mut Context<'ui, 'ctx>,
        widget: UIWidget,
        filters: &mut FilterStack,
    ) {
        ctx.widget_begin(widget);
            ctx.children_all();
        ctx.end();
    }

    fn socket<'ui, 'ctx>(
        &self,
        ctx: &mut Context<'ui, 'ctx>,
        socket: UISocket<'ctx>,
        filters: &mut FilterStack,
    ) {
        ctx.socket_begin(socket);
            ctx.children_all();
        ctx.end();
    }
}
