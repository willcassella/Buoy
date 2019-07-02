use std::rc::Rc;

use crate::space::*;
use crate::input::*;
use crate::core::element::*;
use crate::core::tree::*;
use crate::core::filter::*;

pub struct Context<'window, 'ctx> {
    pub(crate) tree_provider: &'ctx mut dyn TreeProvider,
    pub(crate) out_layout: &'ctx mut Option<LayoutObj>,

    pub(crate) element_id: Id,
    pub(crate) max_area: Area,

    pub(crate) prev_input: &'window InputCache,
    pub(crate) global_data: &'window mut GlobalData,
}

impl<'window, 'ctx> Context<'window, 'ctx> {
    /* Useful for elements that have a complex inner structure. */
    pub fn subcontext(
        &mut self,
        max_area: Area,
        element_id: Id,
        element: &dyn Element,
        subtree: &mut dyn TreeProvider,
    ) -> Option<LayoutObj> {
        let mut out_layout = None;
        let sub_ctx = Context {
            tree_provider: subtree,
            element_id,
            max_area,
            out_layout: &mut out_layout,
            prev_input: self.prev_input,
            global_data: self.global_data,
        };

        element.run(sub_ctx);
        out_layout
    }

    // Returns the id of the currently running element
    pub fn element_id(&self) -> Id {
        self.element_id
    }

    // Returns the max allocatable area for the currently running element
    pub fn max_area(&self) -> Area {
        self.max_area
    }

    /* Opens a socket that may be filled with children. */
    pub fn socket(
        &mut self,
        name: SocketName,
        socket: &mut dyn Socket,
        child_max_area: Area,
    ) -> bool {
        let ctx = TreeContext{
            socket,
            max_area: child_max_area,
            prev_input: self.prev_input,
            global_data: self.global_data,
        };

        self.tree_provider.socket(ctx, name)
    }

    // Constructs a layout, consuming this context.
    pub fn layout_new<L: Layout>(
        self,
        min_area: Area,
        layout: L,
    ) {
        self.layout(LayoutObj::new(min_area, layout))
    }

    // Constructs a layout, consuming this context.
    pub fn layout<L: Layout>(
        self,
        layout: LayoutObj<L>,
    ) {
        // TODO: If this didn't always need to upcast that would be nice
        *self.out_layout = Some(layout.upcast());
    }

    // Runs a new filter next frame, in the context of this element (if an element with this id is not instantiated next frame, this filter will not be run)
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
