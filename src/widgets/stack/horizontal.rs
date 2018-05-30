use element::{Widget, LayoutCalculator, BoundsCalculator};
use layout::{Area, Bounds};
use context::{WidgetContext, BoundsContext, LayoutContext};

pub struct StackWidget {
}

impl Widget for StackWidget {
    fn get_type(&self) -> i32 { 1 }

    fn run(self: Box<Self>, ctx: &mut WidgetContext) {
        ctx.push()
    }
}

struct Child {
    prev_child: Option<Rc<Child>>,
    bounds: Bounds,
}

impl Child {
    fn new(prev_child: Option<Rc<Child>>) -> Self {
        Child {
            prev_child,
            bounds: Bounds::zero(),
        }
    }
}

impl BoundsCalculator for Rc<Child> {
    fn run(self, ctx: &mut BoundsContext, child_bounds: &[Bounds]) {
        self.bounds = Bounds::max(child_bounds);
        ctx.set_bounds(bounds);
        ctx.set_layout_fn(self);
    }
}

impl LayoutCalculator for Rc<Child> {
    fn run(self, ctx: &mut BoundsContext, child_bounds: &[Bounds]) {

    }
}
