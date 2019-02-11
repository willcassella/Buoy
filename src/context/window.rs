use std::any::Any;
use std::rc::Rc;
use std::mem::replace;
use crate::util::fill::Fill;
use crate::layout::Area;
use crate::element::{UIElement, Filter, FilterStack};
use crate::render::UIRender;
use super::context::{Context, UINode, UINodeKind, UIElementNode};
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
        root: UIElement,
    ) -> Option<UIRender> {
        // Increment frame id
        self.frame_id = self.frame_id.next();
        self.next_context_id = Default::default();

        // Swap state cache
        self.prev_state_cache = replace(&mut self.cur_state_cache, StateCache::new());

        // Get filters for the next frame
        let frame_filters = replace(&mut self.next_frame_filters, FilterStack::new());

        // Create storage for resulting render
        let mut out: Option<UIRender> = None;

        // Insert root as the initial root
        let mut roots = vec![UINode::from_element(UIElementNode{ elem: root, children: Vec::new() })];

        // Fill the element
        self.fill_element(&mut out, max_area, frame_filters, None, &mut roots);

        return out;
    }

    pub fn attach_frame_filter_pre(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.next_frame_filters.add_filter_pre(filter);
    }

    pub fn attach_frame_filter_post(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.next_frame_filters.add_filter_post(filter);
    }

    fn fill_element(
        &mut self,
        fill: &mut Fill<UIRender>,
        max_area: Area,
        filters: FilterStack,
        _parent_filter: Option<&dyn Filter>,
        roots: &mut Vec<UINode>,
    ) {
        while fill.remaining_capacity() != 0 {
            let root = match roots.pop() {
                Some(x) => x,
                None => return,
            };

            match root.kind {
                UINodeKind::Render(render_obj) => fill.push(render_obj),
                UINodeKind::Element(UIElementNode{ elem, mut children }) => {
                    // If we still have base filters to run on this node
                    // TODO: Also handle parent_filter
                    if root.filter_index < filters.0.len() {
                        // Create a context for running the filter
                        let mut ctx = Context::new(
                            elem.id,
                            max_area,
                            StateId::new(self.frame_id, self.next_context_id.increment()),
                            &self.prev_state_cache);
                        ctx.children = children;

                        // Run the filter
                        let filter = &filters.0[root.filter_index];
                        filter.filter(&mut ctx, elem);

                        // Increment the root filter index
                        for new_root in &mut ctx.roots {
                            new_root.filter_index = root.filter_index + 1;
                        }

                        // Put the results into the root set
                        roots.append(&mut ctx.roots);
                    } else {
                        // Lay out the children of the element
                        let mut socket = elem.imp.open(max_area);

                        // Initialize the socket
                        {
                            let (filter, fill) = socket.imp.init();

                            self.fill_element(fill, socket.child_max_area, elem.filter_stack, filter, &mut children);
                        }

                        // Create a context for closing the socket
                        let mut ctx = Context::new(
                            elem.id,
                            max_area,
                            StateId::new(self.frame_id, self.next_context_id.increment()),
                            &self.prev_state_cache);
                        ctx.children = children;

                        // Close the socket
                        socket.imp.close(&mut ctx);

                        // Put the results into the root set
                        roots.append(&mut ctx.roots);
                    }
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
