use std::mem;
use context::{GlobalContext, Context};
use widgets::{Fill, min::Min, BlockBorder, max::{Max, VAlign}, hstack::HStack};
use tree::Generator;
use layout::{Region, Area, Point};
use color;
use commands::{CommandList, ColoredQuad};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct buoy_PrimitiveList {
    colored_quads: *mut ColoredQuad,
    num_colored_quads: usize,
    cap_colored_quads: usize,
}

impl From<CommandList> for buoy_PrimitiveList {
    fn from(mut cmd: CommandList) -> Self {
        let res = buoy_PrimitiveList {
            colored_quads: cmd.colored_quads.as_mut_ptr(),
            num_colored_quads: cmd.colored_quads.len(),
            cap_colored_quads: cmd.colored_quads.capacity(),
        };

        mem::forget(cmd);
        return res;
    }
}

#[no_mangle]
pub extern fn buoy_get_primitives(window_width: f32, window_height: f32) -> buoy_PrimitiveList {
    let window_region = Region {
        pos: Point::zero(),
        area: Area {
            width: window_width,
            height: window_height,
        },
    };

    // Build UI
    let mut ctx = GlobalContext::default();
    let elem = ctx.run(window_region.area, Box::new(TestGenerator)).expect("Failed to build UI");

    // Render UI
    let mut commands = CommandList::default();
    elem.render(window_region, &mut commands);

    // Return primitives to caller
    buoy_PrimitiveList::from(commands)
}

#[no_mangle]
pub extern fn buoy_free_primitives(primitives: buoy_PrimitiveList) {
    let commands = unsafe { Vec::from_raw_parts(primitives.colored_quads, primitives.num_colored_quads, primitives.cap_colored_quads) };
    mem::drop(commands);
}

fn test_box(ctx: &mut Context, max_height: f32, v_align: VAlign) {
    Max::default()
    .max_height(max_height)
    .v_align(v_align)
    .push(ctx);

        BlockBorder::uniform(10_f32)
        .color(color::constants::BLUE)
        .push(ctx);

            Fill::new(color::constants::WHITE)
            .push(ctx);

                Min::default()
                .width(100_f32)
                .push(ctx);

                ctx.pop(); // min
            ctx.pop(); // fill
        ctx.pop(); // BlockBorder
    ctx.pop(); // Max
}

struct TestGenerator;
impl Generator for TestGenerator {
    fn run(self: Box<Self>, ctx: &mut Context) {
        HStack::default()
        .push(ctx);

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx);
                test_box(ctx, 500_f32, VAlign::Center);
            ctx.pop(); // BlockBorder

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx);
                test_box(ctx, 200_f32, VAlign::Center);
            ctx.pop(); // BlockBorder

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx);
                test_box(ctx, 400_f32, VAlign::Center);
            ctx.pop(); // BlockBorder

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx);
                test_box(ctx, 300_f32, VAlign::Center);
            ctx.pop(); // BlockBorder

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx);
                test_box(ctx, 700_f32, VAlign::Center);
            ctx.pop(); // BlockBorder

        ctx.pop(); // HStack
    }
}
