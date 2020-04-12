use crate::prelude::Window;

pub mod archetype;

pub mod border;
pub use border::Border;

pub mod fill;
pub use fill::Fill;

pub mod grid;
pub use grid::{ColIndex, Grid, GridRegion, GridTrack, RowIndex};

pub mod hover;
pub use hover::Hover;

pub mod click;
pub use click::Click;

pub mod list;
pub use list::{List, ListDir};

pub mod size;
pub use size::Size;

pub mod overlap;
pub use overlap::Overlap;

pub fn register_primitive_components(window: &mut Window) {
    border::register(window);
    fill::register(window);
    grid::register(window);
    hover::register(window);
    click::register(window);
    list::register(window);
    size::register(window);
    overlap::register(window);
}
