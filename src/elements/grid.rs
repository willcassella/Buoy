use crate::Context;
use crate::element::{IntoUIElement, Panel, PanelObj, Widget, WidgetObj, Filter, FilterStack};

pub struct Grid {
    pub rows: Vec<GridLine>,
    pub cols: Vec<GridLine>,
}

pub enum GridLine {
    Auto,
    Abs(f32),
    Rem(u32),
}