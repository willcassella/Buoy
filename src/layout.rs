#[derive(Clone, Copy, Debug)]
pub struct FlexArea {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub trait Layout {
    fn run(self: Box<Self>, area: Area);
}

pub trait LayoutHandler {
    fn run(self: Box<Self>, flex_area: FlexArea, layout: Box<Layout>);
}
