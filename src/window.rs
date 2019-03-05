use std::any::Any;
use std::mem::replace;
use crate::util::fill::Fill;
use crate::layout::Area;
use crate::element::{UIWidget, UIRender, UISocket, UISocketImpl, UIFilter, FilterStack};
use super::context::{Context, ContextData, TreeNode, TreeNodeKind};
use super::input::{Input, InputState, FrameId, ContextId, InputId, InputCache};

#[derive(Default)]
pub struct Window {
    frame_id: FrameId,
    next_context_id: ContextId,

    prev_input: InputCache,
    cur_input: InputCache,

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

        // Swap input caches
        std::mem::swap(&mut self.prev_input, &mut self.cur_input);
        self.cur_input.clear();

        // Get filters for the next frame
        let frame_filters = replace(&mut self.next_frame_filters, FilterStack::default());

        // Create storage for resulting render
        let mut out: Option<UIRender> = None;

        // Insert root as the initial root
        let mut roots = vec![TreeNode{
            kind: TreeNodeKind::Widget(root),
            children: Vec::new(),
        }];

        // Fill the socket
        self.fill_socket(&mut out, max_area, frame_filters, &mut roots);

        out
    }

    pub fn filter(
        &mut self,
        filter: UIFilter,
    ) {
        self.next_frame_filters.add_filter(filter);
    }

    pub fn filter_late(
        &mut self,
        filter: UIFilter,
    ) {
        self.next_frame_filters.add_filter_late(filter);
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
                TreeNodeKind::Filter(filter) => {
                    let mut filters = filters.clone();
                    filters.add_filter(filter);
                    self.fill_socket(socket, max_area, filters, &mut root.children);
                },
                TreeNodeKind::FilterLate(filter) => {
                    let mut filters = filters.clone();
                    filters.add_filter_late(filter);
                    self.fill_socket(socket, max_area, filters, &mut root.children);
                },
                TreeNodeKind::Socket(mut socket) => {
                    if !filters.filters.is_empty() {
                        unimplemented!()
                    }
                    if !filters.late_filters.is_empty() {
                        unimplemented!()
                    }

                    // Fill the socket
                    self.fill_socket(&mut *socket.imp, socket.max_area, FilterStack::default(), &mut root.children);
                },
                TreeNodeKind::Widget(widget) => {
                    if !filters.filters.is_empty() {
                        unimplemented!()
                    }
                    if !filters.late_filters.is_empty() {
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
                },
                TreeNodeKind::Resume(resume) => {
                    // Create a context for running the widget
                    //let mut ctx_data = ContextData::new()
                },
            }
        }
    }

    pub fn send_input<T: InputState>(&mut self, input: Input<T>, value: T) {
        if input.id.frame_id != self.frame_id {
            panic!("Writing to state for wrong frame");
        }

        self.cur_input.insert(input.id, Box::new(value));
    }
}
