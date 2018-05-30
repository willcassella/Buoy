mod context;
mod widget;
mod widgets;

pub mod layout;

pub mod element {
    pub use super::widget::{Widget, WidgetHandler, SyncWidgetHandler};
    pub use super::layout::{Layout, BoundsCalculator};
}

#[cfg(test)]
mod tests {
    // fn identity_filter(ctx: &mut Context, elem: Box<Template>) {
    //     ctx.push_template(elem);
    //         ctx.yield_children();
    //     ctx.pop(); // elem
    // }

    // fn duper(ctx: &mut Context) {
    //     ctx.yield_children();
    //     ctx.yield_children();
    // }

    // fn printer(_ctx: &mut Context) {
    //     println!("Hello");
    // }

    // fn start(ctx: &mut Context) {
    //     ctx.push_template_handler(Box::new(identity_filter));
    //         ctx.push_template(Box::new(duper));
    //             ctx.push_template(Box::new(printer));
    //             ctx.pop(); // printer
    //         ctx.pop(); // super
    //     ctx.pop(); // identity_filter
    // }

    // #[test]
    // fn it_works() {
    //     let mut ctx = Context::new();

    //     ctx.push_template(Box::new(start));
    //     ctx.pop(); // start

    //     ctx.run();
    // }
}
