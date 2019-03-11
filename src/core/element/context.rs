use std::rc::Rc;

use crate::space::*;
use crate::input::*;
use crate::core::element::*;
use crate::core::tree::*;
use crate::core::filter::*;
use crate::core::common::*;

pub struct Context<'a> {
    tree_provider: &'a mut dyn TreeProvider,

    element_id: Id,
    max_area: Area,

    prev_input: &'a InputCache,
    global_data: &'a mut GlobalData,
}

impl<'a> Context<'a> {
    pub(crate) fn new(
        tree_provider: &'a mut dyn TreeProvider,
        element_id: Id,
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

    pub fn element_id(&self) -> Id {
        self.element_id
    }

    pub fn max_area(&self) -> Area {
        self.max_area
    }

    pub fn layout(
        &self,
        socket: &mut dyn Socket,
        layout: LayoutObj,
    ) {
        socket.push(layout);
    }

    pub fn layout_new<L: Layout + 'static>(
        &self,
        socket: &mut dyn Socket,
        min_area: Area,
        layout: L,
    ) {
        let layout = LayoutObj{ min_area, imp: Box::new(layout) };
        self.layout(socket, layout);
    }

    pub fn socket(
        &mut self,
        name: SocketName,
        socket: &mut dyn Socket,
        child_max_area: Area,
    ) -> bool {
        let mut tree_ctx = TreeContext{
            socket: socket,
            max_area: child_max_area,
            prev_input: self.prev_input,
            global_data: self.global_data,
        };
        self.tree_provider.socket(&mut tree_ctx, name)
    }

    pub fn filter_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.global_data.next_frame_filters.add_filter(filter);
    }

    pub fn filter_late_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
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
