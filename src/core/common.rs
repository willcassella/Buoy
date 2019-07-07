use crate::core::filter::*;
use crate::input::*;

pub(crate) struct GlobalData {
    pub next_input_id: InputId,
    pub next_frame_filters: FilterStack,
}
