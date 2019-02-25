use std::rc::Rc;
use crate::Context;
use crate::element::UIElement;

#[derive(Clone, Default)]
pub struct FilterStack {
    pub(crate) pre_filters: Vec<UIFilter>,
    pub(crate) post_filters: Vec<UIFilter>,
}

impl FilterStack {
    pub fn pre_filter(
        &mut self,
        filter: UIFilter,
    ) {
        self.pre_filters.push(filter);
    }

    pub fn post_filter(
        &mut self,
        filter: UIFilter,
    ) {
        self.post_filters.push(filter);
    }
}

pub type UIFilter = Rc<dyn UIFilterImpl>;

pub trait UIFilterImpl {
    fn element<'ui, 'ctx>(
        &self,
        ctx: &mut Context<'ui, 'ctx>,
        element: UIElement,
        filters: &mut FilterStack,
    );
}
