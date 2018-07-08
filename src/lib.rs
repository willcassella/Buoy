pub mod context;
pub mod tree;
pub mod widgets;
pub mod command_list;
pub mod layout;

#[cfg(test)]
mod tests {
    use context::{GlobalContext, Context};
    use widgets::{BlockBorder, Max, HAlign};
    use tree::Generator;
    use layout::Bounds;
    use command_list::constants::*;

    struct TestGenerator;
    impl Generator for TestGenerator {
        fn run(self: Box<Self>, ctx: &mut Context) {
            Max::default()
                .h_align(HAlign::Right)
                .max_width(100_f32)
                .push(ctx);

                BlockBorder::uniform(10_f32)
                    .color(BLUE)
                    .push(ctx);
                ctx.pop(); // BlockBorder
            ctx.pop(); // Max
        }
    }


    #[test]
    fn it_works() {
        let mut ctx = GlobalContext::default();
        let bounds = Bounds {
            width: 800_f32,
            height: 600_f32,
        };

        let elem = ctx.run(bounds, Box::new(TestGenerator));
    }
}
