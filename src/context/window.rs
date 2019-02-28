use std::any::Any;
use std::mem::replace;
use crate::util::fill::Fill;
use crate::layout::Area;
use crate::element::{UIWidget, UIRender, UISocket, UISocketImpl, UIFilter, FilterStack};
use super::context::{Context, ContextData, TreeNode, TreeNodeKind};
use super::state::{State, FrameId, ContextId, StateId, StateCache};

#[derive(Default)]
pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,

    prev_state_cache: StateCache,
    cur_state_cache: StateCache,

    next_frame_filters: FilterStack,
}

impl Window {
    pub fn run(
        &mut self,
        max_area: Area,
        root: UIWidget,
    ) -> Option<UIRender> {
        // Increment frame id
        self.frame_id = self.frame_id.next();
        self.next_context_id = Default::default();

        // Swap state cache
        self.prev_state_cache = replace(&mut self.cur_state_cache, StateCache::new());

        // Get filters for the next frame
        let frame_filters = replace(&mut self.next_frame_filters, FilterStack::default());

        // Create storage for resulting render
        let mut out: Option<UIRender> = None;

        // Insert root as the initial root
        let mut roots = vec![TreeNode{
            kind: TreeNodeKind::Widget(root),
            target: Default::default(),
            children: Vec::new(),
        }];

        // Fill the socket
        self.fill_socket(&mut out, max_area, frame_filters, &mut roots);

        out
    }

    pub fn filter_pre(
        &mut self,
        filter: UIFilter,
    ) {
        self.next_frame_filters.filter_pre(filter);
    }

    pub fn filter_post(
        &mut self,
        filter: UIFilter,
    ) {
        self.next_frame_filters.filter_post(filter);
    }

    fn fill_socket(
        &mut self,
        socket: &mut dyn UISocketImpl,
        max_area: Area,
        mut filters: FilterStack,
        roots: &mut Vec<TreeNode>,
    ) {
        while socket.remaining_capacity() != 0 {
            let mut root = match roots.pop() {
                Some(x) => x,
                None => return,
            };

            match root.kind {
                TreeNodeKind::Render(render_obj) => socket.push(render_obj),
                TreeNodeKind::PreFilter(filter) => {
                    let mut filters = filters.clone();
                    filters.filter_pre(filter);
                    self.fill_socket(socket, max_area, filters, &mut root.children);
                },
                TreeNodeKind::PostFilter(filter) => {
                    let mut filters = filters.clone();
                    filters.filter_post(filter);
                    self.fill_socket(socket, max_area, filters, &mut root.children);
                },
                TreeNodeKind::Socket(mut socket) => {
                    if !filters.pre_filters.is_empty() {
                        unimplemented!()
                    }
                    if !filters.post_filters.is_empty() {
                        unimplemented!()
                    }

                    // Fill the socket
                    self.fill_socket(&mut *socket.imp, socket.max_area, FilterStack::default(), &mut root.children);
                },
                TreeNodeKind::Widget(widget) => {
                    if !filters.pre_filters.is_empty() {
                        unimplemented!()
                    }
                    if !filters.post_filters.is_empty() {
                        unimplemented!()
                    }

                    // // Create a context for running the widget
                    // let mut ctx_data = ContextData::new(
                    //     widget.id,
                    //     max_area,
                    //     StateId::new(self.frame_id, self.next_context_id.increment()),
                    //     &self.prev_state_cache,
                    //     );
                    // let mut ctx = Context::new(&mut ctx_data);

                    // // Run the widget
                    // widget.imp.run(&mut ctx);
                }
            }
        }
    }

    pub fn write_state<T: Default + Clone + Send + Any>(&mut self, state: State<T>, value: T) {
        if state.id.frame_id != self.frame_id {
            panic!("Writing to state for wrong frame");
        }

        self.cur_state_cache.insert(state.id, Box::new(value));
    }
}
