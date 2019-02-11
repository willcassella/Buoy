use std::rc::Rc;
use std::mem::replace;
use std::any::Any;
use crate::layout::Area;
use crate::element::{Id, UIElement, Filter, FilterStack};
use crate::render::{UIRender, UIRenderImpl};
use super::state::{State, StateId, StateCache};

pub struct UINode {
    pub kind: UINodeKind,
    pub filter_index: usize,
}

impl UINode {
    pub fn from_element(element_node: UIElementNode) -> Self {
        UINode {
            kind: UINodeKind::Element(element_node),
            filter_index: 0,
        }
    }

    pub fn from_render(render_obj: UIRender) -> Self {
        UINode {
            kind: UINodeKind::Render(render_obj),
            filter_index: 0,
        }
    }
}

pub enum UINodeKind {
    Render(UIRender),
    Element(UIElementNode),
}

pub struct UIElementNode {
    pub elem: UIElement,
    pub children: Vec<UINode>,
}

pub struct Context<'ui> {
    element_id: Id,
    max_area: Area,
    next_state_id: StateId,
    prev_state_cache: &'ui StateCache,
    new_state_cache: StateCache,

    next_frame_filters: FilterStack,

    pub(super) children: Vec<UINode>,
    pub(super) stack: Vec<UIElementNode>,
    pub(super) roots: Vec<UINode>,
}

impl<'ui> Context<'ui> {
    pub(super) fn new(
        element_id: Id,
        max_area: Area,
        next_state_id: StateId,
        prev_state_cache: &'ui StateCache,
    ) -> Self {
        Context {
            element_id,
            max_area,
            next_state_id,
            prev_state_cache,
            new_state_cache: StateCache::new(),
            next_frame_filters: FilterStack::new(),
            children: Vec::new(),
            stack: Vec::new(),
            roots: Vec::new(),
        }
    }

    pub fn element_id(&self) -> Id {
        self.element_id
    }

    pub fn max_area(&self) -> Area {
        self.max_area
    }

    // Moves the context upward to the parent of the current element
    // This will panic if the parent is not in scope for this context!
    pub fn pop(&mut self) {
        let node = self.stack.pop().expect("Bad pop");
        let node = UINode::from_element(node);

        if let Some(parent) = self.stack.last_mut() {
            parent.children.insert(0, node);
        } else {
            self.roots.insert(0, node);
        }
    }

    pub fn push(&mut self, elem: UIElement) {
        let node = UIElementNode {
            elem,
            children: Vec::new(),
        };

        self.stack.push(node);
    }

    pub fn render(&mut self, render: UIRender) {
        let node = UINode::from_render(render);

        if let Some(parent) = self.stack.last_mut() {
            parent.children.insert(0, node);
        } else {
            self.roots.insert(0, node);
        }
    }

    pub fn render_new(&mut self, min_area: Area, imp: Box<UIRenderImpl>) {
        self.render(UIRender{ min_area, imp });
    }

    pub fn children(&mut self) {
        let dest = match self.stack.last_mut() {
            Some(parent) => &mut parent.children,
            None => &mut self.roots,
        };

        self.children.append(dest);
        *dest = replace(&mut self.children, Vec::new());
    }

    pub fn next_frame(&mut self, filter: Rc<Filter>) {
        self.next_frame_filters.add_filter_post(filter);
    }

    pub fn new_state<T: Default + Clone + Send + Any>(&mut self) -> State<T> {
        let id = self.next_state_id.increment();
        State::new(id)
    }

    pub fn new_state_default<T: Default + Clone + Send + Any>(&mut self, default: T) -> State<T> {
        let id = self.next_state_id.increment();
        self.new_state_cache.insert(id, Box::new(default));
        State::new(id)
    }

    pub fn read_state<T: Default + Clone + Send + Any>(&self, state: State<T>) -> T {
        if state.id.frame_id != self.next_state_id.frame_id.prev() {
            panic!("Attempt to read state from wrong frame");
        }

        if let Some(v) = self.prev_state_cache.get(&state.id) {
            v.downcast_ref::<T>().expect("Mismatched types").clone()
        } else {
            Default::default()
        }
    }
}
