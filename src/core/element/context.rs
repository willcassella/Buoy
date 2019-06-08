use std::rc::Rc;

use crate::space::*;
use crate::input::*;
use crate::core::element::*;
use crate::core::tree::*;
use crate::core::filter::*;

pub trait Context<'a>: Sized + 'a {
    fn upcast(
        self
    ) -> DynContext<'a>;

    /* Useful for elements that have a complex inner structure. */
    // TODO: It may be useful for this to take self by &mut, so that A) subcontexts may be sandboxed and B) you can have multiple subcontexts
    // In that scenario, the natural return type for this method would be a Layout
    fn subcontext<'b, T: TreeProviderRef<'b>>(
        self,
        subtree: T,
    );

    fn element_id(&self) -> Id;

    fn max_area(&self) -> Area;

    /* Opens a socket that may be filled with children. */
    fn socket<S: ?Sized + Socket>(
        &mut self,
        name: SocketName,
        socket: &mut S,
        child_max_area: Area,
    ) -> bool;

    // Constructs a layout, consuming this context.
    fn layout_new<L: Layout>(
        self,
        min_area: Area,
        layout: L,
    ) {
        self.layout(LayoutObj::new(min_area, layout))
    }

    // Constructs a layout, consuming this context.
    fn layout<L: Layout>(
        self,
        layout: LayoutObj<L>,
    );

    // Runs a new filter next frame, in the context of this element (if an element with this id is not instantiated next frame, this filter will not be run)
    fn filter_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    );

    fn filter_late_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    );

    fn new_input<F: InputState>(
        &mut self
    ) -> Input<F>;

    fn read_input<F: InputState>(
        &self,
        input: Input<F>
    ) -> F;
}

pub type DynContext<'a> = ContextImpl<'a, &'a mut dyn DynTreeProvider>;

pub struct ContextImpl<'a, T: TreeProviderRef<'a>> {
    pub(crate) tree_provider: T,

    pub(crate) element_id: Id,
    pub(crate) max_area: Area,

    pub(crate) prev_input: &'a InputCache,
    pub(crate) global_data: &'a mut GlobalData,
}

impl<'a, T: TreeProviderRef<'a>> Context<'a> for ContextImpl<'a, T> {
    fn upcast(
        self
    ) -> DynContext<'a> {
        ContextImpl {
            tree_provider: self.tree_provider.upcast_mut(),
            element_id: self.element_id,
            max_area: self.max_area,
            prev_input: self.prev_input,
            global_data: self.global_data,
        }
    }

    fn subcontext<'b, F: TreeProviderRef<'b>>(
        self,
        subtree: F,
    ) {
        unimplemented!()
    }

    fn element_id(&self) -> Id {
        self.element_id
    }

    fn max_area(&self) -> Area {
        self.max_area
    }

    fn socket<S: ?Sized + Socket>(
        &mut self,
        name: SocketName,
        socket: &mut S,
        child_max_area: Area,
    ) -> bool {
        let ctx = TreeContextImpl{
            socket,
            max_area: child_max_area,
            prev_input: self.prev_input,
            global_data: self.global_data,
        };

        self.tree_provider.invoke_socket(ctx, name)
    }

    fn layout<L: Layout>(
        self,
        layout: LayoutObj<L>,
    ) {
        unimplemented!()
    }

    fn filter_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.global_data.next_frame_filters.add_filter(filter);
    }

    fn filter_late_next_frame(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.global_data.next_frame_filters.add_filter_late(filter);
    }

    fn new_input<F: InputState>(&mut self) -> Input<F> {
        let id = self.global_data.next_input_id.increment();
        Input::new(id)
    }

    fn read_input<F: InputState>(&self, input: Input<F>) -> F {
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
