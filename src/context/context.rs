use std::rc::Rc;
use std::marker::PhantomData;
use std::mem::replace;
use layout::Area;
use tree::{Filter, Generator, Socket, Element};
use context::{WidgetId, WidgetInfo};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct StateId(pub u16);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct FrameId(pub u16);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct ContextId(pub u32);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct State<T: Default + Clone + Send> {
    id: StateId,
    frame_id: FrameId,
    context_id: ContextId,
    _phantom: PhantomData<T>,
}

pub enum TreeNode {
    Terminal(Area, Box<Element>),
    NonTerminal(NonTerminalNode),
}

pub struct NonTerminalNode {
    pub kind: NonTerminalKind,
    pub children: Vec<TreeNode>,
}

pub enum NonTerminalKind {
    Filter(Rc<Filter>),
    Generator(WidgetInfo, Box<Generator>),
    Socket(WidgetInfo, Box<Socket>),
}

pub struct Context {
    next_state_id: StateId,
    frame_id: FrameId,
    context_id: ContextId,

    widget_id: WidgetId,
    bounds: Area,

    pub(super) children: Vec<TreeNode>,
    pub(super) stack: Vec<NonTerminalNode>,
    pub(super) roots: Vec<TreeNode>,
}

impl Context {
    pub(super) fn new(
        frame_id: FrameId,
        context_id: ContextId,
        widget_id: WidgetId,
        bounds: Area,
    ) -> Self {
        Context {
            next_state_id: StateId(0_u16),
            frame_id,
            context_id,
            widget_id,
            bounds,
            children: Vec::new(),
            stack: Vec::new(),
            roots: Vec::new(),
        }
    }

    pub fn self_id(&self) -> WidgetId {
        self.widget_id
    }

    pub fn self_max(&self) -> Area {
        self.bounds
    }

    // Moves the context upward to the parent of the current element
    // This will panic if the parent is not in scope for this context!
    pub fn pop(&mut self) {
        let node = self.stack.pop().expect("Bad pop");
        let node = TreeNode::NonTerminal(node);

        if let Some(parent) = self.stack.last_mut() {
            parent.children.insert(0, node);
        } else {
            self.roots.insert(0, node);
        }
    }

    pub fn push_generator(&mut self, info: WidgetInfo, generator: Box<Generator>) {
        let node = NonTerminalNode {
            kind: NonTerminalKind::Generator(info, generator),
            children: Vec::new(),
        };

        self.stack.insert(0, node);
    }

    pub fn push_socket(&mut self, info: WidgetInfo, socket: Box<Socket>) {
        let node = NonTerminalNode {
            kind: NonTerminalKind::Socket(info, socket),
            children: Vec::new(),
        };

        self.stack.insert(0, node);
    }

    pub fn push_filter(&mut self, filter: Rc<Filter>) {
        let node = NonTerminalNode {
            kind: NonTerminalKind::Filter(filter),
            children: Vec::new(),
        };

        self.stack.insert(0, node);
    }

    pub fn element(&mut self, bounds: Area, element: Box<Element>) {
        let node = TreeNode::Terminal(bounds, element);

        if let Some(parent) = self.stack.last_mut() {
            parent.children.insert(0, node);
        } else {
            self.roots.insert(0, node);
        }
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
            context_id: self.context_id,
            _phantom: PhantomData,
        }
    }

    pub fn new_state_default<T: Default + Clone + Send>(&mut self, _default: T) -> State<T> {
        let id = self.next_state_id;
        self.next_state_id.0 += 1;

        State {
            id,
            frame_id: self.frame_id,
            context_id: self.context_id,
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