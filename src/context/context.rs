use std::marker::PhantomData;
use std::mem::replace;
use crate::layout::Area;
use crate::element::{Id, UIElementObj, Filter};
use crate::render::{UIRender, UIRenderObj};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct StateId(pub u16);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct FrameId(pub u32);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct State<T: Default + Clone + Send> {
    id: StateId,
    frame_id: FrameId,
    _phantom: PhantomData<T>,
}

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

    pub fn from_render(render_obj: UIRenderObj) -> Self {
        UINode {
            kind: UINodeKind::Render(render_obj),
            filter_index: 0,
        }
    }
}

pub enum UINodeKind {
    Render(UIRenderObj),
    Element(UIElementNode),
}

pub struct UIElementNode {
    pub obj: UIElementObj,
    pub children: Vec<UINode>,
}

pub struct Context {
    next_state_id: StateId,
    frame_id: FrameId,

    element_id: Id,
    max_area: Area,

    pub(super) children: Vec<UINode>,
    pub(super) stack: Vec<UIElementNode>,
    pub(super) roots: Vec<UINode>,
}

impl Context {
    pub(super) fn new(
        frame_id: FrameId,
        element_id: Id,
        max_area: Area,
    ) -> Self {
        Context {
            next_state_id: StateId(0_u16),
            frame_id,
            element_id,
            max_area,
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

    pub fn push(&mut self, obj: UIElementObj) {
        let node = UIElementNode {
            obj,
            children: Vec::new(),
        };

        self.stack.push(node);
    }

    pub fn render(&mut self, render_obj: UIRenderObj) {
        let node = UINode::from_render(render_obj);

        if let Some(parent) = self.stack.last_mut() {
            parent.children.insert(0, node);
        } else {
            self.roots.insert(0, node);
        }
    }

    pub fn render_new(&mut self, min_area: Area, render: Box<UIRender>) {
        self.render(UIRenderObj{ min_area, render });
    }

    pub fn children(&mut self) {
        let dest = match self.stack.last_mut() {
            Some(parent) => &mut parent.children,
            None => &mut self.roots,
        };

        self.children.append(dest);
        *dest = replace(&mut self.children, Vec::new());
    }

    pub fn next_frame(&mut self, _filter: Box<Filter>) {
        unimplemented!()
    }

    pub fn new_state<T: Default + Clone + Send>(&mut self) -> State<T> {
        let id = self.next_state_id;
        self.next_state_id.0 += 1;

        State {
            id,
            frame_id: self.frame_id,
            _phantom: PhantomData,
        }
    }

    pub fn new_state_default<T: Default + Clone + Send>(&mut self, _default: T) -> State<T> {
        let id = self.next_state_id;
        self.next_state_id.0 += 1;

        State {
            id,
            frame_id: self.frame_id,
            _phantom: PhantomData,
        }
    }

    pub fn read_state<T: Default + Clone + Send>(&self, _state: State<T>) -> T {
        if _state.frame_id == self.frame_id {
            panic!("Attempt to read state from current frame!");
        }

        T::default()
    }
}