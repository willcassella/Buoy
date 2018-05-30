use context::WidgetContext;
use widget::{Bounds, Widget};
use layout::{Area, FlexArea, Layout};

pub struct StackWidget {
}

impl Widget for StackWidget {
    fn get_type(&self) -> i32 { 1 }

    fn run(self: Box<Self>, ctx: &mut WidgetContext, bounds: Bounds) {
        ctx.push_bounds(Bounds::bounded_height(bounds.max_height));
            
        ctx.pop();
    }
}
