use std::rc::Rc;
use crate::core::*;
use crate::layout::Area;
use crate::input::{Input, InputId, InputState, InputCache};

pub struct TreeNode<E: Element> {
    pub element: E,
    pub id: element::Id,
}

pub trait TreeProvider {
    fn pop(
        &mut self,
        socket: socket::Id,
    ) -> Option<TreeNode<Box<dyn DynElement>>>;

    fn push_some(
        &mut self,
        socket: socket::Id,
        node: TreeNode<Box<dyn DynElement>>,
    );

    fn push_none(
        &mut self,
    );
}

impl TreeProvider for () {
    fn pop(
        &mut self,
        _socket: socket::Id,
    ) -> Option<TreeNode<Box<dyn DynElement>>> {
        None
    }

    fn push_some(
        &mut self,
        _socket: socket::Id,
        _node: TreeNode<Box<dyn DynElement>>,
    ) {
        unreachable!()
    }

    fn push_none(
        &mut self,
    ) {
        unreachable!()
    }
}

pub(crate) struct GlobalData {
    pub next_input_id: InputId,
    pub next_frame_filters: filter::FilterStack,
}

pub struct Context<'a> {
    tree_provider: &'a mut dyn TreeProvider,

    element_id: element::Id,
    max_area: Area,

    prev_input: &'a InputCache,
    global_data: &'a mut GlobalData,
}

impl<'a> Context<'a> {
    pub(crate) fn new(
        tree_provider: &'a mut dyn TreeProvider,
        element_id: element::Id,
        max_area: Area,
        prev_input: &'a InputCache,
        global_data: &'a mut GlobalData,
    ) -> Self {
        Context {
            tree_provider,
            element_id,
            max_area,
            prev_input,
            global_data,
        }
    }

    pub fn new_sub<'b: 'a>(
        parent: &'a mut Context<'b>,
        tree_provider: &'a mut dyn TreeProvider,
    ) -> Self {
        Context {
            tree_provider,

            element_id: parent.element_id,
            max_area: parent.max_area,

            prev_input: parent.prev_input,
            global_data: parent.global_data,
        }
    }

    pub fn element_id(&self) -> element::Id {
        self.element_id
    }

    pub fn max_area(&self) -> Area {
        self.max_area
    }

    pub fn render(
        &self,
        socket: &mut dyn Socket,
        render: Render,
    ) {
        socket.push(render);
    }

    pub fn render_new<R: render::RenderImpl + 'static>(
        &self,
        socket: &mut dyn Socket,
        min_area: Area,
        render: R,
    ) {
        let render = Render{ min_area, imp: Box::new(render) };
        self.render(socket, render);
    }

    pub fn socket(
        &mut self,
        id: socket::Id,
        socket: &mut dyn Socket,
        child_max_area: Area,
    ) -> bool {
        // Get the next node out of the tree
        let node = match self.tree_provider.pop(id) {
            Some(node) => node,
            None => return false,
        };

        // Backup the Id and area for the current element onto the stack
        let backup_id = self.element_id;
        self.element_id = node.id;
        let backup_area = self.max_area;
        self.max_area = child_max_area;

        // Run the element, and capture its output
        let next = node.element.run(self, socket);

        // Restore Id and area for the current element
        self.max_area = backup_area;
        self.element_id = backup_id;

        match next {
            Some(next) => {
                self.tree_provider.push_some(id, TreeNode{ element: next, id: node.id });
                true
            },
            None => {
                self.tree_provider.push_none();
                true // TODO: This might not always be the case
            }
        }
    }

    pub fn filter_next_frame(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.global_data.next_frame_filters.add_filter(filter);
    }

    pub fn filter_late_next_frame(
        &mut self,
        filter: Rc<dyn Filter>,
    ) {
        self.global_data.next_frame_filters.add_filter_late(filter);
    }

    pub fn new_input<F: InputState>(&mut self) -> Input<F> {
        let id = self.global_data.next_input_id.increment();
        Input::new(id)
    }

    pub fn read_input<F: InputState>(&self, input: Input<F>) -> F {
        if input.id.frame_id != self.global_data.next_input_id.frame_id.prev() {
            panic!("Attempt to read state from wrong frame");
        }

        if let Some(v) = self.prev_input.get(&input.id) {
            v.downcast_ref::<F>().expect("Mismatched types").clone()
        } else {
            Default::default()
        }
    }
}
