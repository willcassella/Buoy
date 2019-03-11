use crate::input::*;
use crate::core::filter::*;

pub(crate) struct GlobalData {
    pub next_input_id: InputId,
    pub next_frame_filters: FilterStack,
}
