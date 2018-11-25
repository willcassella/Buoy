use std::rc::Rc;
use crate::Context;
use crate::element::UIElementObj;

pub trait Filter {
    fn filter(&self, alias: &Rc<Filter>, ctx: &mut Context, mut element_obj: UIElementObj) {
        element_obj.attach_filter_post(alias.clone());
        ctx.push(element_obj);
            ctx.children();
        ctx.pop();
    }
}

#[derive(Clone, Default)]
pub struct FilterStack(pub Vec<Rc<Filter>>);

impl FilterStack {
    pub fn new() -> Self {
        FilterStack(Vec::new())
    }

    pub fn add_filter_pre(&mut self, filter: Rc<Filter>) {
        self.0.insert(0, filter);
    }

    pub fn add_filter_post(&mut self, filter: Rc<Filter>) {
        self.0.push(filter);
    }
}
