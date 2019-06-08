use std::rc::Rc;

use crate::core::common::*;
use crate::core::element::*;
use crate::core::tree::*;

#[derive(Clone, Default)]
pub struct FilterStack {
    pub(crate) filters: Vec<Rc<dyn DynFilter>>,
    pub(crate) late_filters: Vec<Rc<dyn DynFilter>>,
}

impl FilterStack {
    pub fn add_filter(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.filters.push(filter);
    }

    pub fn add_filter_late(
        &mut self,
        filter: Rc<dyn DynFilter>,
    ) {
        self.late_filters.push(filter);
    }
}

pub trait Filter: Sized {
    fn element<'a, C: TreeContext<'a>, E: Element, T: ?Sized + TreeProvider>(
        &self,
        ctx: C,
        id: Id,
        element: E,
        children: &mut T,
        _filters: &mut FilterStack,
    ) {
        //ctx.element(id, element, children);
        unimplemented!()
    }
}

pub trait DynFilter {
    fn element<'a>(
        &self,
        ctx: DynTreeContext<'a>,
        id: Id,
        element: Box<dyn DynElement>,
        children: &'a mut dyn DynTreeProvider,
        filters: &mut FilterStack,
    );
}

impl<T: Filter> DynFilter for T {
    fn element<'a>(
        &self,
        ctx: DynTreeContext<'a>,
        id: Id,
        element: Box<dyn DynElement>,
        children: &'a mut dyn DynTreeProvider,
        filters: &mut FilterStack,
    ) {
        T::element(self, ctx, id, element, children, filters);
    }
}
