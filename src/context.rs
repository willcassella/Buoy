use crate::layout::Area;
use crate::element::{
    Id,
    UIWidget,
    UIWidgetImpl,
    UIFilter,
    FilterStack,
    socket,
    UISocket,
    UIRender,
    UIRenderImpl,
};
use crate::input::{Input, InputId, InputState, InputCache};

pub trait TreeProvider {
    fn take_widget(
        &mut self,
        socket: socket::Id,
    ) -> Option<UIWidget>;
}

pub struct NullTree;

impl TreeProvider for NullTree {
    fn take_widget(
        &mut self,
        _socket: socket::Id,
    ) -> Option<UIWidget> {
        None
    }
}

pub(crate) struct GlobalData {
    pub next_input_id: InputId,
    pub next_frame_filters: FilterStack,
}

pub struct Context<'a> {
    tree_provider: &'a mut dyn TreeProvider,

    widget_id: Id,
    max_area: Area,

    prev_input: &'a InputCache,
    global_data: &'a mut GlobalData,
}

impl<'a> Context<'a> {
    pub(crate) fn new(
        tree_provider: &'a mut dyn TreeProvider,
        widget_id: Id,
        max_area: Area,
        prev_input: &'a InputCache,
        global_data: &'a mut GlobalData,
    ) -> Self {
        Context {
            tree_provider,
            widget_id,
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

            widget_id: parent.widget_id,
            max_area: parent.max_area,

            prev_input: parent.prev_input,
            global_data: parent.global_data,
        }
    }

    pub fn widget_id(&self) -> Id {
        self.widget_id
    }

    pub fn max_area(&self) -> Area {
        self.max_area
    }

    pub fn render(
        &self,
        socket: &mut dyn UISocket,
        render: UIRender,
    ) {
        socket.push(render);
    }

    pub fn render_new<R: UIRenderImpl + 'static>(
        &self,
        socket: &mut dyn UISocket,
        min_area: Area,
        render: R,
    ) {
        let render = UIRender{ min_area, imp: Box::new(render) };
        self.render(socket, render);
    }

    pub fn socket(
        &mut self,
        id: socket::Id,
        socket: &mut dyn UISocket,
        child_max_area: Area,
    ) -> bool {
        let widget = match self.tree_provider.take_widget(id) {
            Some(widget) => widget,
            None => return false,
        };

        let backup_id = self.widget_id;
        self.widget_id = widget.id;

        let backup_area = self.max_area;
        self.max_area = child_max_area;

        let next = widget.imp.run(self, socket);

        self.max_area = backup_area;
        self.widget_id = backup_id;

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
        filter: UIFilter
    ) {
        self.global_data.next_frame_filters.add_filter(filter);
    }

    pub fn filter_late_next_frame(
        &mut self,
        filter: UIFilter,
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
