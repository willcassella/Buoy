use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use crate::layout::Area;
use crate::element::{
    Id,
    Anchor,
    UIWidget,
    UIWidgetImpl,
    UIFilter,
    FilterStack,
    UISocket,
    UISocketImpl,
    UIRender,
    UIRenderImpl
};
use crate::input::{Input, InputId, InputState, InputCache};

struct AnchorPoint {
    socket: Rc<RefCell<dyn UISocketImpl>>,
}

pub struct ContextData<'ui> {
    widget_id: Id,
    max_area: Area,
    next_input_id: InputId,
    prev_input: &'ui InputCache,
    pub next_frame_filters: FilterStack,
}

impl<'ui> ContextData<'ui> {
    pub fn new(
        widget_id: Id,
        max_area: Area,
        next_input_id: InputId,
        prev_input: &'ui InputCache,
    ) -> Self {
        ContextData {
            widget_id,
            max_area,
            next_input_id,
            prev_input,
            next_frame_filters: FilterStack::default(),
        }
    }
}

pub struct TreeNode {
    pub kind: TreeNodeKind,
    pub children: Vec<TreeNode>,
}

pub enum TreeNodeKind {
    Render(UIRender),
    Socket(UISocket),
    Filter(UIFilter),
    FilterLate(UIFilter),
    Widget(UIWidget),
    Resume(Box<dyn WidgetResume>),
    Anchor(Anchor),
    TargetAnchor(Anchor),
}

pub struct Context<'ui, 'ctx> {
    data: &'ctx mut ContextData<'ui>,

    buffer_parents: Vec<TreeNode>,
    buffer_siblings: Vec<TreeNode>,
}

impl<'ui, 'ctx> Context<'ui, 'ctx> {
    pub(super) fn new(
        data: &'ctx mut ContextData<'ui>,
    ) -> Self {
        Context {
            data,
            buffer_parents: Vec::new(),
            buffer_siblings: Vec::new(),
        }
    }

    pub fn widget_id(&self) -> Id {
        self.data.widget_id
    }

    pub fn max_area(&self) -> Area {
        self.data.max_area
    }

    pub fn begin_widget<W: UIWidgetImpl>(
        &mut self,
        widget: UIWidget<W>,
    ) {
        // Run the widget in the current context
        widget.imp.run(self);

        // If the widget called 'children', need to treat that as an output point
    }

    pub fn begin_socket(
        &mut self,
        socket: UISocket,
    ) {
        let node = TreeNode {
            kind: TreeNodeKind::Socket(socket),
            children: Vec::new(),
        };

        self.begin(node);
    }

    pub fn begin_awaitable_socket<S: UISocketImpl + 'static>(
        &mut self,
        max_area: Area,
        socket: S,
    ) -> SocketRef<S> {
        let socket = UISocket::new(max_area, Box::new(socket));
        self.begin_socket(socket);

        SocketRef::new()
    }

    pub fn take_socket<S: UISocketImpl>(
        &mut self,
        socket: SocketRef<S>
    ) -> S {
        unimplemented!()
    }

    pub fn begin_filter(
        &mut self,
        filter: UIFilter,
    ) {
        let node = TreeNode {
            kind: TreeNodeKind::Filter(filter),
            children: Vec::new(),
        };

        self.begin(node);
    }

    pub fn begin_filter_late(
        &mut self,
        filter: UIFilter,
    ) {
        let node = TreeNode {
            kind: TreeNodeKind::FilterLate(filter),
            children: Vec::new(),
        };

        self.begin(node);
    }

    pub fn render_new<R: UIRenderImpl + 'static>(
        &mut self,
        min_area: Area,
        imp: R,
    ) {
        self.render(UIRender{
            min_area,
            imp: Box::new(imp),
        });
    }

    pub fn render(
        &mut self,
        render: UIRender,
    ) {
        let node = TreeNode {
            kind: TreeNodeKind::Render(render),
            children: Vec::new(),
        };

        // Not calling 'begin' here, because renders can't have children
        self.buffer_siblings.push(node);
    }

    pub fn anchor_default(
        &mut self,
    ) {
        self.anchor(Anchor::default())
    }

    pub fn anchor(
        &mut self,
        name: Anchor,
    ) {
        unimplemented!()
    }

    pub fn anchor_global(
        &mut self,
        name: Anchor,
    ) {
        unimplemented!()
    }

    // Forwards anchors in the current scope to the outer scope
    pub fn forward_anchors(
        &mut self,
    ) {
        unimplemented!()
    }

    pub fn begin_target_anchor(
        &mut self,
        name: Anchor,
    ) {
        unimplemented!()
    }

    pub fn begin_target_anchor_global(
        &mut self,
        name: Anchor,
    ) {
        unimplemented!()
    }

    fn begin(&mut self, mut node: TreeNode) {
        node.children = std::mem::replace(&mut self.buffer_siblings, Vec::new());
        self.buffer_parents.push(node);
    }

    // Moves the context upward to the parent of the current widget
    // This will panic if the parent is not in scope for this context!
    pub fn end(&mut self) {
        let mut node = self.buffer_parents.pop().expect("Called 'end' after last widget in WIP stack");

        // While the node was in the WIP stack, it's children field actually held its siblings
        std::mem::swap(&mut self.buffer_siblings, &mut node.children);
        self.buffer_siblings.push(node);
    }

    pub fn await_sockets<W: WidgetResume>(
        &mut self,
        resume: W,
    ) {
        // Need to create a new context
        unimplemented!()
    }

    pub fn filter_next_frame(
        &mut self,
        filter: UIFilter
    ) {
        self.data.next_frame_filters.add_filter(filter);
    }

    pub fn filter_late_next_frame(
        &mut self,
        filter: UIFilter,
    ) {
        self.data.next_frame_filters.add_filter_late(filter);
    }

    pub fn new_input<F: InputState>(&mut self) -> Input<F> {
        let id = self.data.next_input_id.increment();
        Input::new(id)
    }

    pub fn read_input<F: InputState>(&self, input: Input<F>) -> F {
        if input.id.frame_id != self.data.next_input_id.frame_id.prev() {
            panic!("Attempt to read state from wrong frame");
        }

        if let Some(v) = self.data.prev_input.get(&input.id) {
            v.downcast_ref::<F>().expect("Mismatched types").clone()
        } else {
            Default::default()
        }
    }
}

pub struct SocketRef<T> {
    _phantom: PhantomData<T>,
}

impl<T> SocketRef<T> {
    fn new() -> Self {
        SocketRef {
            _phantom: PhantomData,
        }
    }
}

// This is intentionally NOT a subtrait of 'UIWidgetImpl', even though the method is identical.
// The idea is that you shouldn't pass opaque types (like closures) as widgets,
// and you shouldn't pass normal widgets as continuations.
// The reason is that continuations are not passed through the filter stack.
pub trait WidgetResume {
    fn resume<'ui, 'ctx>(
        self: Box<Self>,
        ctx: &mut Context<'ui, 'ctx>,
    );
}

impl<T> WidgetResume for T
where T: FnOnce(&mut Context)
{
    fn resume<'ui, 'ctx>(
        self: Box<Self>,
        ctx: &mut Context<'ui, 'ctx>,
    ) {
        self(ctx);
    }
}
