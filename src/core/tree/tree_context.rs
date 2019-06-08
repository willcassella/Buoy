use std::rc::Rc;

use crate::util::fill::Fill;
use crate::space::*;
use crate::input::*;
use crate::core::element::*;
use crate::core::tree::*;
use crate::core::filter::*;
use crate::core::common::*;

pub trait TreeContext<'a>: Sized {
    fn upcast(
        self,
    ) -> DynTreeContext<'a>;

    fn remaining_capacity(
        &self
    ) -> usize;

    fn element<E: Element, S: TreeProvider>(
        &mut self,
        id: Id,
        element: E,
        sub_provider: S,
    );
}

pub type DynTreeContext<'a> = TreeContextImpl<'a, dyn Socket>;

pub struct TreeContextImpl<'a, S: ?Sized + Socket> {
    pub(crate) socket: &'a mut S,
    pub(crate) max_area: Area,

    pub(crate) prev_input: &'a InputCache,
    pub(crate) global_data: &'a mut GlobalData,
}

impl<'a, S: ?Sized + Socket> TreeContext<'a> for TreeContextImpl<'a, S> {
    fn upcast(
        self,
    ) -> DynTreeContext<'a> {
        TreeContextImpl {
            socket: unimplemented!(),
            max_area: self.max_area,

            prev_input: self.prev_input,
            global_data: self.global_data,
        }
    }

    fn remaining_capacity(&self) -> usize {
        self.socket.remaining_capacity()
    }

    fn element<E: Element, T: TreeProvider>(
        &mut self,
        id: Id,
        element: E,
        mut sub_provider: T,
    ) {
        // Create a new element context
        let ctx = ContextImpl {
            tree_provider: &mut sub_provider,
            element_id: id,
            max_area: self.max_area,
            prev_input: self.prev_input,
            global_data: self.global_data
        };

        element.run(ctx);
    }
}
