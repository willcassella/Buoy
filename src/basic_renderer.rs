use crate::prelude::*;
use crate::util::ref_move::RefMove;
use std::cell::RefCell;
use std::default::Default;

pub trait Render {
    fn render<'frm>(self, ctx: Context<'frm, '_, '_>) -> LayoutNode<'frm>;
}

pub struct BasicRenderer<T: Component + Render> {
    components: RefCell<Vec<Option<T>>>,
}

impl<T: Component + Render> Default for BasicRenderer<T> {
    fn default() -> Self {
        BasicRenderer {
            components: Default::default(),
        }
    }
}

impl<'frm, T: Component + Render + 'frm> Renderer<'frm> for BasicRenderer<T> {
    fn type_id(&self) -> TypeId {
        T::type_id()
    }

    fn alloc(&self, comp: RefMove<dyn DynComponent + 'frm>) -> ComponentIndex {
        debug_assert_eq!(comp.get_type_id(), T::type_id());
        let comp: T = unsafe { RefMove::downcast_unchecked::<T>(comp).take() };

        let mut components = self.components.borrow_mut();
        components.push(Some(comp));
        components.len() - 1
    }

    fn layout<'thrd, 'ctx>(
        &self,
        ctx: Context<'frm, 'thrd, 'ctx>,
        comp_index: ComponentIndex,
    ) -> LayoutNode<'frm> {
        let comp = self
            .components
            .borrow_mut()
            .get_mut(comp_index)
            .unwrap()
            .take()
            .unwrap();
        comp.render(ctx)
    }
}
