use std::rc::Rc;
use crate::core::*;
use crate::layout::Area;
use crate::input::{Input, InputId, InputState, InputCache};

pub trait TreeProvider {
    fn take_element(
        &mut self,
        socket: socket::Id,
    ) -> Option<(Box<dyn DynElement>, element::Id)>;
}

pub struct NullTree;

impl TreeProvider for NullTree {
    fn take_element(
        &mut self,
        _socket: socket::Id,
    ) -> Option<(Box<dyn DynElement>, element::Id)> {
        None
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
        let (element, element_id) = match self.tree_provider.take_element(id) {
            Some((element, element_id)) => (element, element_id),
            None => return false,
        };

        let backup_id = self.element_id;
        self.element_id = element_id;

        let backup_area = self.max_area;
        self.max_area = child_max_area;

        let next = element.run(self, socket);

        self.max_area = backup_area;
        self.element_id = backup_id;

        // match next {
        //     Some(next) => {
        //         self.tree_provider.place_widget(id, UIWidget::new(widget.id, next)); // TODO: This should be different
        //         true
        //     },
        //     None => false,
        // }
        false
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
