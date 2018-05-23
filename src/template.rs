use super::context::Context;

pub trait Template {
    fn get_type(&self) -> i32;

    fn box_clone(&self) -> Box<Template>;

    fn run(self, ctx: &mut Context);
}

impl<T> Template for T where
    T: Clone,
    T: FnOnce(&mut Context),
    T: 'static
{
    fn get_type(&self) -> i32 {
        0
    }

    fn box_clone(&self) -> Box<Template> {
        Box::new(self.clone())
    }

    fn run(self, ctx: &mut Context) {
        self(ctx);
    }
}

impl Clone for Box<Template> {
    fn clone(&self) -> Box<Template> {
        self.box_clone()
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
