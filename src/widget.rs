use super::context::WidgetContext;

pub trait Widget {
    fn get_type(&self) -> i32;

    fn run(self: Box<Self>, ctx: &mut WidgetContext);
}

impl<T> Widget for T where
    T: FnOnce(&mut Context)
{
    fn get_type(&self) -> i32 {
        0
    }

    fn run(self: Box<Self>, ctx: &mut WidgetContext) {
        self(ctx);
    }
}

pub trait WidgetHandler {
    fn run(&self, ctx: &mut WidgetContext, elem: Box<Widget>);
}

impl<T> WidgetHandler for T where
    T: Fn(&mut WidgetContext, Box<Widget>)
{
    fn run(&self, ctx: &mut WidgetContext, elem: Box<Widget>) {
        self(ctx, elem);
    }
}
