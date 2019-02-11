use std::rc::Rc;
use crate::Context;
use crate::element::UIElement;

pub trait Filter {
    fn check(&self, _elem: &UIElement) -> bool {
        true
    }

    fn filter(&self, ctx: &mut Context, elem: UIElement);
}

#[derive(Clone, Default)]
pub struct FilterStack(pub Vec<Rc<dyn Filter>>);

impl FilterStack {
    pub fn new() -> Self {
        FilterStack(Vec::new())
    }

    pub fn add_filter_pre(&mut self, filter: Rc<dyn Filter>) {
        self.0.insert(0, filter);
    }

    pub fn add_filter_post(&mut self, filter: Rc<dyn Filter>) {
        self.0.push(filter);
    }

    pub fn append(&mut self, other: &mut FilterStack) {
        self.0.append(&mut other.0);
    }
}
