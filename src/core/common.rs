use crate::core::filter::*;
use crate::state::*;

pub(crate) struct GlobalData {
    pub next_state_id: StateId,
    pub next_frame_filters: FilterStack,
}
