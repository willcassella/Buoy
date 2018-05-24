use super::context::LayoutContext;

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub min_width: f32,
    pub min_height: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub trait Layout {
    fn run(self: Box<Self>, ctx: &mut LayoutContext, area: Area);
}

pub trait BoundsCalculator {
    fn run(self: Box<Self>, ctx: &mut LayoutContext, child_bounds: &[Bounds]);
}
