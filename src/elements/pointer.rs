use tree::{Socket, Element, NullElement};
use layout::Area;
use context::{State, Context};

pub struct PointerActivate {
    pub action: Option<Box<Fn()>>,
    pub active: State<bool>,
}

impl Socket for PointerActivate {
    fn child(
        self: Box<Self>,
        ctx: &mut Context,
        child_min: Area,
        child: Box<Element>,
    ) {
        ctx.element(child_min, child);
    }

    fn close(
        self: Box<Self>,
        ctx: &mut Context,
    ) {
        ctx.element(Area::zero(), Box::new(NullElement));
    }
}

pub struct PointerHover {
    pub action: Option<Box<Fn()>>,
    pub active: State<bool>,
}

impl Socket for PointerHover {
    fn child(
        self: Box<Self>,
        ctx: &mut Context,
        child_min: Area,
        child: Box<Element>,
    ) {
        ctx.element(child_min, child);
    }

    fn close(
        self: Box<Self>,
        ctx: &mut Context,
    ) {
        ctx.element(Area::zero(), Box::new(NullElement));
    }
}

pub struct PointerDelta {
    pub delta: State<(f32, f32)>,
}

impl Socket for PointerDelta {
    fn child(
        self: Box<Self>,
        ctx: &mut Context,
        child_min: Area,
        child: Box<Element>,
    ) {
        ctx.element(child_min, child);
    }

    fn close(
        self: Box<Self>,
        ctx: &mut Context,
    ) {
        ctx.element(Area::zero(), Box::new(NullElement));
    }
}