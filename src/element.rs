use std::rc::Rc;
use super::context::{Context};

pub trait Template {
    fn get_type(&self) -> i32;

    fn box_clone(&self) -> Box<Template>;

    fn run(&self, ctx: &mut Context);
}

impl<T> Template for T where
    T: Clone,
    T: Fn(&mut Context),
    T: 'static
{
    fn get_type(&self) -> i32 {
        0
    }

    fn box_clone(&self) -> Box<Template> {
        Box::new(self.clone())
    }

    fn run(&self, ctx: &mut Context) {
        self(ctx);
    }
}

impl Clone for Box<Template> {
    fn clone(&self) -> Box<Template> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct TemplateElement {
    pub template: Box<Template>,
    pub children: Vec<TemplateElement>,
    pub handler: Option<Rc<TemplateHandlerElement>>,
}

impl TemplateElement {
    pub fn new(template: Box<Template>, handler: Option<Rc<TemplateHandlerElement>>) -> Self {
        Self {
            template,
            children: Vec::new(),
            handler,
        }
    }
}

pub trait TemplateHandler {
    fn run(&self, ctx: &mut Context, elem: Box<Template>);
}

impl<T> TemplateHandler for T where
    T: Fn(&mut Context, Box<Template>)
{
    fn run(&self, ctx: &mut Context, elem: Box<Template>) {
        self(ctx, elem);
    }
}

pub struct TemplateHandlerElement {
    pub handler: Box<TemplateHandler>,
    pub parent: Option<Rc<TemplateHandlerElement>>,
}

impl TemplateHandlerElement {
    pub fn new(handler: Box<TemplateHandler>, parent: Option<Rc<TemplateHandlerElement>>) -> Self {
        Self {
            handler,
            parent
        }
    }
}
