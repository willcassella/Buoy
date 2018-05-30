use super::context::WidgetContext;

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub max_width: f32,
    pub max_height: f32,
}

impl Bounds {
    pub fn bounded(max_width: f32, max_height: f32) -> Self {
        Bounds {
            max_width,
            max_height,
        }
    }

    pub fn bounded_width(max_width: f32) -> Self {
        Bounds {
            max_width,
            max_height: ::std::f32::INFINITY,
        }
    }

    pub fn bounded_height(max_height: f32) -> Self {
        Bounds {
            max_width: ::std::f32::INFINITY,
            max_height,
        }
    }
}

pub trait Widget {
    fn get_type(&self) -> i32;

    fn run(self: Box<Self>, ctx: &mut WidgetContext, bounds: Bounds);
}

impl<T> Widget for T where
    T: FnOnce(&mut WidgetContext, Bounds)
{
    fn get_type(&self) -> i32 {
        0
    }

    fn run(self: Box<Self>, ctx: &mut WidgetContext, bounds: Bounds) {
        self(ctx, bounds);
    }
}

pub trait WidgetHandler {
    fn run(self: Box<Self>, ctx: &mut WidgetContext, elem: Box<Widget>);
}

impl<T> WidgetHandler for T where
    T: FnOnce(&mut WidgetContext, Box<Widget>)
{
    fn run(self: Box<Self>, ctx: &mut WidgetContext, elem: Box<Widget>) {
        self(ctx, elem);
    }
}
