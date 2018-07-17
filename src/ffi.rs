use std::mem;
use context::{Window, Context, WidgetId};
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
    let mut ctx = Window::default();
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

struct BlueBox;
impl Generator for BlueBox {
    fn run(self: Box<Self>, ctx: &mut Context) {
        BlockBorder::uniform(10_f32)
        .color(color::constants::BLUE)
        .push(ctx, WidgetId::str("BlueBox_border"));

            Fill::new(color::constants::WHITE)
            .push(ctx, WidgetId::str("BlueBox_fill"));

                Min::default()
                .width(100_f32)
                .push(ctx, WidgetId::str("BlueBox_inner"));

                ctx.pop(); // BlueBox_inner
            ctx.pop(); // BlueBox_fill
        ctx.pop(); // BlueBox)border
    }
}

struct TestGenerator;
impl Generator for TestGenerator {
    fn run(self: Box<Self>, ctx: &mut Context) {
        HStack::default()
        .push(ctx, WidgetId::str("TestGenerator_stack"));

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx, WidgetId::str("BlueBox_1_padding"));
                Max::default().height(500_f32).v_align(VAlign::Center).push(ctx, WidgetId::str("BlueBox_1_max"));
                    ctx.push_generator(Box::new(BlueBox), WidgetId::str("BlueBox_1")); ctx.pop();
                ctx.pop(); // BlueBox_1_max
            ctx.pop(); // BlueBox_1_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx, WidgetId::str("BlueBox_2_padding"));
                Max::default().height(200_f32).v_align(VAlign::Center).push(ctx, WidgetId::str("BlueBox_2_max"));
                    ctx.push_generator(Box::new(BlueBox), WidgetId::str("BlueBox_2")); ctx.pop();
                ctx.pop(); // BlueBox_2_max
            ctx.pop(); // BlueBox_2_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx, WidgetId::str("BlueBox_3_padding"));
                Max::default().height(400_f32).v_align(VAlign::Center).push(ctx, WidgetId::str("BlueBox_3_max"));
                    ctx.push_generator(Box::new(BlueBox), WidgetId::str("BlueBox_3")); ctx.pop();
                ctx.pop(); // BlueBox_3_max
            ctx.pop(); // BlueBox_3_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx, WidgetId::str("BlueBox_4_padding"));
                Max::default().height(300_f32).v_align(VAlign::Center).push(ctx, WidgetId::str("BlueBox_4_max"));
                    ctx.push_generator(Box::new(BlueBox), WidgetId::str("BlueBox_4")); ctx.pop();
                ctx.pop(); // BlueBox_4_max
            ctx.pop(); // BlueBox_4_padding

            BlockBorder::default().top(15_f32).bottom(15_f32).right(30_f32).push(ctx, WidgetId::str("BlueBox_5_padding"));
                Max::default().height(700_f32).v_align(VAlign::Center).push(ctx, WidgetId::str("BlueBox_5_max"));
                    ctx.push_generator(Box::new(BlueBox), WidgetId::str("BlueBox_5")); ctx.pop();
                ctx.pop(); // BlueBox_5_max
            ctx.pop(); // BlueBox_5_padding

        ctx.pop(); // TestGenerator_stack
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ffi_Generator {
    pub obj: *mut Generator,
}
