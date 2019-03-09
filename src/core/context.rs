use std::rc::Rc;
use crate::core::*;
use crate::layout::Area;
use crate::input::{Input, InputId, InputState, InputCache};

pub struct TreeNode<E: Element> {
    pub element: E,
    pub id: element::Id,
}

pub struct TreeListener<'a> {
    socket: &'a mut dyn Socket,
    max_area: Area,

    prev_input: &'a InputCache,
    global_data: &'a mut GlobalData,
}

impl<'a> TreeListener<'a> {
    pub fn remaining_capacity(&self) -> usize {
        self.socket.remaining_capacity()
    }

    pub fn element<E: Element, T: TreeProvider>(
        &mut self,
        id: element::Id,
        element: E,
        sub_provider: &mut T,
    ) -> Option<E::Resume> {
        // Create a new context
        let mut ctx = Context::new(
            sub_provider,
            id,
            self.max_area,
            self.prev_input,
            self.global_data
        );

        element.run(&mut ctx, self.socket)
    }
}

pub trait TreeProvider {
    fn socket(
        &mut self,
        id: socket::Id,
        listener: &mut TreeListener,
    ) -> bool;
}

impl TreeProvider for () {
    fn socket(
        &mut self,
        _id: socket::Id,
        _listener: &mut TreeListener,
    ) -> bool {
        // Do nothing
        false
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
        let mut listener = TreeListener{
            socket: socket,
            max_area: child_max_area,
            prev_input: self.prev_input,
            global_data: self.global_data,
        };
        self.tree_provider.socket(id, &mut listener)
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
